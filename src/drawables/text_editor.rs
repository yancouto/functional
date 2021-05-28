use std::path::PathBuf;

use crate::{gamestates::base::TickData, math::*, prelude::*};

pub trait TextEditor {
    /// rect is just the dimensions of the text, not of the whole rectangle
    /// so there's still a border around it (1 left, down, right, 2 up)
    fn new(title: String, rect: Rect, initial_text: String) -> Self;
    fn on_event(&mut self, event: &bl::BEvent, input: &bl::Input);
    fn load_file(&mut self, path: PathBuf) -> std::io::Result<()>;
    fn to_string(&self) -> String;
    fn draw(&mut self, data: &mut TickData);

    fn rect(&self) -> &Rect;
}
