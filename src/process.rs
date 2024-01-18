use crate::client::Client;
use crate::server::Server;
use crate::utils::get_local_time;
use anyhow::Result;
use futures_util::{future, pin_mut, SinkExt, StreamExt, TryStreamExt};
use parking_lot::Mutex;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::mpsc::unbounded_channel;
use tokio_tungstenite::tungstenite::Message::Text;
use tracing::info;
use uuid::Uuid;

pub async fn process(
    server: Arc<Mutex<Server>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<()> {
    info!("Accept a client: {}, waiting for a name...", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream).await?;
    let (tx, rx) = unbounded_channel();

    //client stuff
    let mut client = Client::new(ws_stream, rx);
    let need_name = "Please enter your name".to_owned();
    client.write.send(Text(need_name)).await?;
    let message = client.read.next().await.unwrap()?;
    let name = message.to_text()?;

    //server stuff
    {
        let mut s = server.lock();
        s.clients.insert(addr, tx);
        let new_client_msg = format!(
            "new client[{}] connected: [{}], current user number: {}",
            &addr,
            name,
            s.clients.len()
        );
        info!("{}", new_client_msg);
        s.broadcast(addr, &new_client_msg)?;
    }

    while let Some(Ok(message)) = client.read.next().await {
        let msg = format!("[{}] {}: {}", get_local_time(), name, message.to_text()?);
        server.lock().broadcast(addr, &msg)?;
    }

    Ok(())
}
