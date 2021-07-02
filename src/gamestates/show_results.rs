use super::{base::*, debugger::DebuggerState};
use crate::{
    drawables::{black, Leaderboards}, interpreter::InterpretError, levels::{get_result, Level, TestRunResults}, math::*, prelude::*, save_system::{LevelResult, SaveProfile}
};
#[derive(Debug)]
pub struct ShowResultsState {
    level:        Level,
    save_profile: Arc<SaveProfile>,
    results:      TestRunResults,
    leaderboards: Leaderboards,
}

const BOX_W: i32 = 60;
const LDB_W: i32 = 40;
const BOX_H: i32 = 30;

impl ShowResultsState {
    pub fn new(level: Level, results: TestRunResults, save_profile: Arc<SaveProfile>) -> Self {
        save_profile.mark_level_as_tried(&level, get_result(&results));
        let stats = match get_result(&results) {
            LevelResult::Success { stats } => Some(stats),
            _ => None,
        };
        let ldr = Rect::new(
            (H - 2 * BOX_H - 1) / 2,
            (W - BOX_W - LDB_W - 6) / 2 + BOX_W + 6,
            LDB_W,
            BOX_H,
        );
        Self {
            level: level.clone(),
            save_profile,
            results,
            leaderboards: Leaderboards::new(
                ldr,
                level,
                stats,
                Rect::new(ldr.pos.i + BOX_H + 1, ldr.pos.j, ldr.size.w, ldr.size.h),
            ),
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
        let ret = Rect::new((H - BOX_H) / 2, (W - BOX_W - LDB_W - 6) / 2, BOX_W, BOX_H);
        data.text_box("Solution results", &text, ret.clone(), true);

        let mut instructions = Vec::with_capacity(2);
        instructions.push("Press ESC to go back to editor");
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
                        InterpretError::TooLarge => "NO REDUCTION (GREW TOO BIG)",
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
                    return GameStateEvent::Push(box DebuggerState::new(
                        self.level.clone(),
                        self.save_profile.clone(),
                        run.clone(),
                    ));
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
        self.leaderboards.draw(&mut data);

        if success {
            instructions.push("Press ENTER to exit level")
        }

        data.instructions(&instructions);

        if data.pressed_key == Some(bl::VirtualKeyCode::Escape) {
            SFX::Back.play();
            GameStateEvent::Pop(1)
        } else if success && data.pressed_key == Some(bl::VirtualKeyCode::Return) {
            SFX::Confirm.play();
            GameStateEvent::Pop(2)
        } else {
            GameStateEvent::None
        }
    }
}
