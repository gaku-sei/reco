#![expect(clippy::needless_pass_by_value)]

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use egui::Ui;
use egui_router::{
    EguiRouter, HandlerError, HandlerResult, Request, Route, TransitionConfig,
    history::DefaultHistory,
};

use crate::{
    types::State,
    views::{CbzView, DirectoryView, UnknownFileView},
};

type Handler = HandlerResult<Box<dyn FnMut(&mut egui::Ui, &mut State)>>;

fn render(req: Request<State>) -> Handler {
    let path = req.params.get("path").ok_or(HandlerError::NotFound)?;
    let path = PathBuf::from(path);
    let ext = path.extension().and_then(OsStr::to_str);

    let mut view: Box<dyn Route<State>> = if path.is_dir() {
        Box::new(DirectoryView::new(path))
    } else if path.is_file() && ext == Some("cbz") {
        Box::new(CbzView::new(path))
    } else {
        Box::new(UnknownFileView::new(path))
    };

    Ok(Box::new(move |ui: &mut Ui, state: &mut State| {
        view.ui(ui, state);
    }))
}

pub fn router(initial_path: &Path, state: &mut State) -> EguiRouter<State> {
    EguiRouter::builder()
        .history(DefaultHistory::default())
        .transition(TransitionConfig::fade())
        .default_duration(0.3)
        .default_path(format!("/{}", initial_path.display()))
        .route("/{path}", render)
        .build(state)
}
