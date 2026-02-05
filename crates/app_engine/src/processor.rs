use crate::context::AppContext;
use crate::handlers::{handle_ui_action, handler_net_event};

use crate::input::Input;

pub struct Processor {}

impl Processor {
    pub fn process(input: Input, ctx: &AppContext) {
        match input {
            Input::FromUI(action) => handle_ui_action(action, ctx),
            Input::FromNet(event) => handler_net_event(event, ctx),
        }
    }
}
