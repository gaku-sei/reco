use std::{
    io::{self, Seek, Write},
    path::Path,
};

use glob::{GlobError, PatternError};
use image::ImageReader;
use lexical_sort::{PathSort, natural_lexical_cmp};
use tracing::error;

use reco::{
    ArchiveFinishError, CreateArchiveError, InsertionError as CbzInsertionError,
    Writer as CbzWriter,
};

#[derive(Debug, Clone, Copy)]
pub struct Options {
    autosplit: bool,
}

impl Options {
    fn has_img_modifier(self) -> bool {
        self.autosplit
    }
}

impl Options {
    #[must_use]
    pub fn new(autosplit: bool) -> Self {
        Self { autosplit }
    }
}

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

    #[error("cbz insertion error: {0}")]
    CbzInsertion(#[from] CbzInsertionError),

    #[error("image open error: {0}")]
    ImageOpen(io::Error),

    #[error("image decode error: {0}")]
    ImageDecode(image::error::ImageError),
}

pub fn pack(pattern: &str, path: &Path, opts: Options) -> Result<(), Error> {
    let mut paths = glob::glob(pattern)?.collect::<Result<Vec<_>, _>>()?;

    paths.path_sort_unstable(natural_lexical_cmp);

    let mut cbz = CbzWriter::create_from_path(path)?;

    for path in paths {
        if let Err(err) = insert_from_path(&mut cbz, &path, opts) {
            error!("image insertion error: {err}");
        }
    }

    cbz.finish()?;

    Ok(())
}

#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn insert_from_path(
    cbz: &mut CbzWriter<impl Write + Seek>,
    path: &Path,
    opts: Options,
) -> Result<(), Error> {
    if opts.has_img_modifier() {
        let mut img = ImageReader::open(path)
            .map_err(Error::ImageOpen)?
            .decode()
            .map_err(Error::ImageDecode)?;
        let height = img.height();
        let width = img.width();

        if opts.autosplit && height < width {
            let crop_margin = f64::from(width) / 100.0;
            let left_img =
                img.clone()
                    .crop(0, 0, (f64::from(width) / 2.0 + crop_margin) as u32, height);
            let right_img = img.crop(
                (f64::from(width) / 2.0 - crop_margin) as u32,
                0,
                width,
                height,
            );

            cbz.insert_image_as_jpeg(&left_img, 75)?;
            cbz.insert_image_as_jpeg(&right_img, 75)?;

            return Ok(());
        }
    }

    cbz.insert_from_path(path)?;

    Ok(())
}
