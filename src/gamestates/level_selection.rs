use std::borrow::Cow;

use super::{base::*, editor::EditorState, main_menu::MainMenuState};
use crate::{
    drawables::XiEditor, levels::{Section, LEVELS}, prelude::*, save_system::{LevelResult, SaveProfile}, utils::vec_with_cursor::VecWithCursor
};

pub struct LevelSelectionState<'a> {
    /// Index of selected section
    sections:     VecWithCursor<&'a Section>,
    /// Index of selected level inside section
    level_i:      Option<usize>,
    save_profile: Arc<SaveProfile>,
}

impl LevelSelectionState<'static> {
    pub fn new(save_profile: Arc<SaveProfile>) -> Self {
        LevelSelectionState {
            sections: Vec1::try_from_vec(LEVELS.iter().map(|x| x).collect())
                .unwrap()
                .into(),
            level_i: None,
            save_profile,
        }
    }
}

const CURSOR_J: i32 = 3;
const START_I: i32 = 7;
const LINES_PER_SECTION: i32 = 3;
const MID_J: i32 = W / 2;

impl<'a> LevelSelectionState<'a> {
    fn get_i(&self, index: i32) -> i32 { START_I + LINES_PER_SECTION * index }
}

impl GameState for LevelSelectionState<'static> {
    fn name(&self) -> &'static str { "LevelSelection" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        data.print(Pos::new(2, CURSOR_J), "All sections");
        for (i, section) in self.sections.inner().iter().enumerate() {
            data.print(
                Pos::new(START_I + LINES_PER_SECTION * i as i32, CURSOR_J + 2),
                &section.name.to_string(),
            );
        }
        let cursor_on = ((data.time.as_millis() / 500) % 2) == 0;
        if self.level_i.is_some() {
            data.print(Pos::new(2, MID_J + CURSOR_J), "Levels in section");
            let j = MID_J + CURSOR_J + 2;
            if self.sections.get().levels.is_empty() && cfg!(feature = "demo") {
                data.print(Pos::new(self.get_i(0), j), "BUY FULL GAME TO UNLOCK");
            }
            let mut levels_info = self.save_profile.get_levels_info();
            for (i, level) in self.sections.get().levels.iter().enumerate() {
                let info = levels_info.entry(level.base.name.clone()).or_default();
                let mut text = Cow::Borrowed(&level.base.name);
                match info.result {
                    LevelResult::Success { stats } => text.to_mut().push_str(&format!(
                        " (completed, {:.2} reductions, {} functions)",
                        stats.reductions_x100 as f32 / 100.0,
                        stats.functions,
                    )),
                    LevelResult::Failure => text.to_mut().push_str(" (failed)"),
                    LevelResult::NotTried => {},
                }
                data.print(Pos::new(self.get_i(i as i32), j), &text);
            }
            data.instructions(&[
                "Press ESC/LEFT to close section",
                "Use UP/DOWN to navigate tasks",
                "Press ENTER to select task",
            ]);
        } else {
            data.instructions(&[
                "Use UP/DOWN to navigate sections",
                "Press ENTER/RIGHT to open section",
                "Press ESC to go to main menu",
            ]);
        }
        if self.level_i.is_some() {
            // Always show cursor if section is selected
            data.print(
                Pos::new(self.get_i(self.sections.cursor() as i32), CURSOR_J),
                ">",
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
        match data.pressed_key {
            Some(Key::Escape) =>
                if self.level_i.is_some() {
                    SFX::Select.play();
                    self.level_i.take();
                    GameStateEvent::None
                } else {
                    SFX::Back.play();
                    GameStateEvent::Switch(box MainMenuState::new(self.save_profile.clone(), false))
                },
            Some(Key::Return) =>
                if let Some(l_i) = self.level_i {
                    if self.sections.get().levels.is_empty() {
                        SFX::Wrong.play();
                        GameStateEvent::None
                    } else {
                        SFX::Confirm.play();
                        GameStateEvent::Push(box EditorState::<XiEditor>::new(
                            (&self.sections.get().levels[l_i]).into(),
                            self.save_profile.clone(),
                        ))
                    }
                } else {
                    SFX::Select.play();
                    self.level_i = Some(0);
                    GameStateEvent::None
                },
            _ => GameStateEvent::None,
        }
    }

    fn on_event(&mut self, event: bl::BEvent, _input: &bl::Input) {
        match event {
            bl::BEvent::KeyboardInput {
                key, pressed: true, ..
            } => match key {
                Key::Down =>
                    if let Some(li) = self.level_i {
                        if !self.sections.get().levels.is_empty() {
                            self.level_i = Some((li + 1) % self.sections.get().levels.len());
                        }
                    } else {
                        self.sections.cursor_increment();
                    },
                Key::Up =>
                    if let Some(li) = self.level_i {
                        let len = self.sections.get().levels.len();
                        if len != 0 {
                            self.level_i = Some((li + len - 1) % len);
                        }
                    } else {
                        self.sections.cursor_decrement();
                    },
                Key::Right if self.level_i.is_none() => {
                    self.level_i = Some(0);
                },
                Key::Left if self.level_i.is_some() => {
                    self.level_i = None;
                },
                _ => {},
            },
            _ => {},
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn section_names_too_long() {
        for section in LEVELS.iter() {
            assert!(
                section.name.to_string().len() as i32 + CURSOR_J + 2 <= MID_J,
                "Too long name"
            );
        }
    }
}
