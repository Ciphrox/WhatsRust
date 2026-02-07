use gpui::*;

use crate::{state::ChatStore, views::chat_panel::ChatPanel};

pub struct ChatApp {
    pub chat_store: Entity<ChatStore>,
    pub chat_panel: Entity<ChatPanel>,
}

impl ChatApp {
    pub fn new(chat_store: Entity<ChatStore>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let chat_panel = cx.new(|cx| ChatPanel::new(chat_store.clone(), window, cx));
        Self {
            chat_store,
            chat_panel,
        }
    }
}

impl Render for ChatApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("chat-app")
            .key_context("ChatApp")
            .size_full()
            .flex()
            .flex_row()
            .bg(gpui::rgb(0xf0f0f0))
            .child(div().flex_1().h_full().child(self.chat_panel.clone()))
    }
}
