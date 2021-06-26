use crate::{
    gamestates::base::TickData, math::{Pos, Rect, Size}, prelude::*
};

pub fn white() -> bl::RGBA { bl::RGBA::named(bl::WHITE) }
pub fn black() -> bl::RGBA { bl::RGBA::named(bl::BLACK) }
pub fn gray() -> bl::RGBA { bl::RGBA::named(bl::GRAY) }
pub fn dark_gray() -> bl::RGBA { bl::RGBA::named(bl::DARK_GRAY) }
pub fn light_red() -> bl::RGBA { bl::RGBA::from_u8(255, 100, 100, 255) }

impl TickData<'_> {
    pub fn draw_box_color(&mut self, rect: Rect, fg: bl::RGBA, bg: bl::RGBA) {
        let Rect { pos, size } = rect;
        self.console
            .draw_box(pos.j, pos.i, size.w - 1, size.h - 1, fg, bg);
    }

    pub fn title_box_color(&mut self, title: &str, rect: Rect, fg: bl::RGBA, bg: bl::RGBA) {
        self.draw_box_color(rect, fg, bg);
        self.print(Pos::new(rect.pos.i, rect.pos.j + 1), title);
    }

    pub fn title_box(&mut self, title: &str, rect: Rect) {
        self.title_box_color(title, rect, white(), black());
    }

    pub fn text_box(&mut self, title: &str, text: &str, rect: Rect, fail_on_out_of_space: bool) {
        self.title_box(title, rect);
        let Rect { pos, size } = rect;
        let mut tb = bl::TextBuilder::empty();
        tb.ln();
        for line in text.trim().split('\n') {
            tb.line_wrap(line.trim()).ln();
        }
        tb.reset();

        let mut block = bl::TextBlock::new(pos.j + 1, pos.i + 1, size.w - 3, size.h - 3);
        let r = block.print(&tb);
        if fail_on_out_of_space {
            r.debug_unwrap();
        }
        block.render(&mut self.console);
    }

    /// Position given is the position of the top left corner of the rectangle
    /// Button has height 3, width is the width of the string plus 2.
    pub fn button(&mut self, text: &str, pos: Pos, background: bl::RGBA) -> bool {
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
            background
        };
        self.draw_box_color(rect, white(), bg);
        self.console
            .print_color(pos.j + 1, pos.i + 1, white(), bg, text);
        was_clicked
    }

    /// Draw a text box, with text and a few buttons, returns the clicked button
    pub fn box_with_options(
        &mut self,
        title: &str,
        text: &str,
        rect: Rect,
        buttons: &[&str],
        last_selected: &mut usize,
    ) -> Option<usize> {
        self.text_box(title, text, rect, true);
        let mut ans = None;
        *last_selected = buttons.len().min(*last_selected);
        if self.pressed_key == Some(Key::Up) {
            *last_selected = (*last_selected + buttons.len() - 1) % buttons.len();
        } else if self.pressed_key == Some(Key::Down) {
            *last_selected = (*last_selected + 1) % buttons.len();
        }
        let cursor_on = ((self.time.as_millis() / 500) % 2) == 0;
        for (i, txt) in buttons.iter().enumerate() {
            let pi = rect.bottom() - 3 * (buttons.len() - i) as i32;
            if cursor_on && i == *last_selected {
                self.char(Pos::new(pi + 1, rect.left() + 1), '>');
            }
            if self.button(*txt, Pos::new(pi, rect.left() + 3), black())
                || (i == *last_selected && self.pressed_key == Some(Key::Return))
            {
                ans = Some(i);
            }
        }
        ans
    }

    #[allow(dead_code)]
    pub fn char(&mut self, pos: Pos, c: char) {
        self.console
            .set(pos.j, pos.i, white(), black(), bl::to_cp437(c));
    }

    pub fn print(&mut self, pos: Pos, text: &str) { self.console.print(pos.j, pos.i, text); }

    pub fn instructions(&mut self, texts: &[&str]) {
        let size = texts.len();
        texts.into_iter().enumerate().for_each(|(i, txt)| {
            self.console
                .print_right(W - 1, H - 2 * (size - i) as i32, *txt)
        });
    }
}
