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
    fn pitch_var(self) -> f32 {
        match self {
            Self::Win => 0.01,
            Self::Wrong => 0.05,
            _ => 0.15,
        }
    }

    fn volume_var(self) -> f32 {
        match self {
            Self::Win | Self::Wrong => 0.01,
            _ => 0.1,
        }
    }

    pub fn play(self) {
        let (pitch_var, volume_var) = (self.pitch_var(), self.volume_var());
        let mut manager = MANAGER.lock();
        match Sound::new_with_data(manager.data[self].clone()) {
            Ok(mut s) => {
                s.set_pitch(rand::thread_rng().gen_range((1.0 - pitch_var)..(1.0 + pitch_var)));
                s.set_volume(
                    (manager.volume as f32 / 10.0)
                        * rand::thread_rng().gen_range((1.0 - volume_var)..(1.0 + volume_var)),
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

    fn tick(&mut self) {
        self.playing
            .extract_if(|s| !s.is_playing())
            .for_each(std::mem::drop);
    }
}

pub fn tick() { MANAGER.lock().tick(); }

pub fn set_volume(vol: u8) { MANAGER.lock().volume = vol.min(10).max(0); }
