use std::{env, fs, io, sync::Arc};

use wacore::types::events::Event as WaEvent;
use whatsapp_rust::{bot::Bot, client::Client, store::SqliteStore};

use whatsapp_rust_tokio_transport::TokioWebSocketTransportFactory;
use whatsapp_rust_ureq_http_client::UreqHttpClient;

fn make_db_path(db_file_name: &str) -> io::Result<String> {
    let appdata = env::var("APPDATA")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "%APPDATA% not set"))?;

    let dir = format!(r"{}\WhatsRust", appdata);

    fs::create_dir_all(&dir)?;

    Ok(format!(r"{}\{}", dir, db_file_name))
}

pub struct NetworkClient {
    pub driver: Arc<Client>,
}

impl NetworkClient {
    pub async fn start<F, Fut>(event_handler: F) -> Self
    where
        F: Fn(WaEvent, Arc<Client>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let backend_path = make_db_path("whatsapp_session.db")
            .expect("Critical error: Failed to initialize database path");
        let backend = Arc::new(SqliteStore::new(&backend_path).await.unwrap());

        let mut bot = Bot::builder()
            .with_backend(backend)
            .with_transport_factory(TokioWebSocketTransportFactory::new())
            .with_http_client(UreqHttpClient::new())
            .on_event(event_handler)
            .build()
            .await
            .unwrap();

        let driver_client = bot.client().clone();

        tokio::spawn(async move {
            bot.run().await.unwrap().await.unwrap();
        });

        Self {
            driver: driver_client,
        }
    }
}
