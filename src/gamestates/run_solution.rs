use crate::{levels::Level, math::Rect, save_system::SaveProfile};
use std::rc::Rc;

use super::base::*;

#[derive(Debug)]
pub struct RunSolutionState {
    level: &'static Level,
    code: String,
    save_profile: Rc<SaveProfile>,
}

impl RunSolutionState {
    pub fn new(level: &'static Level, code: String, save_profile: Rc<SaveProfile>) -> Self {
        Self {
            level,
            code,
            save_profile,
        }
    }
}

impl GameState for RunSolutionState {
    fn name(&self) -> &'static str {
        "RunSolution"
    }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        data.text_box("Running solution...", "hello", Rect::new(20, 20, 20, 20));
        if data.time.as_secs() > 3 {
            GameStateEvent::Pop
        } else {
            GameStateEvent::None
        }
    }
}
