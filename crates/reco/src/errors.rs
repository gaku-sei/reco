use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateArchiveError {
    #[error("dir creation error: {0}")]
    DirCreation(io::Error),

    #[error("archive creation error: {0}")]
    ArchiveCreation(io::Error),
}

#[derive(Debug, Error)]
pub enum InsertionError {
    #[error("file open error: {0}")]
    FileOpen(io::Error),

    #[error("cbz archive is full, maximum of {0} reached")]
    CbzFull(usize),

    #[error("cbz archive file creation error {0}")]
    FileCreation(zip::result::ZipError),

    #[error("cbz archive file write error {0}")]
    FileWrite(io::Error),

    #[error("cbz reader comsumption error {0}")]
    ReaderConsumption(io::Error),

    #[error("image encode error {0}")]
    ImageEncode(image::error::ImageError),

    #[error("file is not an image")]
    InvalidFormat,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ArchiveFinishError(#[from] zip::result::ZipError);

#[derive(Debug, Error)]
pub enum SpineCreationError {
    #[error("zip access by name failure: {0}")]
    ZipByName(zip::result::ZipError),

    #[error("zip file read error: {0}")]
    ZipFileRead(io::Error),
}

#[derive(Debug, Error)]
pub enum ReaderCreationError {
    #[error("archive creation error: {0}")]
    ArchiveCreation(zip::result::ZipError),

    #[error("spine creation error: {0}")]
    SpineCreation(#[from] SpineCreationError),
}

#[derive(Debug, Error)]
pub enum ReaderOpenError {
    #[error("cbz file open error: {0}")]
    FileOpen(io::Error),

    #[error("reader creation error: {0}")]
    ReaderCreation(#[from] ReaderCreationError),
}

#[derive(Debug, Error)]
pub enum LoadImageError {
    #[error("zip access by index failure: {0} not found")]
    ZipByIndex(usize),

    #[error("zip access by name failure: {0}")]
    ZipByName(zip::result::ZipError),

    #[error("zip file read error: {0}")]
    ZipFileRead(io::Error),

    #[error("image guessed format error: {0}")]
    ImageGuessedFormat(io::Error),

    #[error("image decode error: {0}")]
    ImageDecode(image::error::ImageError),
}
