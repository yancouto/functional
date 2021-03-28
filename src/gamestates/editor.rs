use crate::{
    math::{Pos, Rect, Size},
    save_system::SaveProfile,
};
use std::{rc::Rc, time::Duration};

use super::base::{GameState, GameStateEvent, TickData};
use super::{level_selection::LevelSelectionState, run_solution::RunSolutionState};
use crate::drawables::TextEditor;
use crate::levels::Level;
use bracket_lib::prelude as bl;

#[derive(Debug)]
pub struct EditorState {
    time: Duration,
    level: &'static Level,
    editor: TextEditor,
    last_result: Option<bool>,
    last_result_expire_at: Duration,
    current_solution: u8,
    save_profile: Rc<SaveProfile>,
}

impl EditorState {
    pub fn new(level: &'static Level, save_profile: Rc<SaveProfile>) -> Self {
        let mut state = Self {
            time: Duration::from_secs(0),
            level,
            editor: TextEditor::new(Pos { i: 26, j: 1 }, Size { w: 30, h: 15 }),
            last_result: None,
            last_result_expire_at: Duration::from_secs(0),
            save_profile,
            current_solution: 1,
        };
        state.load_solution(1);
        state
    }

    fn load_solution(&mut self, solution: u8) {
        self.editor
            .load_text(&self.save_profile.read_level(&self.level.name, solution));
        self.current_solution = solution;
    }

    fn save_current_solution(&mut self) {
        self.save_profile.write_level(
            &self.level.name,
            self.current_solution,
            &self.editor.to_string(),
        );
    }
}

impl GameState for EditorState {
    fn name(&self) -> &'static str {
        "Editor"
    }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        data.text_box(
            &self.level.name,
            &self.level.description,
            Rect::new(1, 0, 50, 20),
        );
        if let Some(info) = &self.level.extra_info {
            data.text_box("Extra info", info, Rect::new(1, 50, 30, 20));
        }

        for i in 1..4u8 {
            if data.button(&i.to_string(), Pos::new(21, (i as i32 - 1) * 3))
                && i != self.current_solution
            {
                self.save_current_solution();
                self.load_solution(i);
            }
        }

        self.editor.draw(&mut data);

        if data.button("Run", Pos::new(47, 2))
            || (data.ctrl && matches!(data.pressed_key, Some(bl::VirtualKeyCode::Return)))
        {
            return GameStateEvent::Push(box RunSolutionState::new(
                self.level,
                self.editor.get_chars(),
                self.save_profile.clone(),
            ));
        }

        data.console
            .print_right(80, 47, "Click Run or press CTRL+ENTER to run");
        data.console.print_right(80, 49, "Press ESC to go back");

        if matches!(data.pressed_key, Some(bl::VirtualKeyCode::F10)) {
            self.save_current_solution();
        }

        if matches!(data.pressed_key, Some(bl::VirtualKeyCode::Escape)) {
            GameStateEvent::Switch(box LevelSelectionState::new(self.save_profile.clone()))
        } else {
            GameStateEvent::None
        }
    }

    fn on_event(&mut self, event: bl::BEvent) {
        self.editor.on_event(&event);
    }
}
