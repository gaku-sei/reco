use std::{
    fs::File,
    io::{self, Cursor, Read, Seek},
    path::Path,
};

use image::DynamicImage;
use image::ImageReader;
use infer::is_image;
use tracing::debug;
use zip::{ZipArchive, read::ZipFile, result::ZipResult};

use super::errors::{LoadImageError, ReaderCreationError, ReaderOpenError, SpineCreationError};

#[derive(Debug)]
pub struct Spine {
    spine: Vec<String>,
    current_index: usize,
}

impl Spine {
    pub fn try_new<R: Read + Seek>(
        archive: &mut ZipArchive<R>,
    ) -> Result<Self, SpineCreationError> {
        let file_names = archive
            .file_names()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        let mut spine = Vec::with_capacity(file_names.len());

        for file_name in file_names {
            let mut file = archive
                .by_name(&file_name)
                .map_err(SpineCreationError::ZipByName)?;

            if !file.is_file() {
                continue;
            }

            let mut buf = [0; 32];
            file.read_exact(&mut buf)
                .map_err(SpineCreationError::ZipFileRead)?;

            if !is_image(&buf) {
                debug!("{file_name} is not an image, skipping");
                continue;
            }

            spine.push(file_name);
        }

        spine.sort_unstable();

        Ok(Self {
            spine,
            current_index: 0,
        })
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.spine.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.spine.is_empty()
    }

    #[must_use]
    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn go_to_prev_index(&mut self) {
        if self.current_index == 0 {
            return;
        }

        self.set_current_index(self.current_index - 1);
    }

    pub fn go_to_next_index(&mut self) {
        self.set_current_index(self.current_index + 1);
    }

    pub fn set_current_index(&mut self, index: usize) {
        self.current_index = index.clamp(0, self.len() - 1);
    }

    #[must_use]
    pub fn get_current_file_name(&self) -> Option<&str> {
        self.spine.get(self.current_index).map(String::as_str)
    }

    pub fn current_index_mut(&mut self) -> &mut usize {
        &mut self.current_index
    }
}

pub struct Reader<R> {
    spine: Spine,
    archive: ZipArchive<R>,
}

impl<R> Reader<R> {
    pub fn spine(&self) -> &Spine {
        &self.spine
    }

    pub fn spine_mut(&mut self) -> &mut Spine {
        &mut self.spine
    }
}

impl<R> Reader<R>
where
    R: Read + Seek,
{
    pub fn try_new(reader: R) -> Result<Self, ReaderCreationError> {
        let mut archive = ZipArchive::new(reader).map_err(ReaderCreationError::ArchiveCreation)?;
        let spine = Spine::try_new(&mut archive)?;

        Ok(Self { spine, archive })
    }

    pub fn by_name(&mut self, file_name: &str) -> ZipResult<ZipFile<'_, R>> {
        self.archive.by_name(file_name)
    }

    #[expect(clippy::cast_possible_truncation)]
    pub fn load_current_img(&mut self) -> Result<DynamicImage, LoadImageError> {
        let Some(file_name) = self.spine.get_current_file_name() else {
            return Err(LoadImageError::ZipByIndex(self.spine.current_index));
        };
        let mut file = self
            .archive
            .by_name(file_name)
            .map_err(LoadImageError::ZipByName)?;

        let mut buf = Cursor::new(Vec::with_capacity(file.size() as usize));
        io::copy(&mut file, &mut buf).map_err(LoadImageError::ZipFileRead)?;
        let _ = buf
            .seek(io::SeekFrom::Start(0))
            .map_err(LoadImageError::ZipFileRead)?;

        let reader = ImageReader::new(buf)
            .with_guessed_format()
            .map_err(LoadImageError::ImageGuessedFormat)?;
        let img = reader.decode().map_err(LoadImageError::ImageDecode)?;

        Ok(img)
    }

    pub fn go_to_prev_index(&mut self) {
        self.spine.go_to_prev_index();
    }

    pub fn go_to_next_index(&mut self) {
        self.spine.go_to_next_index();
    }

    pub fn go_to_index(&mut self, index: usize) {
        self.spine.set_current_index(index);
    }
}

impl Reader<File> {
    pub fn try_open(path: &Path) -> Result<Self, ReaderOpenError> {
        let file = File::open(path).map_err(ReaderOpenError::FileOpen)?;
        Ok(Self::try_new(file)?)
    }
}

impl<R> Reader<R> {
    #[must_use]
    pub fn iter(&self) -> IndicesIter<'_> {
        IndicesIter::new(&self.spine.spine)
    }
}

impl<'a, R> IntoIterator for &'a Reader<R> {
    type IntoIter = IndicesIter<'a>;
    type Item = &'a str;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct IndicesIter<'a> {
    spine: &'a [String],
    current_index: usize,
}

impl<'a> IndicesIter<'a> {
    fn new(spine: &'a [String]) -> Self {
        Self {
            spine,
            current_index: 0,
        }
    }
}

impl<'a> Iterator for IndicesIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.spine.len() {
            return None;
        }
        let item = &self.spine[self.current_index];
        self.current_index += 1;
        Some(item)
    }
}
