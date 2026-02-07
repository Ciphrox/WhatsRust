use core::events::AppAction;

use crate::context::AppContext;

pub fn handle_ui_action(action: AppAction, _ctx: &AppContext) {
    match action {
        AppAction::SendTextMessage {} => {}
    }
}
