use crate::gamestates::base::*;
use bracket_lib::prelude as bl;

use super::editor;

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
    fn tick(&mut self, ctx: &mut bl::BTerm) -> GameStateEvent {
        self.time_since_creation_ms += ctx.frame_time_ms;
        let mut revealed_letters = self.time_since_creation_ms as usize / 100;
        let len = OPENING_STR.len();
        let switch = revealed_letters > len + 5 * 4;
        if revealed_letters > len {
            // Make last letter blink on and off, at a slower rate
            revealed_letters = len - 1 + ((revealed_letters - len) / 5) % 2;
        }
        ctx.print(10, 10, &OPENING_STR[0..revealed_letters]);
        if !switch {
            GameStateEvent::None
        } else {
            GameStateEvent::Switch(Box::new(editor::EditorState::new()))
        }
    }
}
