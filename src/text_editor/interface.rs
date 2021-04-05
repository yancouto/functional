use std::{
    collections::HashMap, convert::{TryFrom, TryInto}, io::{BufRead, Read, Write}, sync::{atomic::AtomicU64, Arc}, thread
};

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
pub use xi_core_lib::rpc::{CoreNotification, CoreRequest};
use xi_core_lib::XiCore;
use xi_rpc::{RemoteError, RpcLoop};
type ServerResponse = Result<Json, RemoteError>;
use crossbeam::channel::{unbounded as channel, Receiver, Sender, TryRecvError};

#[derive(Debug, Deserialize)]
pub struct ConfigChanges {
    #[serde(flatten)]
    extra: HashMap<String, Json>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateAnnotation {
    #[serde(rename = "type")]
    type_:    String,
    ranges:   Vec<(u32, u32, u32, u32)>,
    payloads: Option<Vec<Json>>,
    n:        u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOpType {
    Copy,
    Skip,
    Invalidate,
    Update,
    Ins,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "op")]
pub enum UpdateOp {
    Copy { n: u32, ln: u32 },
    Skip { n: u32 },
    Invalidate { n: u32 },
    Update { n: u32, lines: Vec<Json> },
    Ins { n: u32, lines: Vec<Json> },
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Update {
    pristine:    bool,
    annotations: Vec<UpdateAnnotation>,
    ops:         Vec<UpdateOp>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "method", content = "params")]
#[serde(deny_unknown_fields)]
pub enum ServerNotification {
    ScrollTo {
        view_id: String,
        line:    u32,
        col:     u32,
    },
    LanguageChanged {
        view_id:     String,
        language_id: String,
    },
    ConfigChanged {
        view_id: String,
        changes: ConfigChanges,
    },
    Update {
        view_id: String,
        update:  Update,
    },
    AvailablePlugins {
        view_id: String,
        plugins: Vec<Json>,
    },
    AvailableLanguages {
        languages: Vec<String>,
    },
    AvailableThemes {
        themes: Vec<String>,
    },
}

#[derive(Debug)]
pub enum ServerRequest {}

#[derive(Debug)]
pub enum ServerMessage {
    Response(u64, ServerResponse),
    Notification(ServerNotification),
    Request(u64, ServerRequest),
    // For non-implemented things. In the future, remove
    Unknown,
}

impl TryFrom<&[u8]> for ServerMessage {
    type Error = std::io::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let mut json = serde_json::from_slice::<Json>(bytes)?;
        if json.get("id").is_some() && json.get("method").is_none() {
            let json = json.as_object_mut().unwrap();
            let result = json.remove("result");
            let error = json
                .remove("error")
                .map(|e| serde_json::from_value::<RemoteError>(e).unwrap());
            let res = result.ok_or_else(|| error.unwrap());
            // is response
            Ok(ServerMessage::Response(
                json.get("id").and_then(Json::as_u64).unwrap(),
                res,
            ))
        } else if json.get("id").is_some() {
            // request
            Ok(ServerMessage::Unknown)
        } else {
            Ok(serde_json::from_value::<ServerNotification>(json)
                .map(|n| ServerMessage::Notification(n))
                .unwrap_or(ServerMessage::Unknown))
        }
    }
}

struct JsonSender(Sender<ServerMessage>);

impl Write for JsonSender {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let msg = buf.try_into()?;
        if matches!(msg, ServerMessage::Unknown) {
            log::debug!("Unknown message: {}", String::from_utf8_lossy(buf));
        }
        if let Err(e) = self.0.send(msg) {
            log::error!("Failed to send {:?}", e.0);
        }
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

#[derive(Clone)]
pub struct ClientMessageSender {
    sender:   Sender<ClientMessage>,
    receiver: Arc<ServerMessageReceiver>,
    id_count: Arc<AtomicU64>,
}

impl ClientMessageSender {
    pub fn send_notification(&self, msg: CoreNotification) {
        self.sender.send(msg.into()).unwrap();
    }

    pub fn send_request<C: FnOnce(ServerResponse) -> () + Send + 'static>(
        &self,
        msg: CoreRequest,
        callback: C,
    ) {
        let id = self
            .id_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.sender.send((id, msg).into()).unwrap();
        self.receiver.add_callback(id, box callback);
    }
}

type Callback = Box<dyn FnOnce(ServerResponse) -> () + Send>;

#[derive(Clone)]
pub struct ServerMessageReceiver {
    receiver:  Receiver<ServerMessage>,
    callbacks: Arc<Mutex<HashMap<u64, Callback>>>,
    notif_tx:  Sender<ServerNotification>,
    notif_rx:  Receiver<ServerNotification>,
}

impl ServerMessageReceiver {
    fn new(receiver: Receiver<ServerMessage>) -> Self {
        let (notif_tx, notif_rx) = channel();
        Self {
            receiver,
            callbacks: Arc::new(Mutex::new(HashMap::new())),
            notif_tx,
            notif_rx,
        }
    }

    fn add_callback(&self, id: u64, callback: Callback) {
        self.callbacks.lock().insert(id, callback);
    }

    pub fn next_notif(&self) -> Option<ServerNotification> { self.notif_rx.try_recv().ok() }

    fn process_response(&self, id: u64, response: ServerResponse) {
        log::trace!("Process response {}", id);
        // get before so the lock is dropped when actually calling the callback
        let maybe_callback = self.callbacks.lock().remove(&id);
        if let Some(callback) = maybe_callback {
            callback(response);
        } else {
            log::error!("Callback for id {} is missing!", id);
        }
    }

    pub fn tick(&self) {
        loop {
            match self.receiver.try_recv() {
                Ok(msg) => match msg {
                    ServerMessage::Response(id, response) => self.process_response(id, response),
                    ServerMessage::Notification(n) => self.notif_tx.send(n).unwrap(),
                    ServerMessage::Request(_, _) => todo!(),
                    ServerMessage::Unknown => {}, // ignore
                },
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    log::error!("Disconnected!");
                    break;
                },
            }
        }
    }
}

/// Returns a sender to send messages from client to Xi server, and a receiver
/// to get messages back from Xi server.
pub fn start_xi_thread() -> (ClientMessageSender, Arc<ServerMessageReceiver>) {
    let mut state = XiCore::new();
    let (server_sender, server_receiver) = channel();
    let (client_sender, client_receiver) = channel();
    thread::spawn(move || {
        let r = RpcLoop::new(JsonSender(server_sender))
            .mainloop(|| JsonReceiver::new(client_receiver), &mut state);
        log::info!("Out of Xi main loop! {:?}", r);
    });
    let recv = Arc::new(ServerMessageReceiver::new(server_receiver));
    (
        ClientMessageSender {
            sender:   client_sender,
            receiver: recv.clone(),
            id_count: Arc::new(AtomicU64::new(0)),
        },
        recv,
    )
}
