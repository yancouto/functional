use crate::math::{Pos, Rect, Size};
use std::time::Duration;

use super::base::{GameState, GameStateEvent, TickData};
use crate::drawables::TextEditor;
use crate::levels::Level;
use bracket_lib::prelude as bl;

#[derive(Debug)]
pub struct EditorState<'a> {
    time: Duration,
    level: &'a Level,
    editor: TextEditor,
}

impl EditorState<'static> {
    pub fn new(level: &'static Level) -> Self {
        let size = Size { w: 20, h: 8 };
        Self {
            time: Duration::from_secs(0),
            level,
            editor: TextEditor::new(Pos { i: 23, j: 2 }, Size { w: 20, h: 8 }),
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

        self.editor.draw(&mut data);

        if data.button("Run", Pos::new(47, 2)) {
            println!("Is ok? {}", self.level.test(self.editor.get_text()));
        }

        GameStateEvent::None
    }

    fn on_event(&mut self, event: bl::BEvent) {
        self.editor.on_event(&event);
    }
}
