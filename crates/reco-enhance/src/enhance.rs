use std::sync::Arc;

use image::{ImageBuffer, RgbImage};
use ndarray::Array4;
use ort::{session::Session, value::TensorRef};
use parking_lot::Mutex;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use tracing::{error, info, warn};

use crate::Error;

const TILE_SIZE: u32 = 128;
const PADDING: u32 = 8; // 16 ?
const SCALE: u32 = 4; // x4 model

pub fn enhance_img(sessions: Arc<Vec<Mutex<Session>>>, img: &RgbImage) -> Result<RgbImage, Error> {
    let sessions_count = sessions.len();
    let tiles = split_into_tiles(img);

    let enhanced_tiles = tiles
        .into_par_iter()
        .enumerate()
        .filter_map(|(index, (x, y, tile))| {
            info!("enhancing tile at {x}x{y}");

            let mut session = sessions.get(index % sessions_count)?.lock();
            match enhance_tile(&mut session, &tile) {
                Ok(Some(enhanced_tile)) => {
                    info!("enhanced tile at {x}x{y}");
                    Some((x, y, enhanced_tile))
                }
                Ok(None) => {
                    warn!("tile at {x}x{y} was invalid");
                    None
                }
                Err(err) => {
                    error!("tile enhancement at {x}x{y} failed: {err}");
                    None
                }
            }
        })
        .collect();

    let output_img = stitch_tiles(img, enhanced_tiles);

    Ok(output_img)
}

pub fn build_sessions(num_threads: usize) -> Result<Arc<Vec<Mutex<Session>>>, Error> {
    let mut sessions = Vec::with_capacity(num_threads);
    for _ in 0..num_threads {
        sessions.push(Mutex::new(
            Session::builder()?
                .with_intra_threads(1)?
                .commit_from_file("models/model.onnx")?,
        ));
    }

    Ok(Arc::new(sessions))
}

fn enhance_tile(session: &mut Session, img: &RgbImage) -> Result<Option<RgbImage>, Error> {
    let (width, height) = img.dimensions();

    if width != TILE_SIZE || height != TILE_SIZE {
        return Ok(None);
    }

    debug_assert_eq!(width, TILE_SIZE);
    debug_assert_eq!(height, TILE_SIZE);

    let mut input = Array4::<f32>::zeros((1, 3, TILE_SIZE as usize, TILE_SIZE as usize));
    for (x, y, pixel) in img.enumerate_pixels() {
        let [r, g, b] = pixel.0;
        input[[0, 0, y as usize, x as usize]] = r as f32 / 255.0;
        input[[0, 1, y as usize, x as usize]] = g as f32 / 255.0;
        input[[0, 2, y as usize, x as usize]] = b as f32 / 255.0;
    }

    let outputs = session.run(ort::inputs!["image" => TensorRef::from_array_view(&input)?])?;
    let output = outputs["upscaled_image"].try_extract_array::<f32>()?;

    let shape = output.shape();
    debug_assert_eq!(shape.len(), 4);

    let (_n, _c, out_h, out_w) = (shape[0], shape[1], shape[2], shape[3]);
    debug_assert_eq!(_n, 1);
    debug_assert_eq!(_c, 3);
    debug_assert_eq!(out_h, 512);
    debug_assert_eq!(out_w, 512);

    let mut output_img = ImageBuffer::new(out_w as u32, out_h as u32);

    for y in 0..out_h {
        for x in 0..out_w {
            let r = (output[[0, 0, y, x]].clamp(0.0, 1.0) * 255.0) as u8;
            let g = (output[[0, 1, y, x]].clamp(0.0, 1.0) * 255.0) as u8;
            let b = (output[[0, 2, y, x]].clamp(0.0, 1.0) * 255.0) as u8;
            output_img.put_pixel(x as u32, y as u32, image::Rgb([r, g, b]));
        }
    }

    Ok(Some(output_img))
}

fn split_into_tiles(img: &RgbImage) -> Vec<(u32, u32, RgbImage)> {
    let (width, height) = img.dimensions();
    let stride = TILE_SIZE - PADDING * 2;

    let mut tiles = Vec::new();
    for mut y in (0..height).step_by(stride as usize) {
        for mut x in (0..width).step_by(stride as usize) {
            if width.saturating_sub(x.saturating_sub(PADDING)) < TILE_SIZE {
                x = width.saturating_sub(TILE_SIZE.saturating_sub(PADDING));
            }

            if height.saturating_sub(y.saturating_sub(PADDING)) < TILE_SIZE {
                y = height.saturating_sub(TILE_SIZE.saturating_sub(PADDING));
            }

            let x = x.saturating_sub(PADDING);
            let y = y.saturating_sub(PADDING);

            let tile = image::imageops::crop_imm(img, x, y, TILE_SIZE, TILE_SIZE).to_image();

            tiles.push((x, y, tile));
        }
    }

    tiles
}

fn stitch_tiles(img: &RgbImage, tiles: Vec<(u32, u32, RgbImage)>) -> RgbImage {
    let (width, height) = img.dimensions();

    let mut img = RgbImage::new(width * SCALE, height * SCALE);

    for (x, y, tile) in tiles {
        // TODO: We need to crop based on the padding to remove potential artifacts on the sides
        // let cropped = image::imageops::crop_imm(
        //     &tile,
        //     PADDING.saturating_mul(SCALE),
        //     PADDING.saturating_mul(SCALE),
        //     tile.width()
        //         .saturating_sub(2 * PADDING * SCALE)
        //         .min(img.width().saturating_sub(x * SCALE)),
        //     tile.height()
        //         .saturating_sub(2 * PADDING * SCALE)
        //         .min(img.height().saturating_sub(y * SCALE)),
        // )
        // .to_image();

        image::imageops::replace(&mut img, &tile, (x * SCALE).into(), (y * SCALE).into());
    }

    img
}
