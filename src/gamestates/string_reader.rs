use crossbeam::channel::*;

use super::base::*;
use crate::{
    drawables::{BasicTextEditor, TextEditor, TextEditorInner}, prelude::*
};

#[derive(Debug)]
// Simple text box, sends text back when gamestate is closed
// If ESC is pressed, returns None. Disallows empty string otherwise.
pub struct StringReaderState {
    #[allow(unused)]
    max_width: i32,
    sender:    Sender<Option<String>>,
    editor:    BasicTextEditor,
}

impl StringReaderState {
    pub fn new(title: String, max_width: i32) -> (Self, Receiver<Option<String>>) {
        let (send, recv) = bounded(1);
        (
            Self {
                max_width,
                sender: send,
                editor: BasicTextEditor::new(title, Rect::centered(max_width, 1), String::new()),
            },
            recv,
        )
    }
}

impl GameState for StringReaderState {
    fn name(&self) -> &'static str { "StringReader" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        self.editor.draw(&mut data);
        data.instructions(&["Press ENTER to create level", "Press ESC to go back"]);
        if data.pressed_key == Some(Key::Escape) {
            self.sender.send(None).debug_unwrap();
            GameStateEvent::Pop(1)
        } else if data.pressed_key == Some(Key::Return) {
            let text = self.editor.to_string().trim().to_string();
            if text.is_empty() {
                // TODO: beep invalid
                GameStateEvent::None
            } else {
                self.sender.send(Some(text)).debug_unwrap();
                GameStateEvent::Pop(1)
            }
        } else {
            GameStateEvent::None
        }
    }

    fn on_event(&mut self, event: bl::BEvent, input: &bl::Input) {
        self.editor.on_event(&event, &input);
    }

    fn clear_terminal(&self) -> bool { false }
}
