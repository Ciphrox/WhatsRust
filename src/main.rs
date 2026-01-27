use std::sync::Arc;
use wacore::types::events::Event;
use whatsapp_rust::bot::Bot;
use whatsapp_rust::store::SqliteStore;
use whatsapp_rust_tokio_transport::TokioWebSocketTransportFactory;
use whatsapp_rust_ureq_http_client::UreqHttpClient;

#[tokio::main]
async fn main() {
    let backend = Arc::new(SqliteStore::new("./data/whatsapp.db").await.unwrap());

    let mut bot = Bot::builder()
        .with_backend(backend)
        .with_transport_factory(TokioWebSocketTransportFactory::new())
        .with_http_client(UreqHttpClient::new())
        .on_event(|event, _client| async move {
            match event {
                Event::PairingQrCode { code, .. } => {
                    qr2term::print_qr(code).unwrap();
                }
                Event::Message(msg, info) => {
                    println!("Message from {}: {:#?}", info.source.sender, msg);
                }
                _ => {}
            }
        })
        .build()
        .await
        .unwrap();

    bot.run().await.unwrap().await.unwrap();
}
