use std::path::Path;

pub use cbz_view::CbzView;
use egui::Key;
use egui_router::Route;
pub use errors::{Error, Result};

mod cbz_view;
mod errors;

pub struct App {
    fullscreen: bool,
    cbz_view: CbzView,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);
        ctx.set_pixels_per_point(1.25);

        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        if ctx.input(|i| i.key_pressed(Key::F11)) {
            self.fullscreen = !self.fullscreen;
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.fullscreen));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.cbz_view.ui(ui, &mut ());
        });
    }
}

pub fn view(path: &Path) -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 720.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        &format!("Reco - {}", path.to_string_lossy()),
        options,
        Box::new(|cc| {
            let cbz_view = App {
                fullscreen: false,
                cbz_view: CbzView::try_from_path(&cc.egui_ctx, path)?,
            };
            Ok(Box::new(cbz_view))
        }),
    )
    .map_err(|err| Error::Eframe(err.to_string()))?;

    Ok(())
}
