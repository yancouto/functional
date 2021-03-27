#![feature(div_duration)]
#![feature(box_syntax)]
#[macro_use]
extern crate lazy_static;

mod drawables;
mod gamestates;
mod interpreter;
mod levels;
mod math;
mod save_system;
use gamestates::level_selection;
use structopt::StructOpt;

use bracket_lib::prelude as bl;

struct MainState {
    manager: gamestates::base::GameStateManager,
}

impl bl::GameState for MainState {
    fn tick(&mut self, ctx: &mut bl::BTerm) {
        self.manager.tick(ctx);
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "functional")]
struct Opt {
    /// Skip intro screen
    #[structopt(long, short)]
    skip_intro: bool,
}

// TODO: profile selection
pub const DEFAULT_PROFILE: &str = "default";

fn main() -> bl::BError {
    let opt = Opt::from_args();
    let ctx = bl::BTermBuilder::simple80x50()
        .with_title("functional")
        .build()?;
    let gs = MainState {
        manager: gamestates::base::GameStateManager::new(if opt.skip_intro {
            Box::new(gamestates::level_selection::LevelSelectionState::new(
                DEFAULT_PROFILE,
            ))
        } else {
            Box::new(gamestates::intro::IntroState::new())
        }),
    };
    bl::main_loop(ctx, gs)
}
