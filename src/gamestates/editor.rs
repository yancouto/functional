use crate::{
    math::{Pos, Rect, Size},
    save_system::SaveProfile,
};
use std::{rc::Rc, time::Duration};

use super::base::{GameState, GameStateEvent, TickData};
use super::level_selection::LevelSelectionState;
use crate::drawables::TextEditor;
use crate::levels::Level;
use bracket_lib::prelude as bl;

#[derive(Debug)]
pub struct EditorState<'a> {
    time: Duration,
    level: &'a Level,
    editor: TextEditor,
    last_result: Option<bool>,
    last_result_expire_at: Duration,
    save_profile: Rc<SaveProfile>,
}

impl EditorState<'static> {
    pub fn new(level: &'static Level, save_profile: Rc<SaveProfile>) -> Self {
        let size = Size { w: 20, h: 8 };
        Self {
            time: Duration::from_secs(0),
            level,
            editor: TextEditor::new(Pos { i: 23, j: 2 }, Size { w: 20, h: 8 }),
            last_result: None,
            last_result_expire_at: Duration::from_secs(0),
            save_profile,
        }
    }
}

impl<'a> GameState for EditorState<'a> {
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

        self.editor.draw(&mut data);

        if data.button("Run", Pos::new(47, 2))
            || (data.ctrl && matches!(data.pressed_key, Some(bl::VirtualKeyCode::Return)))
        {
            self.last_result = Some(self.level.test(self.editor.get_text()));
            self.last_result_expire_at = data.time + Duration::from_secs(3);
        }

        if let Some(r) = self.last_result {
            data.print(Pos::new(48, 8), if r { "OK" } else { "WA" });
            if self.last_result_expire_at <= data.time {
                self.last_result = None;
            }
        }

        data.console
            .print_right(80, 47, "Click Run or press CTRL+ENTER to run");
        data.console.print_right(80, 49, "Press ESC to go back");

        if matches!(data.pressed_key, Some(bl::VirtualKeyCode::F10)) {
            self.save_profile.write_level(
                &self.level.name,
                0,
                &self.editor.get_text().collect::<String>(),
            );
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
