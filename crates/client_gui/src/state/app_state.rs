use gpui::Entity;

use crate::state::{chat_store::ChatStore, contact_store::ContactStore};

pub struct AppState {
    pub chat_store: Entity<ChatStore>,
    pub contact_store: Entity<ContactStore>,
}
