#![feature(div_duration)]
#![feature(box_syntax)]
#![feature(try_blocks)]
#![feature(try_trait)]
#![feature(generators, generator_trait)]
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate savefile_derive;

mod drawables;
mod gamestates;
mod interpreter;
mod levels;
mod math;
mod save_system;
mod utils;
use structopt::StructOpt;

use bracket_lib::prelude as bl;
use simplelog::*;
use std::rc::Rc;

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

// Things useful for everyone
mod prelude {
    pub const W: i32 = 130;
    pub const H: i32 = 80;
    pub use crate::math::Pos;
    pub use std::rc::Rc;
}

use prelude::*;

fn main() -> bl::BError {
    let mut log_file = app_dirs::app_root(app_dirs::AppDataType::UserCache, &save_system::APP_INFO)
        .expect("Failed to get app root");
    log_file.push("debug.log");
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            std::fs::File::create(log_file.clone()).expect("Failed to create debug file"),
        ),
    ])
    .expect("Failed to set up logger.");
    log::info!("Writing debug logs to {:?}", log_file);

    let opt = Opt::from_args();
    let ctx = bl::BTermBuilder::simple(W, H)?
        .with_title("functional")
        .build()?;
    let gs = MainState {
        manager: gamestates::base::GameStateManager::new(if opt.skip_intro {
            Box::new(gamestates::level_selection::LevelSelectionState::new(
                Rc::new(save_system::load_profile(DEFAULT_PROFILE)),
            ))
        } else {
            Box::new(gamestates::intro::IntroState::new())
        }),
    };
    bl::main_loop(ctx, gs)
}
