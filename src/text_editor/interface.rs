use std::{
    collections::HashMap, io::{BufRead, Read, Write}, rc::Rc, sync::mpsc::{channel, Receiver, Sender}, thread
};

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
pub use xi_core_lib::rpc::{CoreNotification, CoreRequest};
use xi_core_lib::XiCore;
use xi_rpc::{RemoteError, RpcLoop};

type ServerResponse = Result<Json, RemoteError>;

#[derive(Debug, Deserialize, Serialize)]
pub enum ServerMessage {
    Response(u64, ServerResponse),
    // For non-implemented things. In the future, remove
    Unknown,
}

struct JsonSender(Sender<ServerMessage>);

impl Write for JsonSender {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut json = serde_json::from_slice::<Json>(buf)?;

        let msg = if json.get("id").is_some() && json.get("method").is_none() {
            let json = json.as_object_mut().unwrap();
            let result = json.remove("result");
            let error = json
                .remove("error")
                .map(|e| serde_json::from_value::<RemoteError>(e).unwrap());
            let res = result.ok_or_else(|| error.unwrap());
            // is response
            ServerMessage::Response(json.get("id").and_then(Json::as_u64).unwrap(), res)
        } else {
            log::debug!("Unknown message: {}", String::from_utf8_lossy(buf));
            ServerMessage::Unknown
        };

        self.0.send(msg).expect("Failed to send");
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        debug_assert!(false, "Not expecting flush");
        Ok(())
    }
}

// TODO: Will need to include responses
#[derive(Debug)]
pub enum ClientMessage {
    Notification(CoreNotification),
    Request(u64, CoreRequest),
}

impl Serialize for ClientMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Just serialize directly
        match self {
            Self::Notification(n) => n.serialize(serializer),
            Self::Request(_, r) => r.serialize(serializer),
        }
    }
}

impl From<CoreNotification> for ClientMessage {
    fn from(n: CoreNotification) -> Self { Self::Notification(n) }
}

impl From<(u64, CoreRequest)> for ClientMessage {
    fn from((id, r): (u64, CoreRequest)) -> Self { Self::Request(id, r) }
}

struct JsonReceiver {
    recv: Receiver<ClientMessage>,
    buf:  String,
}

impl JsonReceiver {
    fn new(recv: Receiver<ClientMessage>) -> Self {
        Self {
            recv,
            buf: String::new(),
        }
    }
}

impl Read for JsonReceiver {
    fn read(&mut self, mut buf: &mut [u8]) -> Result<usize, std::io::Error> {
        debug_assert!(false, "Shouldn't call read directly!");
        let ret = buf.write(self.buf.as_bytes());
        self.buf.clear();
        ret
    }
}

impl BufRead for JsonReceiver {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        debug_assert!(self.buf.is_empty(), "Reading from same buffer again!");
        if self.buf.is_empty() {
            self.buf = self
                .recv
                .recv()
                .map(|msg| {
                    let mut json_msg = serde_json::to_value(&msg).unwrap();
                    if let ClientMessage::Request(id, _) = msg {
                        match &mut json_msg {
                            serde_json::Value::Object(map) => {
                                map.insert("id".to_owned(), serde_json::json!(id));
                            },
                            _ => panic!("Invalid ClientMessage"),
                        }
                    }
                    let mut str = json_msg.to_string();
                    str.push('\n');
                    str
                })
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::BrokenPipe, e))?;
        }
        println!("Send: {}", self.buf);
        Ok(self.buf.as_bytes())
    }

    fn consume(&mut self, amt: usize) {
        debug_assert!(amt == self.buf.len(), "Buf reader was not well behaved");
        self.buf.clear();
    }
}

pub struct ClientMessageSender {
    sender:   Sender<ClientMessage>,
    receiver: Rc<ServerMessageReceiver>,
    id_count: u64,
}

impl ClientMessageSender {
    pub fn send_notification(&self, msg: CoreNotification) {
        self.sender.send(msg.into()).unwrap();
    }

    pub fn send_request<C: FnOnce(ServerResponse) -> () + Send + 'static>(
        &mut self,
        msg: CoreRequest,
        callback: C,
    ) {
        let id = self.id_count;
        self.id_count += 1;
        self.sender.send((id, msg).into()).unwrap();
        self.receiver.add_callback(id, box callback);
    }
}

type Callback = Box<dyn FnOnce(ServerResponse) -> () + Send>;
pub struct ServerMessageReceiver {
    receiver:  Receiver<ServerMessage>,
    callbacks: Mutex<HashMap<u64, Callback>>,
}

impl ServerMessageReceiver {
    fn new(receiver: Receiver<ServerMessage>) -> Self {
        Self {
            receiver,
            callbacks: Mutex::new(HashMap::new()),
        }
    }

    fn add_callback(&self, id: u64, callback: Callback) {
        self.callbacks.lock().insert(id, callback);
    }
}

impl ServerMessageReceiver {
    pub fn block_until_next_message(&self) -> ServerMessage { self.receiver.recv().unwrap() }

    fn process_response(&self, id: u64, response: ServerResponse) {
        if let Some(callback) = self.callbacks.lock().remove(&id) {
            callback(response);
        } else {
            log::error!("Callback for id {} is missing!", id);
        }
    }

    pub fn main_loop(&self) {
        loop {
            match self.receiver.recv() {
                Ok(msg) => match msg {
                    ServerMessage::Response(id, response) => self.process_response(id, response),
                    ServerMessage::Unknown => {},
                },
                Err(err) => {
                    log::error!("Error! {:?}", err);
                    break;
                },
            }
        }
    }
}

/// Returns a sender to send messages from client to Xi server, and a receiver
/// to get messages back from Xi server.
pub fn start_xi_thread() -> (ClientMessageSender, Rc<ServerMessageReceiver>) {
    let mut state = XiCore::new();
    let (server_sender, server_receiver) = channel();
    let (client_sender, client_receiver) = channel();
    thread::spawn(move || {
        let r = RpcLoop::new(JsonSender(server_sender))
            .mainloop(|| JsonReceiver::new(client_receiver), &mut state);
        log::info!("Out of Xi main loop! {:?}", r);
    });
    let recv = Rc::new(ServerMessageReceiver::new(server_receiver));
    (
        ClientMessageSender {
            sender:   client_sender,
            receiver: recv.clone(),
            id_count: 0,
        },
        recv,
    )
}
