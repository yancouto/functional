mod gamestates;

use bracket_lib::prelude as bl;

struct MainState {
    cur_gs: Box<dyn gamestates::base::GameState>,
}

impl bl::GameState for MainState {
    fn tick(&mut self, ctx: &mut bl::BTerm) {
        self.cur_gs.tick(ctx);
    }
}

fn main() -> bl::BError {
    let ctx = bl::BTermBuilder::simple80x50()
        .with_title("functional")
        .build()?;
    let gs = MainState {
        cur_gs: Box::new(gamestates::intro::IntroState::new()),
    };
    bl::main_loop(ctx, gs)
}
