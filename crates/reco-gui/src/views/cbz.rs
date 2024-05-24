use std::path::PathBuf;

use egui_router::Route;
use reco_view::CbzView as RecoCbzView;
use tracing::error;

use super::back_button::back_button;
use crate::types::State;

pub struct CbzView {
    initialized: bool,
    path: PathBuf,
    // TODO: Use option result and display error
    inner_view: Option<RecoCbzView>,
}

impl CbzView {
    pub fn new(path: PathBuf) -> Self {
        Self {
            initialized: false,
            path,
            inner_view: None,
        }
    }
}

impl Route<State> for CbzView {
    fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        if !self.initialized {
            let inner_view = match RecoCbzView::try_from_path(ui.ctx(), &self.path) {
                Ok(inner_view) => Some(inner_view),
                Err(err) => {
                    error!("cbz view error: {err}");
                    None
                }
            };

            self.initialized = true;
            self.inner_view = inner_view;
        }

        back_button(&self.path, &state.tx, ui);

        if let Some(inner_view) = &mut self.inner_view {
            inner_view.ui(ui, state);
        }
    }
}
