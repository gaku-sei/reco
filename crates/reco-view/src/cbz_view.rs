use std::{fs::File, path::Path, sync::Arc};

use egui::{Key, TextureHandle, Vec2};
use egui_router::Route;
use image::DynamicImage;
use tracing::error;

use crate::Result;
use reco::Reader as CbzReader;

pub struct CbzView {
    cbz: CbzReader<File>,
    current_img: Arc<egui::ColorImage>,
    current_texture: TextureHandle,
}

impl CbzView {
    pub fn try_from_path(ctx: &egui::Context, path: &Path) -> Result<Self> {
        let mut cbz = CbzReader::try_open(path)?;
        let current_img = Arc::new(convert_img(&cbz.load_current_img()?));

        let current_texture =
            ctx.load_texture("image", current_img.clone(), egui::TextureOptions::LINEAR);

        Ok(Self {
            cbz,
            current_img,
            current_texture,
        })
    }

    fn go_to_prev_index(&mut self, ctx: &egui::Context) {
        self.cbz.go_to_prev_index();
        self.load_current_img(ctx);
    }

    fn go_to_next_index(&mut self, ctx: &egui::Context) {
        self.cbz.go_to_next_index();
        self.load_current_img(ctx);
    }

    fn go_to_index(&mut self, ctx: &egui::Context, index: usize) {
        if self.cbz.spine().current_index() == index {
            return;
        }

        self.cbz.go_to_index(index);
        self.load_current_img(ctx);
    }

    fn load_current_img(&mut self, ctx: &egui::Context) {
        match self.cbz.load_current_img() {
            Ok(img) => {
                self.current_img = Arc::new(convert_img(&img));
                self.current_texture = ctx.load_texture(
                    "image",
                    self.current_img.clone(),
                    egui::TextureOptions::LINEAR,
                );
            }
            Err(err) => error!("image load error: {err}"),
        }
    }
}

impl<S> Route<S> for CbzView {
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn ui(&mut self, ui: &mut egui::Ui, _state: &mut S) {
        let ctx = ui.ctx().clone();

        if ctx.input(|i| i.key_pressed(Key::ArrowLeft)) {
            self.go_to_prev_index(&ctx);
        }

        if ctx.input(|i| i.key_pressed(Key::ArrowRight)) {
            self.go_to_next_index(&ctx);
        }

        ui.vertical_centered(|ui| {
            let available = ui.available_size();
            let (width, height) =
                scale_img_boundaries(&self.current_img, available - egui::vec2(0.0, 32.0));

            let sized_image =
                egui::load::SizedTexture::new(self.current_texture.id(), egui::vec2(width, height));

            ui.image(sized_image);
        });

        ui.vertical_centered(|ui| {
            ui.horizontal_centered(|ui| {
                let current_page = self.cbz.spine().current_index();
                let pages = self.cbz.spine().len();

                if ui.button("Previous").clicked() {
                    self.go_to_prev_index(&ctx);
                }

                ui.label(format!("{} / {}", current_page + 1, pages));

                if ui.button("Next").clicked() {
                    self.go_to_next_index(&ctx);
                }

                let slider = egui::Slider::from_get_set(0.0..=pages as f64, |page| {
                    if let Some(page) = page {
                        self.go_to_index(&ctx, page as usize);
                    }
                    self.cbz.spine().current_index() as f64
                })
                .integer()
                .show_value(false)
                .text("Page");

                ui.add(slider);
            });
        });
    }
}

fn convert_img(img: &DynamicImage) -> egui::ColorImage {
    if let DynamicImage::ImageRgb8(rgb) = img {
        egui::ColorImage::from_rgb([rgb.width() as usize, rgb.height() as usize], rgb.as_raw())
    } else {
        let rgba = img.to_rgba8();
        egui::ColorImage::from_rgba_unmultiplied(
            [rgba.width() as usize, rgba.height() as usize],
            rgba.as_raw(),
        )
    }
}

#[expect(clippy::cast_precision_loss)]
fn scale_img_boundaries(img: &egui::ColorImage, boundaries: Vec2) -> (f32, f32) {
    let width = img.size[0] as f32;
    let height = img.size[1] as f32;

    if boundaries.x / boundaries.y > width / height {
        (width * boundaries.y / height, boundaries.y)
    } else {
        (boundaries.x, height * boundaries.x / width)
    }
}

#[cfg(test)]
mod tests {
    use egui::Vec2;

    use super::scale_img_boundaries;

    fn new_img(width: usize, height: usize) -> egui::ColorImage {
        egui::ColorImage {
            size: [width, height],
            ..Default::default()
        }
    }

    #[test]
    fn it_scale_img_boundaries_accurately() {
        let img = new_img(1000, 2000);
        assert_eq!(
            scale_img_boundaries(&img, Vec2::new(1000.0, 1000.0)),
            (500.0, 1000.0)
        );

        let img = new_img(1000, 2000);
        assert_eq!(
            scale_img_boundaries(&img, Vec2::new(300.0, 1000.0)),
            (300.0, 600.0)
        );

        let img = new_img(2000, 1000);
        assert_eq!(
            scale_img_boundaries(&img, Vec2::new(1000.0, 1000.0)),
            (1000.0, 500.0)
        );

        let img = new_img(2000, 1000);
        assert_eq!(
            scale_img_boundaries(&img, Vec2::new(300.0, 1000.0)),
            (300.0, 150.0)
        );
    }
}
