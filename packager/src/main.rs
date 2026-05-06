// packager/src/main.rs
mod builder;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about = "Build single-file SetupWeaver installers")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Build {
        #[arg(short, long, default_value = "install.toml")]
        config: PathBuf,
        #[arg(short, long)]
        stub: PathBuf,
        #[arg(short, long, default_value = "setup.exe")]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Build { config, stub, output } => {
            builder::build_installer(&config, &stub, &output)?;
        }
    }

    Ok(())
}
