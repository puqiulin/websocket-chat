use crate::client::Client;
use crate::message::Message;
use crate::server::Server;
use crate::utils::get_local_time;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use parking_lot::Mutex;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::mpsc::unbounded_channel;
use tokio_tungstenite::tungstenite::Message::Text;
use tracing::{error, info};

pub async fn process(
    server: Arc<Mutex<Server>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<()> {
    info!("Accept a client: {}, waiting for a name...", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream).await?;
    let (tx, rx) = unbounded_channel();

    //new client
    //TODO check name already exists
    let mut client = Client::new(ws_stream, rx);
    let need_name = format!("[{}] [system] Please enter your name: ", get_local_time());
    client.write.send(Text(need_name)).await?;
    let message = client.read.next().await.unwrap()?;
    let name = message.to_text()?;
    client
        .write
        .send(Text(format!("[{}] Hello! {}", get_local_time(), name)))
        .await?;

    //add new client
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
        s.broadcast_all(
            addr,
            &format!("[{}] [system] {}", get_local_time(), new_client_msg),
        )?;
    }

    //handle client message
    loop {
        select! {
            Some(message)=client.rx.recv()=>{
                 client.write.send(message).await?;
            }
            next=client.read.next()=>match next{
                Some(Ok(message))=>{
                    let text=message.to_text()?;
                    let message=serde_json::from_str::<Message>(text)?;
                    let msg = format!("[{}] {}: {}", message.date, message.name, message.content);
                    info!(msg);
                    server.lock().broadcast_except_self(addr, &text)?;
                }
                Some(Err(e))=>{
                    error!("Failed to read message from {}: {}",name,e);
                }
                None=>break
            }
        }
    }

    //remove client when disconnect
    {
        let mut s = server.lock();
        s.clients.remove(&addr);
        let remove_user_msg = format!(
            "[{}] client [{}] disconnected, current user number: {}",
            get_local_time(),
            name,
            s.clients.len()
        );
        info!("{}", remove_user_msg);
        s.broadcast_except_self(addr, &remove_user_msg)?;
    }

    Ok(())
}
