use std::path::Path;

use enhance::{build_sessions, enhance_img};
pub use errors::Error;
use reco::{Reader as CbzReader, Writer as CbzWriter};

mod enhance;
pub mod errors;

pub fn init(dll_path: &Path) -> Result<(), Error> {
    ort::init_from(dll_path.to_string_lossy()).commit()?;

    Ok(())
}

pub fn enhance(input: &Path, output: &Path, num_threads: usize) -> Result<(), Error> {
    let mut input_cbz = CbzReader::try_open(input)?;
    let mut output_cbz = CbzWriter::create_from_path(output)?;

    let sessions = build_sessions(num_threads)?;

    for index in 0..input_cbz.spine().len() {
        input_cbz.go_to_index(index);
        let img = input_cbz.load_current_img()?.to_rgb8();
        let img = enhance_img(sessions.clone(), &img)?;
        output_cbz.insert_image_as_jpeg(&img, 75)?;
    }

    Ok(())
}
