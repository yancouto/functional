use std::time::Duration;

use super::base::{GameState, GameStateEvent};
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

#[derive(Debug)]
pub struct EditorState {
    size: Dimension,
    text: Vec<Vec<char>>,
    cursor: Cursor,
    cursor_blink_rate: Duration,
    time: Duration,
}

fn with_current_console<F>(ctx: &mut bl::BTerm, f: F)
where
    F: Fn(&mut Box<dyn bl::Console>) -> (),
{
    f(&mut bl::BACKEND_INTERNAL.lock().consoles[ctx.active_console].console);
}

impl EditorState {
    pub fn new() -> Self {
        let size = Dimension { w: 20, h: 8 };
        Self {
            text: vec![vec![' '; size.w]; size.h],
            cursor: Cursor { i: 0, j: 0 },
            cursor_blink_rate: Duration::from_secs_f32(0.5),
            time: Duration::from_secs(0),
            size,
        }
    }

    fn print(&mut self, mut ctx: &mut bl::BTerm) {
        self.time += Duration::from_secs_f32(ctx.frame_time_ms / 1000.);
        let cursor_on = ((self.time.as_millis() / self.cursor_blink_rate.as_millis()) % 2) == 0;
        with_current_console(&mut ctx, |c| {
            self.text
                .iter()
                .enumerate()
                .for_each(|(i, line)| c.print(0, i as i32, &line.iter().collect::<String>()));
            if cursor_on {
                c.set_bg(
                    self.cursor.j as i32,
                    self.cursor.i as i32,
                    bl::RGBA::from_f32(1., 1., 1., 0.5),
                );
            }
        });
    }

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

impl GameState for EditorState {
    fn name(&self) -> &'static str {
        "Editor"
    }

    fn tick(&mut self, ctx: &mut bl::BTerm) -> GameStateEvent {
        self.print(ctx);
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
            bl::BEvent::KeyboardInput { key, pressed, .. } => {
                if !pressed {
                    return;
                }
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
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
