use std::thread;

use crossbeam::channel;

use super::{base::*, show_results::ShowResultsState};
use crate::{
    interpreter::ConstantProvider, levels::{Level, TestRunResults}, math::*, prelude::*, save_system::SaveProfile
};
#[derive(Debug)]
pub struct RunningSolutionState {
    level:        &'static Level,
    save_profile: Rc<SaveProfile>,
    handle:       thread::JoinHandle<()>,
    receiver:     channel::Receiver<TestRunResults>,
}

impl RunningSolutionState {
    pub fn new(level: &'static Level, code: String, save_profile: Rc<SaveProfile>) -> Self {
        let (sender, receiver) = channel::bounded(0);
        let handle = std::thread::spawn(move || {
            sender
                .send(level.test(code.chars(), ConstantProvider::new(level)))
                .debug_unwrap()
        });
        Self {
            level,
            save_profile,
            handle,
            receiver,
        }
    }
}

const WAIT_TEXT: &str = "Running solution, please wait";

impl GameState for RunningSolutionState {
    fn name(&self) -> &'static str { "RunningSolution" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        if let Ok(results) = self.receiver.try_recv() {
            GameStateEvent::Switch(box ShowResultsState::new(
                self.level,
                results,
                self.save_profile.clone(),
            ))
        } else if data.pressed_key == Some(bl::VirtualKeyCode::Escape) {
            GameStateEvent::Pop
        } else {
            // Let's draw here in the reasonably common case where the solution runs very fast, in that case let's not print
            // it for one frame cause it looks a bit weird
            let dots = (data.time.as_millis() / 500) % 4;
            let mut txt = String::with_capacity(WAIT_TEXT.len() + 4);
            txt.push_str(WAIT_TEXT);
            (0..dots).for_each(|_| txt.push('.'));

            data.print(Pos::new(H / 2, W / 2 - WAIT_TEXT.len() as i32 / 2), &txt);
            data.instructions(&["Press ESC to cancel run"]);
            GameStateEvent::None
        }
    }
}
