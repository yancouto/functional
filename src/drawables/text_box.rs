use bracket_lib::prelude as bl;

use crate::gamestates::base::TickData;

fn white() -> bl::RGBA {
    bl::RGBA::named(bl::WHITE)
}
fn black() -> bl::RGBA {
    bl::RGBA::named(bl::BLACK)
}

impl TickData<'_> {
    pub fn text_box(&mut self, title: &str, text: &str, i: i32, j: i32, w: i32, h: i32) {
        self.console.draw_box(j, i, w, h, white(), black());
        self.console.print(j + 1, i, title);
        let mut tb = bl::TextBuilder::empty();
        // TODO: support \n's
        tb.ln().line_wrap(text).reset();

        let mut block = bl::TextBlock::new(j + 1, i + 1, w - 2, h - 2);
        block.print(&tb).unwrap();
        block.render(&mut self.console);
    }

    /// Button has height 3, width is the width of the string plus 2
    pub fn button(&mut self, text: &str, i: i32, j: i32) -> bool {
        self.console
            .draw_box(j, i, text.len() as i32 + 1, 2, white(), black());
        self.console.print(j + 1, i + 1, text);
        false
    }
}
