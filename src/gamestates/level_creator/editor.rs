use std::path::PathBuf;

use super::super::base::*;
use crate::prelude::*;
const WORKSHOP_CONFIG_VERSION: u32 = 0;
const WORKSHOP_FILE: &str = "workshop.data";

#[derive(Savefile, Default)]
struct WorkshopConfig {
    published_id: Option<u64>,
}

#[derive(Debug)]
pub struct EditorState {
    root: PathBuf,
}

impl EditorState {
    pub fn new(root: PathBuf) -> Self { Self { root } }

    fn read_config(&self) -> WorkshopConfig {
        self.root
            .join(WORKSHOP_FILE)
            .to_str()
            .and_then(|filename| savefile::load_file(filename, WORKSHOP_CONFIG_VERSION).ok())
            .debug_unwrap_or(WorkshopConfig::default())
    }

    fn write_config(&self, config: &WorkshopConfig) {
        self.root
            .join(WORKSHOP_FILE)
            .to_str()
            .and_then(|name| savefile::save_file(name, WORKSHOP_CONFIG_VERSION, config).ok())
            .debug_unwrap();
    }
}

impl GameState for EditorState {
    fn name(&self) -> &'static str { "LevelEditor" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent { todo!() }
}
