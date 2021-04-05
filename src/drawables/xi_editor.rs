use std::{iter::FromIterator, time::Duration};

use xi_core_lib::rpc::{EditCommand, EditNotification};

use super::TextEditor;
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

impl TextEditor for XiEditor {
    fn new(pos: Pos, size: Size) -> Self {
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

    fn on_event(&mut self, event: &bl::BEvent) {
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

    fn load_text(&mut self, _text: &str) {}

    fn to_string(&self) -> String {
        todo!();
    }

    fn draw(&mut self, data: &mut TickData) {
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

impl XiEditor {
    fn edit_notif(&self, cmd: EditNotification) -> CoreNotification {
        CoreNotification::Edit(EditCommand {
            view_id: self.view_id,
            cmd,
        })
    }

    fn send_notif(&self, cmd: EditNotification) {
        self.send.send_notification(self.edit_notif(cmd));
    }

    fn update(&mut self, update: Update) {
        // Can be made more efficient, though probably not necessary
        let mut new_lines = Vec::with_capacity(self.text.len());
        let mut old_text = self.text.drain(..);
        for update in update.ops {
            match update {
                UpdateOp::Copy { n, ln: _ } =>
                    for _ in 0..n {
                        if let Some(x) = old_text.next() {
                            new_lines.push(x);
                        }
                    },
                UpdateOp::Skip { n } => old_text.advance_by(n).unwrap(),
                UpdateOp::Invalidate { .. } => unreachable!(),
                UpdateOp::Update { .. } => unreachable!(),
                UpdateOp::Ins { n: _, lines } =>
                    new_lines.extend(lines.into_iter().map(Line::from)),
            }
        }
        std::mem::drop(old_text);
        self.text = new_lines;
        println!("New lines: {:?}", self.text);
    }

    fn check_view_id(&self, view_id: String) {
        debug_assert!(
            match serde_json::from_str::<ViewId>(&view_id).map(|x| x.0) {
                Ok(id) if id == self.view_id => true,
                _ => false,
            },
            "Invalid view id!"
        );
    }

    fn handle_notif(&mut self, notif: ServerNotification) {
        match notif {
            ServerNotification::ScrollTo { view_id, line, col } => {
                self.check_view_id(view_id);
                self.cursor = Pos::new(line as i32, col as i32);
            },
            ServerNotification::Update { view_id, update } => {
                self.check_view_id(view_id);
                self.update(update);
            },
            ServerNotification::ConfigChanged { .. } => {},
            ServerNotification::AvailablePlugins { .. } => {},
            ServerNotification::AvailableLanguages { .. } => {},
            ServerNotification::AvailableThemes { .. } => {},
            ServerNotification::LanguageChanged { .. } => {},
        }
    }
}
