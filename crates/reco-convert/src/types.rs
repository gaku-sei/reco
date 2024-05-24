use std::{fs::OpenOptions, io::Read, path::Path};

use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    // TODO: Add Mobi, Azw3, etc...
    Pdf,
}

impl Format {
    pub fn try_from_path(path: &Path) -> Result<Self, Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .append(false)
            .open(path)
            .map_err(Error::FileOpen)?;

        let mut buf = [0; 32];
        file.read_exact(&mut buf).map_err(Error::FileRead)?;

        if infer::is(&buf, "pdf") {
            return Ok(Self::Pdf);
        }

        Err(Error::UnknownFormat)
    }
}
