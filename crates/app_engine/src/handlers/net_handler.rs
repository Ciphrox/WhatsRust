use core::{
    events::{AppEvent, WaEvent},
    models::{Message, MessageContent, Sender},
    models_base::MessageIdBase,
};

use crate::context::AppContext;

pub fn handler_net_event(event: WaEvent, ctx: &AppContext) {
    match event {
        WaEvent::PairingQrCode { code, timeout } => {
            qr2term::print_qr(code).unwrap();
        }
        WaEvent::Connected(_connected) => {
            println!("Connected");
        }
        WaEvent::Disconnected(_disconnected) => {
            println!("Disconnected");
        }
        WaEvent::PairSuccess(_pair_success) => {
            println!("PAIR SUCCESS");
        }
        WaEvent::PairError(_pair_error) => {
            println!("PAIR ERROR");
        }
        WaEvent::LoggedOut(_logged_out) => {
            println!("LOGGED OUT");
        }
        WaEvent::PairingCode { code, timeout } => {
            println!("PairingCode");
        }
        WaEvent::QrScannedWithoutMultidevice(_qr_scanned_without_multidevice) => {
            println!("QrScannedWithoutMultidevice");
        }
        WaEvent::ClientOutdated(_client_outdated) => {
            println!("ClientOutdated");
        }
        WaEvent::Message(message, info) => {
            println!("Message");
            let id = info.id;
            let chat_lid = info.source.chat.to_string();
            let sender_lid = info.source.sender.to_string();
            let sender_jid = info.source.sender_alt.map(|jid| jid.to_string());
            let sender = Sender::new(sender_lid, sender_jid);
            let name = info.push_name;

            if let Some(text) = message.conversation {
                let msg_content = MessageContent::Text(text);
                let new_message =
                    Message::new(MessageIdBase::new(id), chat_lid, sender, name, msg_content);

                let _ = ctx.ui.send(AppEvent::NewMessage(new_message));
            }
        }
        WaEvent::Receipt(_receipt) => {
            println!("Receipt");
        }
        WaEvent::UndecryptableMessage(_undecryptable_message) => {
            println!("UndecryptableMessage");
        }
        WaEvent::Notification(_node) => {
            println!("Notification");
        }
        WaEvent::ChatPresence(_chat_presence_update) => {
            println!("ChatPresence");
        }
        WaEvent::Presence(_presence_update) => {
            println!("Presence");
        }
        WaEvent::PictureUpdate(_picture_update) => {
            println!("PictureUpdate");
        }
        WaEvent::UserAboutUpdate(_user_about_update) => {
            println!("UserAboutUpdate");
        }
        WaEvent::JoinedGroup(_lazy_conversation) => {
            println!("JoinedGroup");
        }
        WaEvent::GroupInfoUpdate { jid, update } => {
            println!("GroupInfoUpdate");
        }
        WaEvent::ContactUpdate(_contact_update) => {
            println!("ContactUpdate");
        }
        WaEvent::PushNameUpdate(_push_name_update) => {
            println!("PushNameUpdate");
        }
        WaEvent::SelfPushNameUpdated(_self_push_name_updated) => {
            println!("SelfPushNameUpdated");
        }
        WaEvent::PinUpdate(_pin_update) => {
            println!("PinUpdate");
        }
        WaEvent::MuteUpdate(_mute_update) => {
            println!("MuteUpdate");
        }
        WaEvent::ArchiveUpdate(_archive_update) => {
            println!("ArchiveUpdate");
        }
        WaEvent::MarkChatAsReadUpdate(_mark_chat_as_read_update) => {
            println!("MarkChatAsReadUpdate");
        }
        WaEvent::HistorySync(_history_sync) => {
            println!("HistorySync");
        }
        WaEvent::OfflineSyncPreview(_offline_sync_preview) => {
            println!("OfflineSyncPreview");
        }
        WaEvent::OfflineSyncCompleted(_offline_sync_completed) => {
            println!("OFFLINE SYNC COMPLETED");
        }
        WaEvent::DeviceListUpdate(_device_list_update) => {
            println!("DeviceListUpdate");
        }
        WaEvent::StreamReplaced(_stream_replaced) => {
            println!("StreamReplaced");
        }
        WaEvent::TemporaryBan(_temporary_ban) => {
            println!("TemporaryBan");
        }
        WaEvent::ConnectFailure(_connect_failure) => {
            println!("ConnectFailure");
        }
        WaEvent::StreamError(_stream_error) => {
            println!("StreamError");
        }
    }
}
