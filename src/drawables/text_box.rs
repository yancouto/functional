use bracket_lib::prelude as bl;

use crate::gamestates::base::TickData;
use crate::math::{Pos, Size};

fn white() -> bl::RGBA {
    bl::RGBA::named(bl::WHITE)
}
fn black() -> bl::RGBA {
    bl::RGBA::named(bl::BLACK)
}
fn gray() -> bl::RGBA {
    bl::RGBA::named(bl::GRAY)
}

impl TickData<'_> {
    pub fn text_box(&mut self, title: &str, text: &str, pos: Pos, size: Size) {
        self.console
            .draw_box(pos.j, pos.i, size.w, size.h, white(), black());
        self.console.print(pos.j + 1, pos.i, title);
        let mut tb = bl::TextBuilder::empty();
        // TODO: support \n's
        tb.ln().line_wrap(text).reset();

        let mut block = bl::TextBlock::new(pos.j + 1, pos.i + 1, size.w - 2, size.h - 2);
        block.print(&tb).unwrap();
        block.render(&mut self.console);
    }

    /// Button has height 3, width is the width of the string plus 2
    pub fn button(&mut self, text: &str, pos: Pos) -> bool {
        self.console
            .draw_box(pos.j, pos.i, text.len() as i32 + 1, 2, white(), black());
        self.console.print(pos.j + 1, pos.i + 1, text);
        false
    }
}
