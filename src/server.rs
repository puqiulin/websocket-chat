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

    pub fn broadcast_except_self(&mut self, sender: SocketAddr, message: &str) -> Result<()> {
        for c in self.clients.iter_mut() {
            if *c.0 != sender {
                c.1.send(Text(message.to_owned()))?;
            }
        }

        Ok(())
    }

    pub fn broadcast_all(&mut self, sender: SocketAddr, message: &str) -> Result<()> {
        for c in self.clients.iter_mut() {
            c.1.send(Text(message.to_owned()))?;
        }

        Ok(())
    }
}
