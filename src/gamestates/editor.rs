use super::base::{GameState, GameStateEvent};
use bracket_lib::prelude as bl;

#[derive(Debug)]
pub struct EditorState {}

impl EditorState {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameState for EditorState {
    fn tick(&mut self, ctx: &mut bl::BTerm) -> GameStateEvent {
        ctx.print(10, 10, "hello world");
        GameStateEvent::None
    }
}
