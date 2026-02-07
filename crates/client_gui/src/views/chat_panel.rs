use gpui::*;

use crate::state::ChatStore;

pub struct ChatPanel {
    pub chat_store: Entity<ChatStore>,
}
impl Render for ChatPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let chat_store = self.chat_store.read(cx);
        div().children(chat_store.messages.iter().map(|(id, msgs)| {
            div()
                .child(format!("Chat Id : {:?}", id))
                .children(msgs.iter().map(|msg| match &msg.content {
                    crate::models::MessageContent::Text(txt) => format!("Message: {}", txt),
                }))
        }))
    }
}

impl ChatPanel {
    pub fn new(
        chat_store: Entity<ChatStore>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self { chat_store }
    }
}
