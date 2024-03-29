use std::{convert::TryFrom, path::PathBuf, time::Duration};

use super::{TextEditor, TextEditorInner};
use crate::{gamestates::base::TickData, math::*, prelude::*};

#[derive(Debug)]
pub struct BasicTextEditor {
    title:             String,
    rect:              Rect,
    cursor:            Pos,
    text:              Vec1<Vec<char>>,
    cursor_blink_rate: Duration,
    cursor_enabled:    bool,
}

impl BasicTextEditor {
    fn load_text(&mut self, text: String) {
        let size = &self.rect.size;
        self.text = Vec1::try_from(
            text.split('\n')
                .map(|line| {
                    let mut line: Vec<char> = line.chars().collect();
                    line.truncate(size.w as usize);
                    line
                })
                .collect::<Vec<_>>(),
        )
        .unwrap_or(vec1![vec![]]);
        self.text.truncate(size.h as usize).debug_unwrap();

        for line in &mut self.text {
            while let Some(c) = line.last() {
                if c.is_ascii_whitespace() {
                    line.pop().debug_unwrap();
                } else {
                    break;
                }
            }
        }
        while self.text.len() > 1 && self.text.last().is_empty() {
            self.text.pop().debug_unwrap();
        }
    }
}

impl TextEditor for BasicTextEditor {
    fn new(title: String, rect: Rect, _initial_text: String) -> Self {
        Self {
            title,
            rect,
            cursor: Pos { i: 0, j: 0 },
            text: vec1![vec![]],
            cursor_blink_rate: Duration::from_secs_f32(0.5),
            cursor_enabled: true,
        }
    }
}

impl TextEditorInner for BasicTextEditor {
    fn on_event(&mut self, event: &bl::BEvent, _input: &bl::Input) {
        match event {
            bl::BEvent::Character { c } => {
                if !c.is_control() {
                    if self.line_len() < self.rect.size.w - 1 {
                        let j = self.cursor.j as usize;
                        self.line_mut().insert(j, *c);
                        self.cursor.j += 1;
                    } else {
                        // line is full
                    }
                }
            },
            bl::BEvent::KeyboardInput {
                key, pressed: true, ..
            } => {
                use bl::VirtualKeyCode as K;
                match key {
                    K::Back => {
                        if self.cursor.j == 0 {
                            // join lines
                            if self.cursor.i > 0 {
                                let new_i = self.cursor.i as usize - 1;
                                if self.line_len() + (self.text[new_i].len() as i32)
                                    < self.rect.size.w
                                {
                                    let mut second_line = self.text.remove(new_i + 1).unwrap();
                                    self.cursor.j = self.text[new_i].len() as i32;
                                    self.text[new_i].append(&mut second_line);
                                    self.cursor.i = new_i as i32;
                                } else {
                                    // cannot delete because lines are big
                                }
                            }
                        } else {
                            self.cursor.j -= 1; // will now be < length
                            let j = self.cursor.j as usize;
                            self.line_mut().remove(j);
                        }
                    },
                    K::Return | K::NumpadEnter => {
                        if (self.text.len() as i32) < self.rect.size.h {
                            let j = self.cursor.j as usize;
                            let rest = self.line_mut().split_off(j);
                            self.text.insert(self.cursor.i as usize + 1, rest);
                            self.cursor.i += 1;
                            self.cursor.j = 0;
                        } else {
                            // Cannot because there are too much lines
                        }
                    },
                    K::Right => {
                        self.move_cursor_right();
                    },
                    K::Left => {
                        self.move_cursor_left();
                    },
                    K::Up =>
                        if self.cursor.i > 0 {
                            self.cursor.i -= 1;
                            self.cursor.j = self.cursor.j.min(self.line_len());
                        },
                    K::Down =>
                        if self.cursor.i < self.text.len() as i32 - 1 {
                            self.cursor.i += 1;
                            self.cursor.j = self.cursor.j.min(self.line_len());
                        },
                    _ => {},
                }
            },
            _ => {},
        }
    }

    fn load_file(&mut self, path: Option<PathBuf>) -> std::io::Result<()> {
        let file_data = path.map(|path| std::fs::read(path)).unwrap_or(Ok(vec![]))?;
        self.load_text(String::from_utf8_lossy(&file_data).to_string());
        Ok(())
    }

    fn to_string(&self) -> String {
        self.text
            .iter()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn draw(&mut self, data: &mut TickData) {
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

        self.text.iter().enumerate().for_each(|(i, line)| {
            data.console.print(
                self.rect.pos.j,
                i as i32 + self.rect.pos.i,
                &line.iter().collect::<String>(),
            )
        });
        if cursor_on {
            data.console.set_bg(
                self.cursor.j + self.rect.pos.j,
                self.cursor.i + self.rect.pos.i,
                bl::RGBA::from_f32(1., 1., 1., 0.5),
            );
        }
    }

    fn rect(&self) -> &Rect { &self.rect }

    fn set_cursor(&mut self, enable: bool) { self.cursor_enabled = enable; }

    fn load_string(&mut self, str: String) { self.load_text(str); }
}

impl BasicTextEditor {
    fn line_mut(&mut self) -> &mut Vec<char> {
        unsafe { self.text.get_unchecked_mut(self.cursor.i as usize) }
    }

    fn line(&self) -> &Vec<char> { unsafe { self.text.get_unchecked(self.cursor.i as usize) } }

    fn line_len(&self) -> i32 { self.line().len() as i32 }

    fn move_cursor_right(&mut self) -> bool {
        let line_len = self.line_len();
        let c = &mut self.cursor;
        if c.j == line_len {
            if c.i == self.text.len() as i32 - 1 {
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
                self.cursor.j = self.line_len();
                true
            }
        } else {
            c.j -= 1;
            true
        }
    }
}
