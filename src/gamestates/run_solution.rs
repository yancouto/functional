use crate::{
    levels::{get_result, Level, TestRunResults},
    math::*,
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
        let text = if let Err(err) = &self.results {
            format!("Failed to parse code:\n{}", err)
        } else {
            "Code compiled successfully.".to_owned()
        };
        data.text_box("Solution results", &text, Rect::new(20, 20, 40, 20));

        if let Ok(runs) = &self.results {
            let mut cur_i = 26;
            for (i, run) in runs.iter().enumerate() {
                let result_str = match &run.result {
                    Ok(node) => if *node == run.expected_result {
                        "SUCCESS!"
                    } else {
                        "WRONG ANSWER!"
                    }
                    .to_owned(),
                    Err(err) => format!("ERROR ({})", err),
                };
                data.print(
                    Pos::new(cur_i, 21),
                    &format!("Test Case #{}: {}", i, result_str),
                );
                if data.button("Debug", Pos::new(cur_i - 1, 50)) {
                    log::info!("Pressed debug {}!", i);
                }
                cur_i += 3;
            }
        }

        if data.time.as_secs() > 5 {
            self.save_profile
                .mark_level_as_tried(&self.level.name, get_result(&self.results));
            GameStateEvent::Pop
        } else {
            GameStateEvent::None
        }
    }
}
