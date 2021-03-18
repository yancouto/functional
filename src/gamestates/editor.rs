use crate::math::{Pos, Rect, Size};
use std::time::Duration;

use super::base::{GameState, GameStateEvent, TickData};
use crate::levels::Level;
use bracket_lib::prelude as bl;

// TODO: Split editor itself to different component.
#[derive(Debug)]
pub struct EditorState<'a> {
    size: Size,
    text: Vec<Vec<char>>,
    cursor: Pos,
    cursor_blink_rate: Duration,
    time: Duration,
    level: &'a Level,
}

impl EditorState<'static> {
    pub fn new(level: &'static Level) -> Self {
        let size = Size { w: 20, h: 8 };
        Self {
            text: vec![vec![' '; size.w as usize]; size.h as usize],
            cursor: Pos { i: 0, j: 0 },
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

    fn get_text(&self) -> impl Iterator<Item = char> {
        self.text.clone().into_iter().flatten()
    }
}

const EDITOR_I: i32 = 23;
const EDITOR_J: i32 = 2;

impl<'a> GameState for EditorState<'a> {
    fn name(&self) -> &'static str {
        "Editor"
    }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        let cursor_on = ((data.time.as_millis() / self.cursor_blink_rate.as_millis()) % 2) == 0;

        data.text_box(
            &self.level.name,
            &self.level.description,
            Rect::new(1, 0, 50, 20),
        );

        data.draw_box(
            "Text editor",
            Rect::new(EDITOR_I - 2, EDITOR_J - 1, self.size.w + 2, self.size.h + 3),
        );

        self.text.iter().enumerate().for_each(|(i, line)| {
            data.console.print(
                EDITOR_J,
                i as i32 + EDITOR_I,
                &line.iter().collect::<String>(),
            )
        });

        if cursor_on {
            data.console.set_bg(
                self.cursor.j as i32 + EDITOR_J,
                self.cursor.i as i32 + EDITOR_I,
                bl::RGBA::from_f32(1., 1., 1., 0.5),
            );
        }

        if data.button("Run", Pos::new(47, 2)) {
            println!("Is ok? {}", self.level.test(self.get_text()));
        }

        GameStateEvent::None
    }

    fn on_event(&mut self, event: bl::BEvent) {
        match event {
            bl::BEvent::Character { c } => {
                if !c.is_control() {
                    let cu = &self.cursor;
                    self.text[cu.i as usize][cu.j as usize] = c;
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
                            self.text[self.cursor.i as usize][self.cursor.j as usize] = ' ';
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
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
