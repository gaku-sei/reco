use egui_inbox::UiInboxSender;

pub struct State {
    pub tx: UiInboxSender<Message>,
}

impl State {
    pub fn new(tx: UiInboxSender<Message>) -> Self {
        Self { tx }
    }
}

#[derive(Debug)]
pub enum Message {
    Navigate(String),
}
