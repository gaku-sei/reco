use std::path::Path;

use egui_inbox::UiInboxSender;
use tracing::error;

use crate::types::Message;

pub fn back_button(path: &Path, tx: &UiInboxSender<Message>, ui: &mut egui::Ui) {
    if let Some(parent) = path.parent() {
        let parent_path_str = parent.to_string_lossy();

        if ui.button("back").clicked() {
            if let Err(err) = tx.send(Message::Navigate(parent_path_str.into_owned())) {
                error!("send error: {err:?}");
            }
        }
    }
}
