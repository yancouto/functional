#![feature(div_duration)]
#![feature(box_syntax)]
#![feature(try_blocks)]
#![feature(generators, generator_trait)]
#![feature(iter_advance_by)]
#![feature(map_first_last)]
#![feature(trait_alias)]
#![feature(backtrace)]
#![feature(assert_matches)]
#![feature(option_zip)]
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate savefile_derive;

#[allow(unused_imports)]
#[macro_use]
extern crate maplit;

mod drawables;
mod gamestates;
mod interpreter;
mod levels;
mod math;
mod prelude;
mod save_system;
mod text_editor;
mod utils;

use std::path::Path;

use simplelog::*;
use structopt::StructOpt;

#[cfg(feature = "steam")]
type MainThreadSteamClient = steamworks::SingleClient;

#[cfg(not(feature = "steam"))]
type MainThreadSteamClient = ();

struct MainState {
    manager: gamestates::base::GameStateManager,
    client:  Option<MainThreadSteamClient>,
}

impl bl::GameState for MainState {
    fn tick(&mut self, ctx: &mut bl::BTerm) {
        #[cfg(feature = "steam")]
        if let Some(c) = &self.client {
            c.run_callbacks();
        }
        self.manager.tick(ctx);
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "functional")]
struct Opt {
    /// Skip intro screen
    #[structopt(long)]
    skip_intro: bool,
    /// Run game connected to steam
    #[structopt(long)]
    steam:      bool,
    /// Run game connected to steam when developing it locally
    #[structopt(long)]
    steam_dev:  bool,
}

use prelude::*;

#[cfg(feature = "steam")]
const APP_ID: u32 = 1636730;

const ICON_DATA: &[u8] = include_bytes!("inlined_assets/icon.bmp");

fn maybe_load_icon() {
    let result = bl::BACKEND
        .lock()
        .context_wrapper
        .as_ref()
        .map(|wrapped_ctx| {
            bmp::from_reader(&mut ICON_DATA.clone()).map(|img| {
                let mut data =
                    Vec::with_capacity((img.get_height() * img.get_width() * 4) as usize);
                for (x, y) in img.coordinates() {
                    let bmp::Pixel { r, g, b } = img.get_pixel(x, y);
                    data.append(&mut vec![r, g, b, 255]);
                }
                winit::window::Icon::from_rgba(data, img.get_width(), img.get_height())
                    .map(|icon| wrapped_ctx.wc.window().set_window_icon(Some(icon)))
            })
        });
    match result {
        Some(Ok(Ok(()))) => {},
        err @ _ => log::warn!("Failed to set icon correctly: {:?}", err),
    }
}

fn main() -> bl::BError {
    let log_file = save_system::PROJECT_DIR.cache_dir().join("debug.log");
    println!("Writing debug logs to {:?}", log_file);
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

    let opt = Opt::from_args();
    let clients = if opt.steam || opt.steam_dev {
        #[cfg(feature = "steam")]
        {
            if opt.steam_dev {
                std::fs::write(Path::new("steam_appid.txt"), APP_ID.to_string())
                    .expect("Failed to write dev steam file.");
            }
            if steamworks::restart_app_if_necessary(steamworks::AppId(APP_ID)) {
                log::error!("Failed to connect to Steam, trying to relaunch.");
                return Ok(());
            }
            let ans = steamworks::Client::init().expect("Failed to initialise Steam.");
            log::info!("Successfully connected to Steam!");
            (Some(ans.0), Some(ans.1))
        }
        #[cfg(not(feature = "steam"))]
        panic!("Please build game with feature steam enabled!");
    } else {
        (None, None)
    };

    let ctx = bl::BTermBuilder::simple(W, H)?
        .with_title("functional")
        // We use the event queue
        .with_advanced_input(true)
        .with_tile_dimensions(12, 12)
        .build()?;
    maybe_load_icon();
    let first_state = || gamestates::profile_selection::try_load_default_profile();
    let gs = MainState {
        manager: gamestates::base::GameStateManager::new(
            if opt.skip_intro || cfg!(debug_assertions) {
                first_state()
            } else {
                box gamestates::intro::IntroState::new(first_state)
            },
            clients.0,
        ),
        client:  clients.1,
    };
    bl::main_loop(ctx, gs)
}
