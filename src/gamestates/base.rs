use bracket_lib::prelude as bl;

#[derive(Debug)]
pub struct GameStateManager {
    cur_gs: Box<dyn GameState>,
}

impl GameStateManager {
    pub fn new(first: Box<dyn GameState>) -> Self {
        Self { cur_gs: first }
    }

    pub fn tick(&mut self, ctx: &mut bl::BTerm) {
        self.cur_gs.tick(ctx);
    }
}

pub enum GameStateEvent {
    None,
}

pub trait GameState: std::fmt::Debug {
    fn tick(&mut self, ctx: &mut bl::BTerm) -> GameStateEvent;
}
