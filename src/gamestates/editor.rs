use super::base::{GameState, GameStateEvent};
use bracket_lib::prelude as bl;

#[derive(Debug)]
pub struct EditorState {
    text: Vec<Vec<u8>>,
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
            text: vec![vec![' ' as u8; 8]; 8],
            cursor: (0, 0),
        }
    }

    fn print(&mut self, mut ctx: &mut bl::BTerm) {
        let mut builder = bl::TextBuilder::empty();
        self.text[1][1] = 'x' as u8;
        self.text.iter().for_each(|line| {
            builder.append(&String::from_utf8_lossy(line));
        });
        builder.reset();
        let mut block = bl::TextBlock::new(10, 10, 8, 8);
        block.print(&builder).unwrap();
        with_current_console(&mut ctx, |c| block.render(c));
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
