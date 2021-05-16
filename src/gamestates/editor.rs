use std::time::Duration;

use super::base::{GameState, GameStateEvent, TickData};
use crate::{
    drawables::{black, dark_gray, TextEditor}, gamestates::{playground::PlaygroundState, running_solution::RunningSolutionState}, levels::Level, math::{Rect, Size}, prelude::*, save_system::SaveProfile
};

#[derive(Debug)]
pub struct EditorState<Editor: TextEditor> {
    level:            &'static Level,
    editor:           Editor,
    current_solution: u8,
    save_profile:     Rc<SaveProfile>,
    last_save:        Duration,
    known_constants:  Option<Vec1<&'static str>>,
    pressed_hint:     bool,
}

impl<Editor: TextEditor> EditorState<Editor> {
    pub fn new(level: &'static Level, save_profile: Rc<SaveProfile>) -> Self {
        let mut state = Self {
            level,
            editor: Editor::new(
                "Text Editor".to_string(),
                Rect {
                    pos:  Pos { i: 36, j: 1 },
                    size: Size { w: W / 2, h: 25 },
                },
                String::new(),
            ),
            save_profile,
            current_solution: 1,
            last_save: Duration::from_secs(0),
            known_constants: Vec1::try_from_vec(level.all_known_constants()).ok(),
            pressed_hint: false,
        };
        state.load_solution(1);
        state
    }

    fn load_solution(&mut self, solution: u8) {
        let _ = self.editor.load_file(
            self.save_profile
                .level_code_file(&self.level.name, solution),
        );
        self.current_solution = solution;
    }

    fn save_current_solution(&mut self, current_time: Duration) {
        self.save_profile.write_level(
            &self.level.name,
            self.current_solution,
            &self.editor.to_string(),
        );
        self.last_save = current_time;
    }
}

impl<Editor: 'static + TextEditor> GameState for EditorState<Editor> {
    fn name(&self) -> &'static str { "Editor" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        data.text_box(
            &self.level.name,
            &self.level.description,
            Rect::new(1, 0, W / 2, 30),
            true,
        );
        if let Some(info) = &self.level.extra_info {
            if self.level.extra_info_is_hint && !self.pressed_hint {
                self.pressed_hint = data.button("Get hint", Pos::new(2, W / 2 + 1), black());
            } else {
                data.text_box(
                    if self.level.extra_info_is_hint {
                        "Hint"
                    } else {
                        "Extra info"
                    },
                    info,
                    Rect::new(1, W / 2 + 1, W - W / 2 - 1, 20),
                    true,
                );
            }
        }

        for idx in 1..4u8 {
            if data.button(
                &idx.to_string(),
                Pos::new(31, (idx as i32 - 1) * 3),
                if idx == self.current_solution {
                    dark_gray()
                } else {
                    black()
                },
            ) && idx != self.current_solution
            {
                self.save_current_solution(data.time);
                self.load_solution(idx);
            }
        }

        if data.time - self.last_save > Duration::from_secs(20) {
            log::debug!("Auto saving code!");
            self.save_current_solution(data.time);
        }

        self.editor.draw(&mut data);

        if let Some(cts) = &self.known_constants {
            data.text_box(
                "Known constants",
                &cts.join(", "),
                Rect::new(35, W / 2 + 2, W / 2 - 2, 27),
                true,
            )
        }

        const RUN: &str = "Run";
        const OPEN: &str = "Open on playground";

        if data.button(RUN, Pos::new(H - 3, 2), black())
            || (data.ctrl && matches!(data.pressed_key, Some(Key::Return)))
        {
            self.save_current_solution(data.time);
            return GameStateEvent::Push(box RunningSolutionState::new(
                self.level,
                self.editor.to_string(),
                self.save_profile.clone(),
            ));
        } else if data.button(OPEN, Pos::new(H - 3, 2 + RUN.len() as i32 + 3), black()) {
            return GameStateEvent::Push(box PlaygroundState::<Editor>::new(
                self.editor.to_string(),
            ));
        }

        data.instructions(&[
            "Click Run or press CTRL+ENTER to run code",
            "Press ESC to go back",
        ]);

        if matches!(data.pressed_key, Some(Key::F10)) {
            self.save_current_solution(data.time);
        }

        if matches!(data.pressed_key, Some(Key::Escape)) {
            self.save_current_solution(data.time);
            GameStateEvent::Pop(1)
        } else {
            GameStateEvent::None
        }
    }

    fn on_event(&mut self, event: bl::BEvent, input: &bl::Input) {
        self.editor.on_event(&event, input);
    }
}
