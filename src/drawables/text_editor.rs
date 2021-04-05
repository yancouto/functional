use std::path::PathBuf;

use crate::{gamestates::base::TickData, math::*, prelude::*};

pub trait TextEditor {
    fn new(pos: Pos, size: Size) -> Self;
    fn on_event(&mut self, event: &bl::BEvent);
    fn load_file(&mut self, path: PathBuf) -> std::io::Result<()>;
    fn to_string(&self) -> String;
    fn draw(&mut self, data: &mut TickData);
}
