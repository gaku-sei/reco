use std::io::{self, Cursor};
use std::path::Path;
use std::vec::IntoIter;

use image::{DynamicImage, ImageReader};
use pdf::enc::StreamFilter;
use pdf::file::{File as PdfFile, FileOptions as PdfFileOptions, NoLog, ObjectCache, StreamCache};
use pdf::object::{Ref, Resolve, XObject};

use crate::Error;

pub struct Pdf {
    inner: PdfFile<Vec<u8>, ObjectCache, StreamCache, NoLog>,
}

#[derive(Debug, thiserror::Error)]
pub enum PdfError {
    #[error(transparent)]
    Pdf(#[from] pdf::PdfError),

    #[error(transparent)]
    Image(#[from] image::ImageError),

    #[error(transparent)]
    IO(#[from] io::Error),
}

impl Pdf {
    pub fn try_from_path(path: &Path) -> Result<Self, Error> {
        let pdf = PdfFileOptions::cached()
            .open(path)
            .map_err(PdfError::from)?;

        Ok(Self { inner: pdf })
    }
}

impl IntoIterator for Pdf {
    type Item = Result<DynamicImage, Error>;

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut imgs = Vec::new();
        let resolver = self.inner.resolver();

        for page in self.inner.pages() {
            let page = match page {
                Ok(page) => page,
                Err(err) => {
                    imgs.push(Err(PdfError::from(err).into()));
                    continue;
                }
            };

            let resources = match page.resources() {
                Ok(resources) => resources,
                Err(err) => {
                    imgs.push(Err(PdfError::from(err).into()));
                    continue;
                }
            };

            for resource in resources.xobjects.values() {
                if let Err(err) = decode_imgs(&resolver, resource, &mut imgs) {
                    imgs.push(Err(err.into()));
                    continue;
                }
            }
        }

        imgs.into_iter()
    }
}

fn decode_imgs(
    resolver: &impl Resolve,
    resource: &Ref<XObject>,
    imgs: &mut Vec<Result<DynamicImage, Error>>,
) -> Result<(), PdfError> {
    let resource = resolver.get(*resource)?;

    if let XObject::Image(image) = &*resource {
        let (image, filter) = image.raw_image_data(resolver)?;

        if let Some(StreamFilter::DCTDecode(_)) = filter {
            let img_reader = ImageReader::new(Cursor::new(image));
            let img = img_reader.with_guessed_format()?.decode()?;

            imgs.push(Ok(img));

            return Ok(());
        }
    }

    Ok(())
}
