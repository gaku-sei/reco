use reco::{LoadImageError, ReaderOpenError as CbzReaderOpenError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("eframe error: {0}")]
    Eframe(String),

    #[error("cbz reader error: {0}")]
    CbzReader(#[from] CbzReaderOpenError),

    #[error("cbz image load error: {0}")]
    LoadImage(#[from] LoadImageError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
