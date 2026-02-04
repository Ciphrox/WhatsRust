use derive_new::new;

#[derive(Debug, Clone, Hash, Eq, PartialEq, new)]
pub struct ChatIdBase<S>(pub S);

#[derive(Debug, Clone, Hash, Eq, PartialEq, new)]
pub struct MessageIdBase<S>(pub S);

#[derive(Debug, Clone, new)]
pub struct SenderBase<S> {
    pub lid: S,
    pub jid: Option<S>,
}

#[derive(Debug, Clone, new)]
pub enum MessageContentBase<S> {
    Text(S),
}

#[derive(Debug, Clone, new)]
pub struct MessageBase<S> {
    pub id: MessageIdBase<S>,
    pub chat_lid: S,
    pub sender: SenderBase<S>,
    pub name: S,

    pub content: MessageContentBase<S>,
}

#[derive(Debug, Clone, new)]
pub struct ChatBase<S> {
    pub id: ChatIdBase<S>,
    pub name: S,
}
