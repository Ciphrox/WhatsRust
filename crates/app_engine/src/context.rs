use core::events::AppEvent;

use flume::Sender;
use network::client::NetworkClient;

pub struct AppContext {
    //TODO: pub db:
    pub client: NetworkClient,
    pub ui: Sender<AppEvent>,
}

impl AppContext {
    pub fn new(client: NetworkClient, ui: Sender<AppEvent>) -> Self {
        Self { client, ui }
    }
}
