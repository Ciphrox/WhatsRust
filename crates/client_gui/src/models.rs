use gpui::SharedString;

use core::models_base::*;

pub type ChatId = ChatIdBase<SharedString>;
pub type MessageId = MessageIdBase<SharedString>;

pub type Sender = SenderBase<SharedString>;
pub type MessageContent = MessageContentBase<SharedString>;
pub type Message = MessageBase<SharedString>;

pub type Chat = ChatBase<SharedString>;
