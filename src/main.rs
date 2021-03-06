use bracket_lib::prelude as bl;

struct State;

impl bl::GameState for State {
    fn tick(&mut self, ctx: &mut bl::BTerm) {
        ctx.cls();
        ctx.print(1, 1, "this is functional");
    }
}

fn main() -> bl::BError {
    let ctx = bl::BTermBuilder::simple80x50()
        .with_title("functional")
        .build()?;
    let gs = State;
    bl::main_loop(ctx, gs)
}
