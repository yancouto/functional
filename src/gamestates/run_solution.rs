use crate::{levels::Level, math::Rect, save_system::SaveProfile};
use std::rc::Rc;

use super::base::*;

#[derive(Debug)]
pub struct RunSolutionState {
    save_profile: Rc<SaveProfile>,
    ans: bool,
}

impl RunSolutionState {
    pub fn new(
        level: &'static Level,
        code: impl Iterator<Item = char>,
        save_profile: Rc<SaveProfile>,
    ) -> Self {
        let ans = level.test(code);
        Self { save_profile, ans }
    }
}

impl GameState for RunSolutionState {
    fn name(&self) -> &'static str {
        "RunSolution"
    }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        data.text_box(
            "Running solution...",
            if self.ans {
                "Your solution is correct!"
            } else {
                "Your solution is wrong :("
            },
            Rect::new(20, 20, 20, 20),
        );
        if data.time.as_secs() > 3 {
            GameStateEvent::Pop
        } else {
            GameStateEvent::None
        }
    }
}
