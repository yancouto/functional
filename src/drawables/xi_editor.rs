use std::{convert::TryFrom, iter::FromIterator, time::Duration};

use serde::Deserialize;
use xi_core_lib::rpc::{EditCommand, EditNotification, EditRequest};

use crate::{gamestates::base::TickData, math::*, prelude::*, text_editor::interface::*};
#[derive(Debug)]
pub struct XiEditor {
    pos:               Pos,
    cursor:            Pos,
    size:              Size,
    text:              Vec1<Vec<char>>,
    cursor_blink_rate: Duration,
    view_id:           xi_core_lib::ViewId,
    send:              ClientMessageSender,
    recv:              ServerMessageReceiver,
}

impl XiEditor {
    pub fn new(pos: Pos, size: Size) -> Self {
        let (send, recv) = start_xi_thread();
        send.send_notification(CoreNotification::ClientStarted {
            config_dir:        None,
            client_extras_dir: None,
        });
        let resp = serde_json::from_value::<ViewId>(
            send.send_request_block(CoreRequest::NewView { file_path: None })
                .unwrap(),
        )
        .unwrap();

        Self {
            pos,
            cursor: Pos { i: 0, j: 0 },
            size,
            text: vec1![vec![]],
            cursor_blink_rate: Duration::from_secs_f32(0.5),
            send,
            recv,
            view_id: resp.0,
        }
    }

    fn edit_notif(&self, cmd: EditNotification) -> CoreNotification {
        CoreNotification::Edit(EditCommand {
            view_id: self.view_id,
            cmd,
        })
    }

    pub fn on_event(&mut self, event: &bl::BEvent) {
        match event {
            bl::BEvent::Character { c } =>
                if !c.is_control() {
                    self.send
                        .send_notification(self.edit_notif(EditNotification::Insert {
                            chars: String::from_iter(&[*c]),
                        }));
                },
            bl::BEvent::KeyboardInput {
                key, pressed: true, ..
            } => {
                use bl::VirtualKeyCode as K;
                match key {
                    K::Back => {
                        todo!();
                    },
                    K::Return | K::NumpadEnter => {
                        todo!();
                    },
                    K::Right => {
                        todo!();
                    },
                    K::Left => {
                        todo!();
                    },
                    K::Up => {
                        todo!();
                    },
                    K::Down => {
                        todo!();
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }

    pub fn load_text(&mut self, text: &str) {}

    pub fn get_chars(&mut self) -> impl Iterator<Item = char> {
        self.to_string().chars().collect::<Vec<char>>().into_iter()
    }

    pub fn to_string(&self) -> String {
        todo!();
    }

    pub fn draw(&mut self, data: &mut TickData) {
        while let Some(n) = self.recv.next_notif() {
            println!("Notif: {:?}", n);
        }
    }
}
