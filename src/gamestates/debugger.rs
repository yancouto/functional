use super::base::*;
use crate::{
    interpreter::{interpret_itermediates, ConstantProvider, Node}, levels::{Level, TestCaseRun}, prelude::*, save_system::SaveProfile
};
#[derive(Debug)]
pub struct DebuggerState {
    run:   TestCaseRun,
    steps: Vec1<Box<Node>>,
}

impl DebuggerState {
    pub fn new(level: Level, save_profile: Arc<SaveProfile>, run: TestCaseRun) -> Self {
        let mut steps = vec1![run.test_expression.clone()];
        steps.append(
            &mut interpret_itermediates(
                run.test_expression.clone(),
                false,
                ConstantProvider::new(level, save_profile),
            )
            .take(30)
            .collect(),
        );
        Self { run, steps }
    }
}

impl GameState for DebuggerState {
    fn name(&self) -> &'static str { "Debugger" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        data.console
            .print_centered(2, &format!("Expected result: {}", self.run.expected_result));
        data.console
            .print_centered(5, "Step by step test case reduction");
        let mut j = 8;
        for code in &self.steps {
            data.console.print_centered(j, &format!("{}", code));
            j += 3;
        }

        data.instructions(&["Press ESC to go back"]);

        if data.pressed_key == Some(bl::VirtualKeyCode::Escape) {
            GameStateEvent::Pop(1)
        } else {
            GameStateEvent::None
        }
    }
}
