// runtime/src/payload.rs
use std::fs::File;
use std::io::{Cursor, Read};
use std::ops::Range;
use std::path::{Path, PathBuf};

use memmap2::Mmap;
use setupweaver_common::{PackagedInstaller, PACKAGED_MANIFEST_PATH};
use tar::Archive;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PayloadError {
    #[error("failed to open installer binary {path}: {source}")]
    OpenExe {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to memory-map installer binary {path}: {source}")]
    MapExe {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("installer binary is too small to contain a payload trailer")]
    MissingTrailer,
    #[error("payload offset {offset} is outside installer size {file_len}")]
    InvalidOffset { offset: u64, file_len: usize },
    #[error("failed to read payload archive: {0}")]
    Archive(#[from] std::io::Error),
    #[error("failed to parse packaged manifest: {0}")]
    ManifestParse(#[from] toml::de::Error),
    #[error("packaged manifest entry {path} not found")]
    ManifestMissing { path: &'static str },
}

pub struct EmbeddedPayload {
    exe_path: PathBuf,
    mmap: Mmap,
    archive_range: Range<usize>,
}

impl EmbeddedPayload {
    pub fn from_current_exe() -> Result<Self, PayloadError> {
        let exe_path = std::env::current_exe().map_err(|source| PayloadError::OpenExe {
            path: PathBuf::from("<current_exe>"),
            source,
        })?;
        Self::from_exe(&exe_path)
    }

    pub fn from_exe(path: &Path) -> Result<Self, PayloadError> {
        let file = File::open(path).map_err(|source| PayloadError::OpenExe {
            path: path.to_path_buf(),
            source,
        })?;

        // SAFETY: The file descriptor stays alive for the duration of mmap creation,
        // and the returned Mmap owns the mapping independently of File afterwards.
        let mmap = unsafe { Mmap::map(&file) }.map_err(|source| PayloadError::MapExe {
            path: path.to_path_buf(),
            source,
        })?;

        if mmap.len() < 8 {
            return Err(PayloadError::MissingTrailer);
        }

        let offset_index = mmap.len() - 8;
        let offset = u64::from_le_bytes(mmap[offset_index..].try_into().expect("slice length is fixed"));
        if offset as usize > offset_index {
            return Err(PayloadError::InvalidOffset {
                offset,
                file_len: mmap.len(),
            });
        }

        Ok(Self {
            exe_path: path.to_path_buf(),
            mmap,
            archive_range: offset as usize..offset_index,
        })
    }

    pub fn exe_path(&self) -> &Path {
        &self.exe_path
    }

    pub fn archive_bytes(&self) -> &[u8] {
        &self.mmap[self.archive_range.clone()]
    }

    pub fn read_manifest(&self) -> Result<PackagedInstaller, PayloadError> {
        let mut archive = Archive::new(zstd::stream::read::Decoder::new(Cursor::new(self.archive_bytes()))?);
        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?;
            if path.to_string_lossy() == PACKAGED_MANIFEST_PATH {
                let mut manifest = String::new();
                entry.read_to_string(&mut manifest)?;
                return Ok(toml::from_str(&manifest)?);
            }
        }

        Err(PayloadError::ManifestMissing {
            path: PACKAGED_MANIFEST_PATH,
        })
    }
}
