use crate::gamestates::base::*;

const OPENING_STR: &str = "this is functional.";

// In practice will only be called once, but it's not FnOnce
pub trait GameStateBuilder = Fn() -> Box<dyn GameState>;
#[derive(Debug)]
pub struct IntroState<F: GameStateBuilder> {
    #[allow(unused)]
    time_since_creation_ms: f32,
    next:                   F,
}

impl<F: GameStateBuilder> IntroState<F> {
    pub fn new(next: F) -> Self {
        IntroState {
            time_since_creation_ms: 0.0,
            next,
        }
    }
}

impl<F: GameStateBuilder> GameState for IntroState<F> {
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
            GameStateEvent::Switch((self.next)())
        }
    }
}
