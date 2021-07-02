use ears::{AudioController, Sound, SoundData, SoundError};
use enum_map::{enum_map, Enum, EnumMap};
use parking_lot::Mutex;
use rand::Rng;

use crate::{prelude::*, save_system::load_common};

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
                s.set_pitch(rand::thread_rng().gen_range(0.8..1.2));
                s.set_volume(
                    (manager.volume as f32 / 10.0) * rand::thread_rng().gen_range(0.9..1.1),
                );
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
    volume:  u8,
}

impl AudioManager {
    fn new() -> Result<Self, SoundError> {
        let config = load_common();
        Ok(Self {
            playing: Vec::new(),
            data:    enum_map! {
                sfx => Arc::new(std::sync::Mutex::new(SoundData::new(&format!("assets/sounds/{}.wav", sfx))?)),
            },
            volume:  config.volume,
        })
    }

    fn tick(&mut self) { self.playing.drain_filter(|s| !s.is_playing()); }
}

pub fn tick() { MANAGER.lock().tick(); }

pub fn set_volume(vol: u8) { MANAGER.lock().volume = vol.min(10).max(0); }
