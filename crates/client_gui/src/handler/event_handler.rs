use core::{events::AppEvent, models_base::MessageContentBase};

use gpui::{AsyncApp, Entity, SharedString};

use crate::{
    models::{ChatId, Message, MessageContent, MessageId, Sender},
    state::ChatStore,
};

pub fn handle_app_event(event: AppEvent, cx: &mut AsyncApp, store: &Entity<ChatStore>) {
    match event {
        AppEvent::NewMessage(msg) => {
            let id = MessageId::new(msg.id.0.into());
            let chat_lid: SharedString = msg.chat_lid.into();
            let sender = Sender::new(msg.sender.lid.into(), msg.sender.jid.map(|s| s.into()));
            let name = msg.name.into();
            let _ = store.update(cx, |store, cx| match msg.content {
                MessageContentBase::Text(txt) => {
                    let content = MessageContent::Text(txt.into());

                    let key = ChatId::new(chat_lid.clone());
                    let msg = Message::new(id, chat_lid.clone(), sender, name, content);
                    store.messages.entry(key).or_default().push(msg);
                    cx.notify();
                }
            });
        }
    }
}
