use std::{convert::TryFrom, iter::FromIterator, time::Duration};

use serde::Deserialize;
use xi_core_lib::rpc::{EditCommand, EditNotification, EditRequest};

use crate::{gamestates::base::TickData, math::*, prelude::*, text_editor::interface::*};

#[derive(Debug, Default)]
struct Line {
    text: String,
}

impl From<InsLine> for Line {
    fn from(line: InsLine) -> Self { Self { text: line.text } }
}

#[derive(Debug)]
pub struct XiEditor {
    pos:               Pos,
    cursor:            Pos,
    size:              Size,
    text:              Vec<Line>,
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
            text: vec![],
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

    fn send_notif(&self, cmd: EditNotification) {
        self.send.send_notification(self.edit_notif(cmd));
    }

    pub fn on_event(&mut self, event: &bl::BEvent) {
        match event {
            bl::BEvent::Character { c } =>
                if !c.is_control() {
                    self.send_notif(EditNotification::Insert {
                        chars: String::from_iter(&[*c]),
                    });
                },
            bl::BEvent::KeyboardInput {
                key, pressed: true, ..
            } => {
                use bl::VirtualKeyCode as K;
                match key {
                    K::Back => {
                        self.send_notif(EditNotification::DeleteBackward);
                    },
                    K::Return | K::NumpadEnter => {
                        self.send_notif(EditNotification::InsertNewline);
                    },
                    K::Right => {
                        self.send_notif(EditNotification::MoveRight);
                    },
                    K::Left => {
                        self.send_notif(EditNotification::MoveLeft);
                    },
                    K::Up => {
                        self.send_notif(EditNotification::MoveUp);
                    },
                    K::Down => {
                        self.send_notif(EditNotification::MoveDown);
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

    fn update(&mut self, update: Update) {
        // Can be made more efficient, though probably not necessary
        let mut new_lines = Vec::with_capacity(self.text.len());
        let mut old_text = self.text.drain(..);
        for update in update.ops {
            match update {
                UpdateOp::Copy { n, ln } =>
                    for _ in 0..n {
                        if let Some(x) = old_text.next() {
                            new_lines.push(x);
                        }
                    },
                UpdateOp::Skip { n } => old_text.advance_by(n).unwrap(),
                UpdateOp::Invalidate { n } => unreachable!(),
                UpdateOp::Update { n, lines } => unreachable!(),
                UpdateOp::Ins { n, lines } => new_lines.extend(lines.into_iter().map(Line::from)),
            }
        }
        std::mem::drop(old_text);
        self.text = new_lines;
        println!("New lines: {:?}", self.text);
    }

    fn handle_notif(&mut self, notif: ServerNotification) {
        match notif {
            ServerNotification::ScrollTo { view_id, line, col } => {
                self.cursor = Pos::new(line as i32, col as i32);
            },
            ServerNotification::Update { view_id, update } => {
                self.update(update);
            },
            ServerNotification::ConfigChanged { .. } => {},
            ServerNotification::AvailablePlugins { .. } => {},
            ServerNotification::AvailableLanguages { .. } => {},
            ServerNotification::AvailableThemes { .. } => {},
            ServerNotification::LanguageChanged { .. } => {},
        }
    }

    pub fn draw(&mut self, data: &mut TickData) {
        while let Some(n) = self.recv.next_notif() {
            self.handle_notif(n);
        }
        let cursor_on = (data.time.div_duration_f32(self.cursor_blink_rate) as i32 % 2) == 0;
        data.draw_box(
            "Text editor",
            Rect::new(
                self.pos.i - 2,
                self.pos.j - 1,
                self.size.w + 2,
                self.size.h + 3,
            ),
        );

        self.text.iter().enumerate().for_each(|(i, line)| {
            data.console
                .print(self.pos.j, i as i32 + self.pos.i, &line.text)
        });
        if cursor_on {
            data.console.set_bg(
                self.cursor.j + self.pos.j,
                self.cursor.i + self.pos.i,
                bl::RGBA::from_f32(1., 1., 1., 0.5),
            );
        }
    }
}
