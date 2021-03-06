use core::time;

use bracket_lib::prelude as bl;

const opening_str: &str = "this is functional.";

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

impl bl::GameState for IntroState {
    fn tick(&mut self, ctx: &mut bl::BTerm) {
        ctx.cls();
        self.time_since_creation_ms += ctx.frame_time_ms;
        let mut revealed_letters = (self.time_since_creation_ms as usize / 100);
        let len = opening_str.len();
        if revealed_letters > len {
            // Make last letter blink on and off, at a slower rate
            revealed_letters = len - 1 + ((revealed_letters - len) / 5) % 2;
        }
        ctx.print(10, 10, &opening_str[0..revealed_letters]);
    }
}
