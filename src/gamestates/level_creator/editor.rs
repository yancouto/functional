use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::super::base::*;
use crate::prelude::*;

const WORKSHOP_FILE: &str = "workshop.yaml";
#[derive(Debug, Default, Serialize, Deserialize)]
struct WorkshopConfig {
    published_id: Option<u64>,
}

#[derive(Debug)]
pub struct EditorState {
    root: PathBuf,
}

impl EditorState {
    pub fn new(root: PathBuf) -> Self { Self { root } }

    fn workshop_file(&self) -> PathBuf { self.root.join(WORKSHOP_FILE) }

    fn read_config(&self) -> WorkshopConfig {
        match std::fs::File::open(self.workshop_file()).map(|f| serde_yaml::from_reader(f)) {
            Ok(Ok(config)) => config,
            err @ _ => {
                log::warn!("Failed to read workshop config! {:?}", err);
                debug_assert!(false);
                WorkshopConfig::default()
            },
        }
    }

    fn write_config(&self, config: &WorkshopConfig) {
        match std::fs::File::create(self.workshop_file()).map(|f| serde_yaml::to_writer(f, config))
        {
            Ok(Ok(_)) => {},
            err @ _ => {
                log::warn!("Failed to write workshop config! {:?}", err);
                debug_assert!(false);
            },
        }
    }
}

impl GameState for EditorState {
    fn name(&self) -> &'static str { "LevelEditor" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent { todo!() }
}
