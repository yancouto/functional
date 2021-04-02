use super::base::*;
use super::editor::EditorState;
use crate::levels::{Level, LEVELS};
use crate::prelude::*;
use crate::save_system::{LevelResult, SaveProfile};
use crate::utils::vec_with_cursor::VecWithCursor;

use std::borrow::Cow;
use vec1::vec1;

struct Section<'a> {
    name: String,
    levels: Vec<&'a Level>,
}

pub struct LevelSelectionState<'a> {
    /// Index of selected section
    sections: VecWithCursor<Section<'a>>,
    /// Index of selected level inside section
    level_i: Option<usize>,
    save_profile: Rc<SaveProfile>,
}

impl LevelSelectionState<'static> {
    pub fn new(save_profile: Rc<SaveProfile>) -> Self {
        let random_levels = |n: usize| {
            (0..n)
                .map(|i| &LEVELS[i % LEVELS.len()])
                .collect::<Vec<&Level>>()
        };
        let l = LevelSelectionState {
            sections: vec1![Section {
                name: "basic".to_string(),
                levels: random_levels(5),
            }]
            .into(),
            level_i: None,
            save_profile,
        };
        for section in l.sections.inner() {
            if section.name.len() as i32 + CURSOR_J + 2 > MID_J {
                panic!("Too long name");
            }
        }
        l
    }
}

const CURSOR_J: i32 = 3;
const START_I: i32 = 2;
const LINES_PER_SECTION: i32 = 3;
const MID_J: i32 = W / 2;

impl<'a> LevelSelectionState<'a> {
    fn get_i(&self, index: i32) -> i32 {
        START_I + LINES_PER_SECTION * index
    }
}

impl GameState for LevelSelectionState<'static> {
    fn name(&self) -> &'static str {
        "LevelSelection"
    }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        for (i, section) in self.sections.inner().iter().enumerate() {
            data.print(
                Pos::new(START_I + LINES_PER_SECTION * i as i32, CURSOR_J + 2),
                &section.name,
            );
        }
        let cursor_on = ((data.time.as_millis() / 500) % 2) == 0;
        if let Some(l_i) = self.level_i {
            let mut levels_info = self.save_profile.get_levels_info();
            for (i, level) in self.sections.get().levels.iter().enumerate() {
                let info = levels_info.entry(level.name.clone()).or_default();
                let mut text = Cow::Borrowed(&level.name);
                match info.result {
                    LevelResult::Success => text.to_mut().push_str(" (completed)"),
                    LevelResult::Failure => text.to_mut().push_str(" (failed)"),
                    LevelResult::NotTried => {}
                }
                data.print(Pos::new(self.get_i(i as i32), MID_J + CURSOR_J + 2), &text);
            }
            data.print(
                Pos::new(self.get_i(l_i as i32) + 1, MID_J + CURSOR_J + 5),
                "press ENTER to select level",
            )
        } else {
            data.print(
                Pos::new(self.get_i(self.sections.cursor() as i32), MID_J + CURSOR_J),
                "press RIGHT to open section",
            );
        }
        if cursor_on {
            data.print(
                Pos::new(
                    self.get_i(self.level_i.unwrap_or(self.sections.cursor()) as i32),
                    CURSOR_J + self.level_i.map_or(0, |_| MID_J),
                ),
                ">",
            );
        }
        match data.pressed_key.zip(self.level_i) {
            Some((bl::VirtualKeyCode::Return, l_i)) => GameStateEvent::Switch(
                box EditorState::new(self.sections.get().levels[l_i], self.save_profile.clone()),
            ),
            _ => GameStateEvent::None,
        }
    }

    fn on_event(&mut self, event: bl::BEvent) {
        match event {
            bl::BEvent::KeyboardInput {
                key, pressed: true, ..
            } => match key {
                bl::VirtualKeyCode::Down => {
                    if let Some(li) = self.level_i {
                        self.level_i = Some((li + 1) % self.sections.get().levels.len());
                    } else {
                        self.sections.cursor_increment();
                    }
                }
                bl::VirtualKeyCode::Up => {
                    if let Some(li) = self.level_i {
                        let len = self.sections.get().levels.len();
                        self.level_i = Some((li + len - 1) % len);
                    } else {
                        self.sections.cursor_decrement();
                    }
                }
                bl::VirtualKeyCode::Right if self.level_i.is_none() => {
                    self.level_i = Some(0);
                }
                bl::VirtualKeyCode::Left if self.level_i.is_some() => {
                    self.level_i = None;
                }
                _ => {}
            },
            _ => {}
        }
    }
}
