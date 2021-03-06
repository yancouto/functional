use bracket_lib::prelude as bl;

pub enum GameStateEvent {
    None,
}
pub trait GameState {
    fn tick(&mut self, ctx: &mut bl::BTerm) -> GameStateEvent;
}
