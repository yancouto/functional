use bracket_lib::prelude as bl;

pub fn text_box(
    console: &mut Box<dyn bl::Console>,
    title: &str,
    text: &str,
    i: i32,
    j: i32,
    w: i32,
    h: i32,
) {
    console.draw_box(
        j,
        i,
        w,
        h,
        bl::RGBA::named(bl::WHITE),
        bl::RGBA::named(bl::BLACK),
    );
    console.print(j + 1, i, title);
    let mut tb = bl::TextBuilder::empty();
    // TODO: support \n's
    tb.ln().line_wrap(text).reset();

    let mut block = bl::TextBlock::new(j + 1, i + 1, w - 2, h - 2);
    block.print(&tb).unwrap();
    block.render(console);
}
