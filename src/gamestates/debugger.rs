use crate::{
    interpreter::{interpret_itermediates, Node},
    levels::TestCaseRun,
    prelude::*,
};

use super::base::*;

#[derive(Debug)]
pub struct DebuggerState {
    run: TestCaseRun,
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
    fn name(&self) -> &'static str {
        "Debugger"
    }

    fn tick(&mut self, data: TickData) -> GameStateEvent {
        data.console.print_centered(3, "Running solution");
        let mut j = 5;
        for code in &self.steps {
            data.console.print_centered(j, &format!("{:?}", code));
            j += 2;
        }
        if data.pressed_key == Some(bl::VirtualKeyCode::Escape) {
            GameStateEvent::Pop
        } else {
            GameStateEvent::None
        }
    }
}