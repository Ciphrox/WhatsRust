use core::events::{AppAction, WaEvent};

#[derive(Debug)]
pub enum Input {
    FromUI(AppAction),
    FromNet(WaEvent),
}
