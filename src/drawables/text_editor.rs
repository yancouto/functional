use crate::{gamestates::base::TickData, math::*};
use bracket_lib::prelude as bl;
use std::time::Duration;

#[derive(Debug)]
pub struct TextEditor {
    pos: Pos,
    cursor: Pos,
    size: Size,
    text: Vec<Vec<char>>,
    cursor_blink_rate: Duration,
}

impl TextEditor {
    pub fn new(pos: Pos, size: Size) -> Self {
        Self {
            pos,
            cursor: Pos { i: 0, j: 0 },
            size,
            text: vec![vec![' '; size.w as usize]; size.h as usize],
            cursor_blink_rate: Duration::from_secs_f32(0.5),
        }
    }

    pub fn on_event(&mut self, event: &bl::BEvent) {
        match event {
            bl::BEvent::Character { c } => {
                if !c.is_control() {
                    let cu = &self.cursor;
                    self.text[cu.i as usize][cu.j as usize] = *c;
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

    pub fn load_text(&mut self, text: &str) {
        let size = &self.size;
        self.text = text
            .split('\n')
            .map(|line| {
                let mut line: Vec<char> = line.chars().collect();
                line.resize_with(size.w as usize, || ' ');
                line
            })
            .collect();
        self.text
            .resize_with(size.h as usize, || vec![' '; size.w as usize]);
    }

    pub fn to_string(&self) -> String {
        self.text
            .iter()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn get_chars(&self) -> impl Iterator<Item = char> {
        self.text.clone().into_iter().flatten()
    }

    pub fn draw(&mut self, data: &mut TickData) {
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
            data.console.print(
                self.pos.j,
                i as i32 + self.pos.i,
                &line.iter().collect::<String>(),
            )
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

impl TextEditor {
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
}
