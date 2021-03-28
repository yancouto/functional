use bracket_lib::prelude as bl;

use crate::gamestates::base::TickData;
use crate::math::{Pos, Rect, Size};

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
    pub fn draw_box(&mut self, title: &str, rect: Rect) {
        let Rect { pos, size } = rect;
        self.console
            .draw_box(pos.j, pos.i, size.w - 1, size.h - 1, white(), black());
        self.console.print(pos.j + 1, pos.i, title);
    }

    pub fn text_box(&mut self, title: &str, text: &str, rect: Rect) {
        self.draw_box(title, rect);
        let Rect { pos, size } = rect;
        let mut tb = bl::TextBuilder::empty();
        // TODO: support \n's
        tb.ln();
        for line in text.trim().split('\n') {
            tb.line_wrap(line.trim()).ln();
        }
        tb.reset();

        let mut block = bl::TextBlock::new(pos.j + 1, pos.i + 1, size.w - 3, size.h - 3);
        block.print(&tb).unwrap();
        block.render(&mut self.console);
    }

    /// Button has height 3, width is the width of the string plus 2
    pub fn button(&mut self, text: &str, pos: Pos) -> bool {
        let size = Size::new(text.len() as i32 + 2, 3);
        let rect = Rect { pos, size };
        let mut was_clicked = false;
        let bg = if self.mouse_pos.inside(&rect) {
            if self.left_click {
                was_clicked = true;
                white()
            } else {
                gray()
            }
        } else {
            black()
        };
        self.console
            .draw_box(pos.j, pos.i, size.w - 1, size.h - 1, white(), bg);
        self.console
            .print_color(pos.j + 1, pos.i + 1, white(), bg, text);
        was_clicked
    }

    #[allow(dead_code)]
    pub fn char(&mut self, pos: Pos, c: char) {
        self.console
            .set(pos.j, pos.i, white(), black(), bl::to_cp437(c));
    }

    pub fn print(&mut self, pos: Pos, text: &str) {
        self.console.print(pos.j, pos.i, text);
    }
}
