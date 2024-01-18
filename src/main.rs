mod client;
mod process;
mod server;
mod utils;

use crate::process::process;
use crate::server::Server;
use crate::utils::LocalTimeLayer;
use anyhow::Result;
use futures_util::{stream::TryStreamExt, StreamExt};
use parking_lot::Mutex;
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

#[tokio::main]
async fn main() -> Result<()> {
    // tracing_subscriber::fmt().init();
    let subscriber = Registry::default()
        .with(LocalTimeLayer)
        .with(tracing_subscriber::fmt::layer());
    tracing::subscriber::set_global_default(subscriber)?;

    let default_addr = "127.0.0.1:8888".to_owned();
    let addr = env::args().nth(1).unwrap_or_else(|| default_addr);
    let tcp_server = TcpListener::bind(&addr).await?;
    let server = Arc::new(Mutex::new(Server::new()));
    info!("Websocket server running on {}", &addr);

    loop {
        let (stream, addr) = tcp_server.accept().await?;
        let server_clone = server.clone();

        if let Err(e) = tokio::spawn(process(server_clone, stream, addr)).await {
            error!("Processing client[{}] error: {}", &addr, e)
        }
    }
}
