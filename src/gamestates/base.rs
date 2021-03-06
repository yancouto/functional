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
        ctx.cls();
        match self.cur_gs.tick(ctx) {
            GameStateEvent::None => {}
            GameStateEvent::Switch(new) => {
                self.cur_gs = new;
            }
        }
    }
}

pub enum GameStateEvent {
    None,
    Switch(Box<dyn GameState>),
}

pub trait GameState: std::fmt::Debug {
    fn tick(&mut self, ctx: &mut bl::BTerm) -> GameStateEvent;
}
