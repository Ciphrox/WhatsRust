use crate::models::Message;

pub use whatsapp_rust::types::events::Event as WaEvent;

#[derive(Debug, Clone)]
pub enum AppAction {
    SendTextMessage {},
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    NewMessage(Message),
}
