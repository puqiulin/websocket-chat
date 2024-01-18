use futures_util::stream::{SplitSink, SplitStream};
use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use uuid::Uuid;

type WsStream = WebSocketStream<TcpStream>;
type Rx = UnboundedReceiver<Message>;

pub struct Client {
    pub uuid: Uuid,
    pub name: String,
    pub read: SplitStream<WsStream>,
    pub write: SplitSink<WsStream, Message>,
    pub rx: Rx,
}

impl Client {
    pub fn new(ws_stream: WsStream, rx: Rx) -> Self {
        let (write, read) = ws_stream.split();
        let uuid = Uuid::new_v4();
        Client {
            uuid: Uuid::new_v4(),
            name: uuid.to_string(),
            read,
            write,
            rx,
        }
    }
}
