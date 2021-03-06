mod gamestates;

use bracket_lib::prelude as bl;

struct MainState {
    manager: gamestates::base::GameStateManager,
}

impl bl::GameState for MainState {
    fn tick(&mut self, ctx: &mut bl::BTerm) {
        self.manager.tick(ctx);
    }
}

fn main() -> bl::BError {
    let ctx = bl::BTermBuilder::simple80x50()
        .with_title("functional")
        .build()?;
    let gs = MainState {
        manager: gamestates::base::GameStateManager::new(Box::new(
            gamestates::intro::IntroState::new(),
        )),
    };
    bl::main_loop(ctx, gs)
}
