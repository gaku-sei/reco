use std::path::Path;

use converters::Pdf;
pub use errors::Error;
use image::DynamicImage;
use reco::Writer as CbzWriter;
use tracing::error;
pub use types::Format;

mod converters;
mod errors;
mod types;

pub fn convert(input_path: &Path, output_path: &Path) -> Result<(), Error> {
    let format = Format::try_from_path(input_path)?;

    let imgs: Box<dyn Iterator<Item = Result<DynamicImage, Error>>> = match format {
        // TODO: Add other formats
        Format::Pdf => {
            let pdf = Pdf::try_from_path(input_path)?;
            Box::new(pdf.into_iter())
        }
    };

    let mut cbz = CbzWriter::create_from_path(output_path)?;
    for res in imgs {
        let img = match res {
            Ok(img) => img,
            Err(err) => {
                error!("image error: {err}");
                continue;
            }
        };

        if let Err(err) = cbz.insert_image_as_jpeg(&img, 75) {
            error!("image insertion error: {err}");
        }
    }

    cbz.finish()?;

    Ok(())
}
