use std::path::PathBuf;

use egui_router::Route;

use super::back_button::back_button;
use crate::types::State;

pub struct UnknownFileView {
    path: PathBuf,
}

impl UnknownFileView {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Route<State> for UnknownFileView {
    fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        back_button(&self.path, &state.tx, ui);
    }
}
