#![deny(clippy::all, clippy::pedantic, clippy::unwrap_used)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Convert {
        /// Path to the source file
        path: PathBuf,

        /// Path to the cbz file
        #[clap(default_value = ".")]
        output: PathBuf,

        /// The file name of the archive file
        #[clap(default_value = "out.cbz")]
        filename: String,
    },

    Pack {
        /// A glob pattern that matches the files to pack
        pattern: String,

        /// The output directory for the packed archive
        #[clap(default_value = ".")]
        output: PathBuf,

        /// The file name of the archive file
        #[clap(default_value = "out.cbz")]
        filename: String,

        /// Automatically split landscape images into 2 pages
        #[clap(long, action)]
        autosplit: bool,
    },

    Merge {
        /// A glob pattern that matches the archive files to merge
        pattern: String,

        /// The output directory for the merged archive
        #[clap(default_value = ".")]
        output: PathBuf,

        /// The file name of the archive file
        #[clap(default_value = "out.cbz")]
        filename: String,
    },

    View {
        /// Path to the archive to view
        path: PathBuf,
    },

    #[cfg(feature = "enhance")]
    Enhance {
        /// Path to the cbz file to enhance
        input: PathBuf,

        /// Path to the enhanced cbz
        #[clap(default_value = ".")]
        output: PathBuf,

        /// Threads to use
        #[clap(long, default_value_t = 4)]
        max_threads: usize,

        /// Path to the onnx runtime
        #[clap(long)]
        onnx_runtime_path: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    match args.command {
        Command::Convert {
            path,
            output,
            filename,
        } => {
            let output_path = output.join(filename);
            reco_convert::convert(&path, &output_path)?;
        }
        Command::Pack {
            pattern,
            output,
            filename,
            autosplit,
        } => {
            let path = output.join(filename);
            let opts = reco_pack::Options::new(autosplit);
            reco_pack::pack(&pattern, &path, opts)?;
        }
        Command::Merge {
            pattern,
            output,
            filename,
        } => {
            let path = output.join(filename);
            reco_merge::merge(&pattern, &path)?;
        }
        Command::View { path } => reco_view::view(&path)?,
        #[cfg(feature = "enhance")]
        Command::Enhance {
            input,
            output,
            max_threads,
            onnx_runtime_path,
        } => {
            if let Some(onnx_runtime_path) = onnx_runtime_path {
                reco_enhance::init(&onnx_runtime_path)?;
            }
            reco_enhance::enhance(&input, &output, max_threads)?;
        }
    }

    Ok(())
}
