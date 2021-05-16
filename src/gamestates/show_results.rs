use super::{base::*, debugger::DebuggerState};
use crate::{
    drawables::black, interpreter::InterpretError, levels::{get_result, Level, TestRunResults}, math::*, prelude::*, save_system::{LevelResult, SaveProfile}
};
#[derive(Debug)]
pub struct ShowResultsState {
    level:        &'static Level,
    save_profile: Rc<SaveProfile>,
    results:      TestRunResults,
}

impl ShowResultsState {
    pub fn new(
        level: &'static Level,
        results: TestRunResults,
        save_profile: Rc<SaveProfile>,
    ) -> Self {
        save_profile.mark_level_as_tried(&level.name, get_result(&results));
        Self {
            level,
            save_profile,
            results,
        }
    }
}

const DEBUG: &str = "Explain";

impl GameState for ShowResultsState {
    fn name(&self) -> &'static str { "ShowResults" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        let text = if let Err(err) = &self.results {
            format!("Failed to parse expression:\n{}", err)
        } else {
            "Parsed expression successfully.".to_owned()
        };
        let ret = Rect::centered(60, 30);
        data.text_box("Solution results", &text, ret.clone());

        let mut instructions = Vec::with_capacity(2);
        instructions.push("Press ESC to go back");
        let mut success = false;

        if let Ok(runs) = &self.results {
            let mut cur_i = ret.pos.i + 5;
            for (i, run) in runs.runs.iter().enumerate() {
                let result_str = match &run.result {
                    Ok(node) =>
                        if node.term == run.expected_result {
                            format!("SUCCESS! ({} reductions)", node.stats.reductions)
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
            if let LevelResult::Success { stats } = get_result(&self.results) {
                success = true;
                data.print(
                    Pos::new(ret.bottom() - 3, ret.left() + 2),
                    &format!(
                        "Average reductions: {:.2}",
                        stats.reductions_x100 as f32 / 100.0
                    ),
                );
                data.print(
                    Pos::new(ret.bottom() - 2, ret.left() + 2),
                    &format!("Functions used: {}", stats.functions),
                );
            }
        }

        if success {
            instructions.push("Press ENTER to go to level selection")
        }

        data.instructions(&instructions);

        if data.pressed_key == Some(bl::VirtualKeyCode::Escape) {
            GameStateEvent::Pop(1)
        } else if success && data.pressed_key == Some(bl::VirtualKeyCode::Return) {
            GameStateEvent::Pop(2)
        } else {
            GameStateEvent::None
        }
    }
}
