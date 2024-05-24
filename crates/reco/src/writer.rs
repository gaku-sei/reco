use std::{
    fs::{File, create_dir_all},
    io::{self, Cursor, Seek, Write},
    path::Path,
};

use image::{GenericImageView, PixelWithColorType, codecs::jpeg::JpegEncoder};
use infer::is_image;
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

use crate::{
    ArchiveFinishError,
    errors::{CreateArchiveError, InsertionError},
};

pub struct Writer<W: Write + Seek> {
    archive: ZipWriter<W>,
    current_index: usize,
}

impl<W> Writer<W>
where
    W: Write + Seek,
{
    pub fn new(writer: W) -> Self {
        let archive = ZipWriter::new(writer);

        Self {
            archive,
            current_index: 0,
        }
    }
}

impl Writer<File> {
    pub fn create_from_path(path: &Path) -> Result<Self, CreateArchiveError> {
        create_dir_all(path.parent().unwrap_or_else(|| Path::new(".")))
            .map_err(CreateArchiveError::DirCreation)?;
        let file = File::create(path).map_err(CreateArchiveError::ArchiveCreation)?;

        Ok(Self::new(file))
    }
}

impl<W> Writer<W>
where
    W: Write + Seek,
{
    pub fn insert_from_path(&mut self, path: &Path) -> Result<(), InsertionError> {
        let file = File::open(path).map_err(InsertionError::FileOpen)?;
        let ext = path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        self.insert_reader(ext, file)
    }

    pub fn insert_reader(
        &mut self,
        ext: &str,
        mut reader: impl io::Read,
    ) -> Result<(), InsertionError> {
        let mut buf = Vec::new();
        reader
            .read_to_end(&mut buf)
            .map_err(InsertionError::ReaderConsumption)?;

        self.insert_buf(ext, &buf)
    }

    pub fn insert_image_as_jpeg<I>(&mut self, img: &I, quality: u8) -> Result<(), InsertionError>
    where
        I: GenericImageView,
        I::Pixel: PixelWithColorType,
    {
        let mut buf = Cursor::new(Vec::new());
        let mut encoder = JpegEncoder::new_with_quality(&mut buf, quality);
        encoder
            .encode_image(img)
            .map_err(InsertionError::ImageEncode)?;

        self.insert_buf(".jpeg", &buf.into_inner())
    }

    pub fn insert_buf(&mut self, ext: &str, buf: &[u8]) -> Result<(), InsertionError> {
        if self.is_full() {
            return Err(InsertionError::CbzFull(usize::MAX));
        }

        if !is_image(buf) {
            return Err(InsertionError::InvalidFormat);
        }

        let name = format!("{:0>20}{ext}", self.current_index);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
        self.archive
            .start_file(name, options)
            .map_err(InsertionError::FileCreation)?;

        // TODO: Revert start file on error?
        self.archive
            .write_all(buf)
            .map_err(InsertionError::FileWrite)?;

        self.current_index += 1;

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.current_index == 0
    }

    pub fn is_full(&self) -> bool {
        self.current_index == usize::MAX
    }

    pub fn finish(self) -> Result<W, ArchiveFinishError> {
        self.archive.finish().map_err(Into::into)
    }
}
