use crate::{
    levels::{get_result, Level, TestRunResults},
    math::Rect,
    save_system::SaveProfile,
};
use std::fmt::Write;
use std::rc::Rc;

use super::base::*;

#[derive(Debug)]
pub struct RunSolutionState {
    level: &'static Level,
    save_profile: Rc<SaveProfile>,
    results: TestRunResults,
}

impl RunSolutionState {
    pub fn new(
        level: &'static Level,
        code: impl Iterator<Item = char>,
        save_profile: Rc<SaveProfile>,
    ) -> Self {
        Self {
            level,
            save_profile,
            // Do we need to not block here? Probably not.
            results: level.test(code),
        }
    }
}

impl GameState for RunSolutionState {
    fn name(&self) -> &'static str {
        "RunSolution"
    }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        let mut text = String::new();
        match &self.results {
            Ok(runs) => runs
                .iter()
                .enumerate()
                .map(|(i, run)| {
                    write!(&mut text, "Test Case #{}: ", i)?;
                    match &run.result {
                        Ok(node) => {
                            if *node == run.expected_result {
                                writeln!(&mut text, "SUCCESS!")
                            } else {
                                writeln!(&mut text, "WRONG ANSWER!")
                            }
                        }
                        Err(err) => writeln!(&mut text, "ERROR ({})", err),
                    }
                })
                .collect(),
            Err(err) => writeln!(&mut text, "Failed to parse code: {}", err),
        }
        .unwrap();
        data.text_box("Running solution...", &text, Rect::new(20, 20, 40, 20));
        if data.time.as_secs() > 5 {
            self.save_profile
                .mark_level_as_tried(&self.level.name, get_result(&self.results));
            GameStateEvent::Pop
        } else {
            GameStateEvent::None
        }
    }
}
