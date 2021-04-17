use super::save_loader::SaveLoaderState;
use crate::{gamestates::base::*, DEFAULT_PROFILE};

const OPENING_STR: &str = "this is functional.";

#[derive(Debug)]
pub struct IntroState {
    time_since_creation_ms: f32,
}

impl IntroState {
    pub fn new() -> Self {
        IntroState {
            time_since_creation_ms: 0.0,
        }
    }
}

impl GameState for IntroState {
    fn name(&self) -> &'static str { "Intro" }

    fn tick(&mut self, data: TickData) -> GameStateEvent {
        let mut revealed_letters = (data.time.as_millis() as usize) / 100;
        let len = OPENING_STR.len();
        let mut switch = revealed_letters > len + 5 * 4;
        if data.left_click || data.pressed_key.is_some() {
            switch = true;
        }
        if revealed_letters > len {
            // Make last letter blink on and off, at a slower rate
            revealed_letters = len - 1 + ((revealed_letters - len) / 5) % 2;
        }
        data.console
            .print(20, 20, &OPENING_STR[0..revealed_letters]);
        if !switch {
            GameStateEvent::None
        } else {
            GameStateEvent::Switch(SaveLoaderState::try_load(DEFAULT_PROFILE.to_string()))
        }
    }
}
