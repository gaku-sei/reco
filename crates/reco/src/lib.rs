#![deny(clippy::all, clippy::pedantic, clippy::unwrap_used)]
#![expect(clippy::missing_errors_doc)]

pub use errors::{
    ArchiveFinishError, CreateArchiveError, InsertionError, LoadImageError, ReaderCreationError,
    ReaderOpenError,
};
pub use reader::Reader;
pub use writer::Writer;

pub mod errors;
pub mod reader;
pub mod writer;
