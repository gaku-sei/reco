#![deny(clippy::all, clippy::pedantic, clippy::unwrap_used)]

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

static DEFAULT_FILENAME: &str = "out.cbz";

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

        /// The file name of the archive file (defaults to the file stem + cbz or out.cbz if none is found)
        filename: Option<String>,
    },

    Pack {
        /// A glob pattern that matches the files to pack
        pattern: String,

        /// The output directory for the packed archive
        #[clap(default_value = ".")]
        output: PathBuf,

        /// The file name of the archive file
        #[clap(default_value = DEFAULT_FILENAME)]
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
        #[clap(default_value = DEFAULT_FILENAME)]
        filename: String,
    },

    View {
        /// Path to the archive to view
        path: PathBuf,
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
            let filename = filename
                .or_else(|| {
                    path.file_stem()
                        .and_then(|stem| stem.to_str())
                        .map(|stem| format!("{stem}.cbz"))
                })
                .unwrap_or_else(|| DEFAULT_FILENAME.to_string());
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
    }

    Ok(())
}
