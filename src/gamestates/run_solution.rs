use super::{base::*, debugger::DebuggerState};
use crate::{
    drawables::black, interpreter::{ConstantProvider, InterpretError}, levels::{get_result, Level, TestRunResults}, math::*, prelude::*, save_system::SaveProfile
};

#[derive(Debug)]
pub struct RunSolutionState {
    level:        &'static Level,
    save_profile: Rc<SaveProfile>,
    results:      TestRunResults,
}

impl RunSolutionState {
    pub fn new(
        level: &'static Level,
        code: impl Iterator<Item = char>,
        save_profile: Rc<SaveProfile>,
    ) -> Self {
        // TODO: not use 100 here
        let results = level.test(code, ConstantProvider::new(level));
        save_profile.mark_level_as_tried(&level.name, get_result(&results));
        Self {
            level,
            save_profile,
            // Do we need to not block here? Probably not.
            results,
        }
    }
}

const DEBUG: &str = "Explain";

impl GameState for RunSolutionState {
    fn name(&self) -> &'static str { "RunSolution" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        let text = if let Err(err) = &self.results {
            format!("Failed to parse expression:\n{}", err)
        } else {
            "Parsed expression successfully.".to_owned()
        };
        let ret = Rect::centered(60, 30);
        data.text_box("Solution results", &text, ret.clone());

        data.instructions(&["Press ESC to go back"]);

        if let Ok(runs) = &self.results {
            let mut cur_i = ret.pos.i + 5;
            for (i, run) in runs.runs.iter().enumerate() {
                let result_str = match &run.result {
                    Ok(node) =>
                        if node.term == run.expected_result {
                            format!("SUCCESS! ({} reductions)", node.reductions)
                        } else {
                            "WRONG ANSWER!".to_owned()
                        },
                    Err(err) => match err {
                        InterpretError::AlgorithmError => "UNKNOWN ERROR, CONTACT DEVELOPERS!",
                        InterpretError::TooDeep => "NO REDUCTION (INFINITE LOOP)",
                    }
                    .to_owned(),
                };
                data.print(
                    Pos::new(cur_i, ret.pos.j + 2),
                    &format!("Test Case #{}: {}", i, result_str),
                );
                if data.button(
                    DEBUG,
                    Pos::new(cur_i - 1, ret.pos.j + ret.size.w - DEBUG.len() as i32 - 4),
                    black(),
                ) {
                    return GameStateEvent::Push(box DebuggerState::new(self.level, run.clone()));
                }
                cur_i += 3;
            }
        }

        if data.pressed_key == Some(bl::VirtualKeyCode::Escape) {
            GameStateEvent::Pop
        } else {
            GameStateEvent::None
        }
    }
}
