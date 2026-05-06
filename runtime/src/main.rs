// runtime/src/main.rs
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use anyhow::Result;
use clap::Parser;
use setupweaver_runtime::{engine::InstallerEngine, ui};

#[derive(Debug, Parser)]
#[command(author, version, about = "SetupWeaver runtime stub")]
struct Cli {
    #[arg(long)]
    print_manifest: bool,
    #[arg(long)]
    silent: bool,
    #[arg(long)]
    install_dir: Option<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let engine = InstallerEngine::from_current_exe()?;

    if cli.print_manifest {
        println!("{:#?}", engine.manifest());
        return Ok(());
    }

    if cli.silent {
        engine.install(cli.install_dir.as_deref())?;
        engine.finish(cli.install_dir.as_deref())?;
        return Ok(());
    }

    slint::BackendSelector::new().backend_name("winit".into()).select()?;
    ui::run_installer(&engine, cli.install_dir.as_deref())
}
