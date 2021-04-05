use crate::{gamestates::base::TickData, math::*, prelude::*};

pub trait TextEditor {
    fn new(pos: Pos, size: Size) -> Self;
    fn on_event(&mut self, event: &bl::BEvent);
    fn load_text(&mut self, text: &str);
    fn to_string(&self) -> String;
    fn draw(&mut self, data: &mut TickData);
}
