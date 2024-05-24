#![deny(clippy::all, clippy::pedantic, clippy::unwrap_used)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use anyhow::{Result, anyhow};
use clap::Parser;
use egui_inbox::UiInbox;
use egui_router::EguiRouter;
use tracing::error;

use crate::{
    routes::router,
    types::{Message, State},
};

mod routes;
mod types;
mod views;

#[derive(Debug, Parser)]
#[command(version, about,  long_about = None)]
struct Args {
    /// Path to the root folder/archive
    initial_path: PathBuf,
}

pub struct App {
    router: EguiRouter<State>,
    inbox: UiInbox<Message>,
    state: State,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);
        ctx.set_pixels_per_point(1.25);

        egui::CentralPanel::default().show(ctx, |ui| {
            // TODO: Properly encode path
            if let Some(Message::Navigate(path)) = self.inbox.read(ui).last()
                && let Err(err) = self.router.navigate(&mut self.state, format!("/{path}"))
            {
                error!("navigation error: {err}");
            }

            self.router.ui(ui, &mut self.state);
        });
    }
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let (tx, inbox) = UiInbox::channel();

    let mut state = State::new(tx);

    let router = router(&args.initial_path, &mut state);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 720.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Reco",
        options,
        Box::new(|_cc| {
            Ok(Box::new(App {
                router,
                inbox,
                state,
            }))
        }),
    )
    .map_err(|err| anyhow!("app init error: {err}"))?;

    Ok(())
}
