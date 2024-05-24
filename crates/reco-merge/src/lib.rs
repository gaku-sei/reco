use std::path::Path;

use glob::{GlobError, PatternError};
use lexical_sort::{StringSort, natural_lexical_cmp};
use tracing::error;

use reco::{
    ArchiveFinishError, CreateArchiveError, InsertionError as CbzInsertionError,
    Reader as CbzReader, ReaderOpenError as CbzReaderOpenError, Writer as CbzWriter,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("archive creation error: {0}")]
    ArchiveCreation(#[from] CreateArchiveError),

    #[error("archive creation error: {0}")]
    ArchiveFinish(#[from] ArchiveFinishError),

    #[error("glob pattern error: {0}")]
    GlobPattern(#[from] PatternError),

    #[error("glob error: {0}")]
    Glob(#[from] GlobError),

    #[error("cbz open error: {0}")]
    CbzOpen(#[from] CbzReaderOpenError),

    #[error("cbz insertion error: {0}")]
    CbzInsertion(#[from] CbzInsertionError),

    #[error("zip file error: {0}")]
    ZipFile(#[from] zip::result::ZipError),
}

pub fn merge(pattern: &str, path: &Path) -> Result<(), Error> {
    let paths = glob::glob(pattern)?;
    let mut cbz = CbzWriter::create_from_path(path)?;

    for path in paths {
        let path = path?;
        let mut cbz_reader = CbzReader::try_open(&path)?;
        let mut file_names = cbz_reader
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        file_names.string_sort_unstable(natural_lexical_cmp);

        for file_name in file_names {
            let file = cbz_reader.by_name(&file_name)?;
            let path = Path::new(file.name());
            let ext = path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string();

            if let Err(err) = cbz.insert_reader(&ext, file) {
                error!("error inserting file: {err}");
            }
        }
    }

    cbz.finish()?;

    Ok(())
}
