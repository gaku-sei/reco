use std::{fs::read_dir, io, path::PathBuf};

use egui_router::Route;
use tracing::error;

use super::back_button::back_button;
use crate::types::{Message, State};

pub struct DirectoryView {
    path: PathBuf,
    paths: Option<io::Result<Vec<String>>>,
}

impl DirectoryView {
    pub fn new(path: PathBuf) -> Self {
        Self { path, paths: None }
    }
}

impl Route<State> for DirectoryView {
    fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        back_button(&self.path, &state.tx, ui);

        let paths = self.paths.get_or_insert_with(|| {
            let entries = read_dir(&self.path)?;
            let mut paths = Vec::new();
            for entry in entries {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(err) => {
                        error!("entry error: {err}");
                        continue;
                    }
                };

                let path = entry.path();
                let path_str = path.to_string_lossy();
                let ext = path.extension().and_then(|ext| ext.to_str());

                if path.is_dir() || (path.is_file() && ext == Some("cbz")) {
                    paths.push(path_str.into_owned());
                }
            }

            Ok(paths)
        });

        let paths = match paths {
            Ok(paths) => paths,
            Err(err) => {
                ui.label(format!("read dir error: {err}"));
                return;
            }
        };

        egui::ScrollArea::vertical().show(ui, |ui| {
            for path in paths {
                if ui.button(&*path).clicked()
                    && let Err(err) = state.tx.send(Message::Navigate(path.clone()))
                {
                    error!("send error: {err:?}");
                }
            }
        });
    }
}
