use ears::{AudioController, Sound, SoundData, SoundError};
use enum_map::{enum_map, Enum, EnumMap};
use parking_lot::Mutex;

use crate::prelude::*;

#[derive(strum::Display, Debug, Enum, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
pub enum SFX {
    Back,
    Confirm,
    Select,
    Win,
    Wrong,
}

impl SFX {
    pub fn play(self) {
        let mut manager = MANAGER.lock();
        match Sound::new_with_data(manager.data[self].clone()) {
            Ok(mut s) => {
                s.play();
                manager.playing.push(s);
            },
            Err(err) => debug_assert!(false, "{:?}", err),
        }
    }
}

lazy_static! {
    static ref MANAGER: Mutex<AudioManager> =
        Mutex::new(AudioManager::new().expect("Failed to load sounds"));
}

struct AudioManager {
    playing: Vec<Sound>,
    // Use std::sync::Mutex here because it's what ears lib requires
    data:    EnumMap<SFX, Arc<std::sync::Mutex<SoundData>>>,
}

impl AudioManager {
    fn new() -> Result<Self, SoundError> {
        Ok(Self {
            playing: Vec::new(),
            data:    enum_map! {
                sfx => Arc::new(std::sync::Mutex::new(SoundData::new(&format!("assets/sounds/{}.wav", sfx))?)),
            },
        })
    }

    fn tick(&mut self) { self.playing.drain_filter(|s| !s.is_playing()); }
}

pub fn tick() { MANAGER.lock().tick(); }
