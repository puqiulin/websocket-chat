use anyhow::Result;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::Message::Text;

type Tx = UnboundedSender<Message>;

pub struct Server {
    pub clients: HashMap<SocketAddr, Tx>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            clients: HashMap::new(),
        }
    }

    pub fn broadcast(&mut self, addr: SocketAddr, message: &str) -> Result<()> {
        let other_clients = self
            .clients
            .iter()
            .filter(|(client_addr, _)| client_addr != &&addr)
            .map(|(_, ws_sink)| ws_sink);

        for c in other_clients {
            c.send(Text(message.to_owned()))?;
        }
        Ok(())
    }
}
