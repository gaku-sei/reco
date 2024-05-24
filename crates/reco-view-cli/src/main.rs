#![deny(clippy::all, clippy::pedantic, clippy::unwrap_used)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about,  long_about = None)]
struct Args {
    /// Path to the archive to view
    path: PathBuf,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    reco_view::view(&args.path)?;

    Ok(())
}
