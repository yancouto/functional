use std::{
    io::{BufRead, Read, Write}, sync::mpsc::{channel, Receiver, Sender}, thread
};

use serde_json::Value as Json;
use xi_core_lib::XiCore;
use xi_rpc::RpcLoop;

struct JsonSender(Sender<Json>);

impl Write for JsonSender {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let json = serde_json::from_slice(buf)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::BrokenPipe, e))?;
        self.0.send(json).expect("Failed to send");
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        debug_assert!(false, "Not expecting flush");
        Ok(())
    }
}

struct JsonReceiver {
    recv: Receiver<Json>,
    buf:  String,
}

impl JsonReceiver {
    fn new(recv: Receiver<Json>) -> Self {
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
                .map(|json| json.to_string())
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::BrokenPipe, e))?;
        }
        Ok(self.buf.as_bytes())
    }

    fn consume(&mut self, amt: usize) {
        debug_assert!(amt == self.buf.len(), "Buf reader was not well behaved");
        self.buf.clear();
    }
}

/// Returns a sender to send messages from client to Xi server, and a receiver
/// to get messages back from Xi server.
pub fn start_xi_thread() -> (Sender<Json>, Receiver<Json>) {
    let mut state = XiCore::new();
    let (server_sender, server_receiver) = channel();
    let (client_sender, client_receiver) = channel();
    thread::spawn(move || {
        RpcLoop::new(JsonSender(server_sender))
            .mainloop(|| JsonReceiver::new(client_receiver), &mut state)
            .unwrap();
        log::info!("Out of Xi main loop!");
    });
    (client_sender, server_receiver)
}
