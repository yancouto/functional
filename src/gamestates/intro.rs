use bracket_lib::prelude as bl;

pub struct IntroState;

impl bl::GameState for IntroState {
    fn tick(&mut self, ctx: &mut bl::BTerm) {
        ctx.cls();
        ctx.print(10, 10, "this is functional");
    }
}
