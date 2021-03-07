use super::base::{GameState, GameStateEvent};
use bracket_lib::prelude as bl;

#[derive(Debug)]
pub struct EditorState {
    text: Vec<Vec<char>>,
    cursor: (u8, u8),
}

fn with_current_console<F>(ctx: &mut bl::BTerm, f: F)
where
    F: Fn(&mut Box<dyn bl::Console>) -> (),
{
    f(&mut bl::BACKEND_INTERNAL.lock().consoles[ctx.active_console].console);
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            text: vec![vec![' '; 8]; 8],
            cursor: (0, 0),
        }
    }

    fn print(&mut self, mut ctx: &mut bl::BTerm) {
        self.text[0][0] = 'x';
        self.text[1][1] = 'y';
        with_current_console(&mut ctx, |c| {
            self.text
                .iter()
                .enumerate()
                .for_each(|(i, line)| c.print(0, i as i32, &line.iter().collect::<String>()));
            c.set_bg(
                self.cursor.0.into(),
                self.cursor.1.into(),
                bl::RGBA::from_f32(1., 1., 1., 0.5),
            );
        });
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
}
