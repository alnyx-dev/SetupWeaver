// common/src/packaged.rs
use serde::{Deserialize, Serialize};

use crate::InstallConfig;

pub const PACKAGED_MANIFEST_PATH: &str = ".setupweaver/manifest.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackagedInstaller {
    pub config: InstallConfig,
    #[serde(default)]
    pub license_text: Option<String>,
    pub payload: Vec<PackagedFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackagedFile {
    pub archive_path: String,
    pub destination: String,
    pub size: u64,
}
