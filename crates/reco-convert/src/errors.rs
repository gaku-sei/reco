use std::io;

use reco::{ArchiveFinishError, CreateArchiveError, InsertionError};

use crate::converters::PdfError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("creating archive: {0}")]
    CreateArchive(#[from] CreateArchiveError),

    #[error("inserting image into archive: {0}")]
    Insertion(#[from] InsertionError),

    #[error("finishing archive: {0}")]
    FinishArchive(#[from] ArchiveFinishError),

    #[error(transparent)]
    Pdf(#[from] PdfError),

    #[error("opening file: {0}")]
    FileOpen(io::Error),

    #[error("reading file: {0}")]
    FileRead(io::Error),

    #[error("unknown file format")]
    UnknownFormat,
}
