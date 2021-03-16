use std::time::Duration;

use super::base::{GameState, GameStateEvent, TickData};
use crate::levels::Level;
use bracket_lib::prelude as bl;

#[derive(Debug)]
struct Cursor {
    i: usize,
    j: usize,
}

#[derive(Debug)]
struct Dimension {
    w: usize,
    h: usize,
}

// TODO: Split editor itself to different component.
#[derive(Debug)]
pub struct EditorState<'a> {
    size: Dimension,
    text: Vec<Vec<char>>,
    cursor: Cursor,
    cursor_blink_rate: Duration,
    time: Duration,
    level: &'a Level,
}

impl EditorState<'static> {
    pub fn new(level: &'static Level) -> Self {
        let size = Dimension { w: 20, h: 8 };
        Self {
            text: vec![vec![' '; size.w]; size.h],
            cursor: Cursor { i: 0, j: 0 },
            cursor_blink_rate: Duration::from_secs_f32(0.5),
            time: Duration::from_secs(0),
            size,
            level,
        }
    }
}

impl<'a> EditorState<'a> {
    fn move_cursor_right(&mut self) -> bool {
        let c = &mut self.cursor;
        if c.j == self.size.w - 1 {
            if c.i == self.size.h - 1 {
                // do nothing, we're on the last char
                false
            } else {
                c.j = 0;
                c.i += 1;
                true
            }
        } else {
            c.j += 1;
            true
        }
    }

    fn move_cursor_left(&mut self) -> bool {
        let c = &mut self.cursor;
        if c.j == 0 {
            if c.i == 0 {
                false
            } else {
                c.i -= 1;
                c.j = self.size.w - 1;
                true
            }
        } else {
            c.j -= 1;
            true
        }
    }

    fn get_text(&self) -> String {
        self.text
            .clone()
            .into_iter()
            .map(|v| v.into_iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

const EDITOR_I: i32 = 20;
const EDITOR_J: i32 = 2;

impl<'a> GameState for EditorState<'a> {
    fn name(&self) -> &'static str {
        "Editor"
    }

    fn tick(&mut self, data: TickData) -> GameStateEvent {
        let cursor_on = ((data.time.as_millis() / self.cursor_blink_rate.as_millis()) % 2) == 0;
        let mut c = data.console;

        c.print(2, 0, "Editor");
        let mut text = bl::TextBuilder::empty();
        text.line_wrap(&self.level.name)
            .ln()
            .ln()
            .line_wrap(&self.level.description)
            .reset();

        let mut block = bl::TextBlock::new(1, 2, 30, 20);
        block.print(&text).unwrap();
        block.render(&mut c);

        self.text.iter().enumerate().for_each(|(i, line)| {
            c.print(
                EDITOR_J,
                i as i32 + EDITOR_I,
                &line.iter().collect::<String>(),
            )
        });

        if cursor_on {
            c.set_bg(
                self.cursor.j as i32 + EDITOR_J,
                self.cursor.i as i32 + EDITOR_I,
                bl::RGBA::from_f32(1., 1., 1., 0.5),
            );
        }
        GameStateEvent::None
    }

    fn on_event(&mut self, event: bl::BEvent) {
        match event {
            bl::BEvent::Character { c } => {
                if !c.is_control() {
                    let cu = &self.cursor;
                    self.text[cu.i][cu.j] = c;
                    self.move_cursor_right();
                }
            }
            bl::BEvent::KeyboardInput {
                key, pressed: true, ..
            } => {
                use bl::VirtualKeyCode as K;
                match key {
                    K::Back => {
                        if self.move_cursor_left() {
                            self.text[self.cursor.i][self.cursor.j] = ' ';
                        };
                    }
                    K::Return | K::NumpadEnter => {
                        if self.cursor.i < self.size.h - 1 {
                            self.cursor.i += 1;
                            self.cursor.j = 0;
                        }
                    }
                    K::Right => {
                        self.move_cursor_right();
                    }
                    K::Left => {
                        self.move_cursor_left();
                    }
                    K::Up => {
                        if self.cursor.i > 0 {
                            self.cursor.i -= 1;
                        }
                    }
                    K::Down => {
                        if self.cursor.i < self.size.h - 1 {
                            self.cursor.i += 1;
                        }
                    }
                    K::F1 => {
                        println!("Is ok? {}", self.level.test(&self.get_text()));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
