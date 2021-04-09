use super::base::*;
use crate::{
    interpreter::{interpret_itermediates, Node}, levels::TestCaseRun, prelude::*
};

#[derive(Debug)]
pub struct DebuggerState {
    run:   TestCaseRun,
    steps: Vec1<Box<Node>>,
}

impl DebuggerState {
    pub fn new(run: TestCaseRun) -> Self {
        let mut steps = vec1![run.test_expression.clone()];
        steps.append(
            &mut interpret_itermediates(run.test_expression.clone(), true)
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
            .print_centered(3, "Step by step test case reduction");
        let mut j = 6;
        for code in &self.steps {
            data.console.print_centered(j, &format!("{:?}", code));
            j += 3;
        }

        data.instructions(&["Press ESC to go back"]);

        if data.pressed_key == Some(bl::VirtualKeyCode::Escape) {
            GameStateEvent::Pop
        } else {
            GameStateEvent::None
        }
    }
}
