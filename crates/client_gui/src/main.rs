mod handler;
mod models;
mod state;
mod views;

use crate::{state::ChatStore, views::chat_app::ChatApp};

use app_engine::coordinator;
use core::events::{AppAction, AppEvent};
// use gpui_component::*;

use gpui::*;

#[tokio::main]
async fn main() {
    let (_ui_tx, ui_rx) = flume::unbounded::<AppAction>();
    let (evt_tx, evt_rx) = flume::unbounded::<AppEvent>();

    Application::new().run(move |cx| {
        // gpui_component::init(cx);

        tokio::spawn(async move {
            coordinator::start(ui_rx, evt_tx).await;
        });

        let chat_store = cx.new(|_| ChatStore::new());

        let rec = evt_rx.clone();
        let chat_store_clone = chat_store.clone();
        cx.spawn(async move |cx| {
            while let Ok(event) = rec.recv_async().await {
                handler::handle_app_event(event, cx, &chat_store_clone);
            }
        })
        .detach();

        let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| ChatApp::new(chat_store, window, cx)),
        )
        .unwrap();
    });
}
