#![feature(div_duration)]
#![feature(box_syntax)]
#![feature(try_blocks)]
#![feature(try_trait)]
#![feature(generators, generator_trait)]
#![feature(iter_advance_by)]
#![feature(map_first_last)]
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
mod text_editor;
mod utils;

use simplelog::*;
use structopt::StructOpt;

struct MainState {
    manager: gamestates::base::GameStateManager,
}

impl bl::GameState for MainState {
    fn tick(&mut self, ctx: &mut bl::BTerm) { self.manager.tick(ctx); }
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
    pub use std::rc::Rc;

    pub use bracket_lib::prelude as bl;
    pub use vec1::{vec1, Vec1};

    pub use crate::{
        math::{Pos, Rect}, utils::debug_asserts::DebugUnwrap
    };
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
            gamestates::save_loader::SaveLoaderState::try_load(DEFAULT_PROFILE.to_string())
        } else {
            box gamestates::intro::IntroState::new()
        }),
    };
    bl::main_loop(ctx, gs)
}
