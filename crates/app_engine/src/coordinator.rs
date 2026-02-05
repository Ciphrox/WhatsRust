use crate::{context::AppContext, processor::Processor};
use core::events::{AppAction, AppEvent};

use crate::input::Input;
use flume::{Receiver, Sender};
use network::client::NetworkClient;

pub async fn start(ui_rx: Receiver<AppAction>, evt_tx: Sender<AppEvent>) {
    //TODO: db connection
    let (funnel_tx, funnel_rx) = flume::unbounded::<Input>();

    let tx = funnel_tx.clone();
    let client = NetworkClient::start(move |event, _| {
        let tx = tx.clone();
        async move {
            let _ = tx.send_async(Input::FromNet(event)).await;
        }
    })
    .await;

    let ui_forwarder = funnel_tx.clone();
    tokio::spawn(async move {
        while let Ok(action) = ui_rx.recv_async().await {
            let _ = ui_forwarder.send_async(Input::FromUI(action)).await;
        }
    });

    let ctx = AppContext::new(client, evt_tx);

    while let Ok(input) = funnel_rx.recv_async().await {
        Processor::process(input, &ctx);
    }
}
