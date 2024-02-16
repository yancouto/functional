use std::{convert::TryFrom, iter::FromIterator, path::PathBuf, time::Duration};

#[cfg(feature = "clipboard")]
use clipboard::{ClipboardContext, ClipboardProvider};
use xi_core_lib::rpc::{
    EditCommand, EditNotification, EditRequest, GestureType, LineRange, SelectionGranularity
};

use super::{black, TextEditor, TextEditorInner};
use crate::{gamestates::base::TickData, math::*, prelude::*, text_editor::interface::*};

#[derive(Debug, Default, Clone)]
struct Line {
    text: String,
}

impl From<InsLine> for Line {
    fn from(line: InsLine) -> Self { Self { text: line.text } }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct XiEditor {
    title:             String,
    rect:              Rect,
    cursor:            Pos,
    selections:        Vec<(Pos, Pos)>,
    text:              Vec<Line>,
    cursor_blink_rate: Duration,
    cursor_enabled:    bool,
    view_id:           xi_core_lib::ViewId,
    send:              ClientMessageSender,
    recv:              ServerMessageReceiver,
    #[cfg(feature = "clipboard")]
    #[derivative(Debug = "ignore")]
    clipboard:         Option<ClipboardContext>,
    backup_clipboard:  String,
}

const HARDCODED_MAIN_CONSOLE: usize = 0;

impl XiEditor {
    fn init_view(&self) {
        let lines = LineRange {
            first: 0,
            last:  self.rect.size.h as i64,
        };
        self.send_notif(EditNotification::Scroll(lines.clone()));
        self.send_notif(EditNotification::RequestLines(lines));
    }
}

impl TextEditor for XiEditor {
    fn new(title: String, rect: Rect, initial_text: String) -> Self {
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

        #[cfg(feature = "clipboard")]
        #[cfg(debug_assertions)]
        ClipboardContext::new().expect("Failed to get clipboard provider");

        let this = Self {
            title,
            rect,
            cursor: Pos { i: 0, j: 0 },
            text: vec![],
            cursor_blink_rate: Duration::from_secs_f32(0.5),
            cursor_enabled: true,
            send,
            recv,
            view_id: resp.0,
            selections: vec![],
            #[cfg(feature = "clipboard")]
            clipboard: ClipboardContext::new().ok(),
            backup_clipboard: String::new(),
        };
        if !initial_text.is_empty() {
            this.send_notif(EditNotification::Paste {
                chars: textwrap::fill(&initial_text, rect.size.w as usize),
            });
        }
        this.init_view();
        this
    }
}

impl TextEditorInner for XiEditor {
    fn on_event(&mut self, event: &bl::BEvent, input: &bl::Input) {
        use bl::VirtualKeyCode as K;
        let pressed = input.key_pressed_set();
        let shift = pressed.contains(&K::LShift) || pressed.contains(&K::RShift);
        let ctrl = pressed.contains(&K::LControl) || pressed.contains(&K::RControl);
        match event {
            bl::BEvent::Character { c } =>
                if !c.is_control()
                    && self
                        .text
                        .get(self.cursor.i as usize)
                        .map(|l| l.text.len())
                        .debug_unwrap_or(0)
                        < self.rect.size.w as usize
                {
                    self.send_notif(EditNotification::Insert {
                        chars: String::from_iter(&[*c]),
                    });
                },
            bl::BEvent::KeyboardInput {
                key, pressed: true, ..
            } => {
                let notif = match key {
                    K::Back => Some(EditNotification::DeleteBackward),
                    K::Return | K::NumpadEnter
                        if !ctrl && self.text.len() < self.rect.size.h as usize =>
                        Some(EditNotification::InsertNewline),
                    K::Right => Some(if shift {
                        EditNotification::MoveRightAndModifySelection
                    } else {
                        EditNotification::MoveRight
                    }),
                    K::Left => Some(if shift {
                        EditNotification::MoveLeftAndModifySelection
                    } else {
                        EditNotification::MoveLeft
                    }),
                    K::Up => Some(if shift {
                        EditNotification::MoveUpAndModifySelection
                    } else {
                        EditNotification::MoveUp
                    }),
                    K::Down => Some(if shift {
                        EditNotification::MoveDownAndModifySelection
                    } else {
                        EditNotification::MoveDown
                    }),
                    K::A if ctrl => Some(EditNotification::SelectAll),
                    K::Z if ctrl && !shift => Some(EditNotification::Undo),
                    K::Z if ctrl && shift => Some(EditNotification::Redo),
                    K::V if ctrl => Some(EditNotification::Paste {
                        chars: self.read_clipboard(),
                    }),
                    K::Tab => Some(EditNotification::InsertTab),
                    K::Delete => Some(EditNotification::DeleteForward),
                    _ => None,
                };
                if let Some(notif) = notif {
                    self.send_notif(notif);
                }
                match key {
                    K::C | K::X if ctrl => {
                        let ans = self.send.send_request_block(CoreRequest::Edit(EditCommand {
                            view_id: self.view_id,
                            cmd:     if *key == K::C {
                                EditRequest::Copy
                            } else {
                                EditRequest::Cut
                            },
                        }));
                        if let Ok(Some(txt)) = ans.map(|v| v.as_str().map(|s| s.to_string())) {
                            self.write_clipboard(txt);
                        }
                    },
                    _ => {},
                }
            },
            bl::BEvent::CursorMoved { .. } =>
                if input.is_mouse_button_pressed(0) {
                    let mut mouse =
                        Pos::from_xy(input.mouse_tile_pos(HARDCODED_MAIN_CONSOLE)) - self.rect.pos;
                    mouse.i = mouse.i.max(0);
                    mouse.j = mouse.j.max(0);
                    self.send_notif(EditNotification::Gesture {
                        line: mouse.i as u64,
                        col:  mouse.j as u64,
                        ty:   GestureType::Drag,
                    });
                },
            bl::BEvent::MouseClick { button, pressed } if *pressed && *button == 0 => {
                let mouse =
                    Pos::from_xy(input.mouse_tile_pos(HARDCODED_MAIN_CONSOLE)) - self.rect.pos;
                self.send_notif(EditNotification::Gesture {
                    line: mouse.i as u64,
                    col:  mouse.j as u64,
                    ty:   GestureType::Select {
                        granularity: SelectionGranularity::Point,
                        multi:       false,
                    },
                });
            },
            _ => {},
        }
    }

    fn load_file(&mut self, path: Option<PathBuf>) -> std::io::Result<()> {
        self.send.send_notification(CoreNotification::CloseView {
            view_id: self.view_id,
        });
        let resp = serde_json::from_value::<ViewId>(
            self.send
                .send_request_block(CoreRequest::NewView {
                    file_path: if let Some(inner_path) = path {
                        if inner_path.exists() {
                            Some(inner_path.to_string_lossy().to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    },
                })
                .unwrap(),
        )
        .unwrap();
        log::info!("Changing view id from {} to {}", self.view_id, resp.0);
        self.view_id = resp.0;
        self.text = vec![];
        self.init_view();
        Ok(())
    }

    fn to_string(&self) -> String {
        self.text
            .iter()
            .map(|l| l.text.clone())
            .collect::<Vec<_>>()
            .join("")
    }

    fn draw(&mut self, data: &mut TickData) {
        while let Some(n) = self.recv.next_notif() {
            self.handle_notif(n);
        }
        let cursor_on = self.cursor_enabled
            && (data.time.div_duration_f32(self.cursor_blink_rate) as i32 % 2) == 0;
        data.title_box(
            &self.title,
            Rect::new(
                self.rect.pos.i - 2,
                self.rect.pos.j - 1,
                self.rect.size.w + 2,
                self.rect.size.h + 3,
            ),
        );

        self.text
            .iter()
            .take(self.rect.size.h as usize)
            .enumerate()
            .for_each(|(i, line)| {
                data.console.print(
                    self.rect.pos.j,
                    i as i32 + self.rect.pos.i,
                    &line.text[0..(self.rect.size.w as usize).min(line.text.len())],
                )
            });

        for select in &self.selections {
            let mut init_j = select.0.j;
            for i in select.0.i..=select.1.i {
                let end_j = if i == select.1.i {
                    select.1.j
                } else {
                    self.text[i as usize].text.len() as i32 - 1
                };
                for j in init_j..=end_j {
                    data.console.set_bg(
                        j + self.rect.pos.j,
                        i + self.rect.pos.i,
                        bl::RGBA::from_f32(1., 1., 1., 0.2),
                    )
                }
                init_j = 0;
            }
        }

        data.console.set_bg(
            self.cursor.j + self.rect.pos.j,
            self.cursor.i + self.rect.pos.i,
            if cursor_on {
                bl::RGBA::from_f32(1., 1., 1., 0.5)
            } else {
                black()
            },
        );
    }

    fn rect(&self) -> &Rect { &self.rect }

    fn set_cursor(&mut self, enable: bool) { self.cursor_enabled = enable; }

    fn load_string(&mut self, str: String) {
        self.load_file(None).debug_unwrap();
        self.send_notif(EditNotification::Paste { chars: str });
    }
}

impl XiEditor {
    fn read_clipboard(&mut self) -> String {
        #[cfg(feature = "clipboard")]
        {
            self.clipboard
                .as_mut()
                .and_then(|w| w.get_contents().ok())
                .unwrap_or_else(|| self.backup_clipboard.clone())
        }
        #[cfg(not(feature = "clipboard"))]
        "".to_string()
    }

    fn write_clipboard(&mut self, str: String) {
        self.backup_clipboard = str.clone();
        #[cfg(feature = "clipboard")]
        if let Some(w) = self.clipboard.as_mut() {
            w.set_contents(str).debug_unwrap();
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
                UpdateOp::Skip { n } => old_text.advance_by(n).debug_unwrap(),
                UpdateOp::Invalidate { n } => new_lines.extend(vec![Line::default(); n]),
                UpdateOp::Update { .. } => unreachable!(),
                UpdateOp::Ins { n: _, lines } =>
                    new_lines.extend(lines.into_iter().map(Line::from)),
            }
        }
        std::mem::drop(old_text);
        self.text = new_lines;
        for annotation in update.annotations {
            match annotation {
                UpdateAnnotation::Selection {
                    ranges,
                    n: _,
                    payloads: _,
                } => {
                    self.selections = ranges
                        .into_iter()
                        .map(|(i1, j1, i2, j2)| {
                            (
                                Pos::new(i1 as i32, j1 as i32),
                                Pos::new(i2 as i32, j2 as i32),
                            )
                        })
                        .collect();
                },
            }
        }
    }

    fn check_view_id(&self, view_id: String) -> bool {
        match ViewId::try_from(view_id).map(|x| x.0) {
            Ok(id) if id == self.view_id => true,
            _ => false,
        }
    }

    fn handle_notif(&mut self, notif: ServerNotification) {
        match notif {
            ServerNotification::ScrollTo { view_id, line, col } =>
                if self.check_view_id(view_id) {
                    self.cursor = Pos::new(line as i32, col as i32);
                },
            ServerNotification::Update { view_id, update } =>
                if self.check_view_id(view_id) {
                    self.update(update);
                },
            ServerNotification::ConfigChanged { view_id, changes } => {
                if self.check_view_id(view_id) {
                    log::info!("Configs: {:?}", changes);
                }
            },
            ServerNotification::AvailablePlugins { .. } => {},
            ServerNotification::AvailableLanguages { .. } => {},
            ServerNotification::AvailableThemes { .. } => {},
            ServerNotification::LanguageChanged { .. } => {},
        }
    }
}
