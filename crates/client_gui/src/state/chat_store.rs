use crate::models::Message;
use std::collections::HashMap;

use crate::models::{Chat, ChatId};

pub struct ChatStore {
    pub conversations: HashMap<ChatId, Chat>,
    pub messages: HashMap<ChatId, Vec<Message>>,
    pub active_conversation: Option<ChatId>,
}
impl ChatStore {
    pub fn new() -> Self {
        Self {
            conversations: HashMap::new(),
            messages: HashMap::new(),
            active_conversation: None,
        }
    }
}
