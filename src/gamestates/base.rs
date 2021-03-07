use bracket_lib::prelude as bl;

#[derive(Debug)]
pub struct GameStateManager {
    cur_gs: Box<dyn GameState>,
}

impl GameStateManager {
    pub fn new(first: Box<dyn GameState>) -> Self {
        bl::INPUT.lock().activate_event_queue();
        println!("Starting on gamestate {}", first.name());
        Self { cur_gs: first }
    }

    fn process_events(&mut self, ctx: &mut bl::BTerm) {
        let mut input = bl::INPUT.lock();
        while let Some(e) = input.pop() {
            // Blib stops tracking close events when we activate event queue
            if let bl::BEvent::CloseRequested = e {
                ctx.quit();
            } else {
                self.cur_gs.on_event(e);
            }
        }
    }

    pub fn tick(&mut self, ctx: &mut bl::BTerm) {
        self.process_events(ctx);
        ctx.cls();
        match self.cur_gs.tick(ctx) {
            GameStateEvent::None => {}
            GameStateEvent::Switch(new) => {
                println!(
                    "Switching gamestate from {} to {}",
                    self.cur_gs.name(),
                    new.name()
                );
                self.cur_gs = new;
            }
        }
    }
}

pub enum GameStateEvent {
    None,
    Switch(Box<dyn GameState>),
}

pub trait GameState: std::fmt::Debug {
    fn name(&self) -> &'static str;
    fn tick(&mut self, ctx: &mut bl::BTerm) -> GameStateEvent;
    fn on_event(&mut self, _event: bl::BEvent) -> () {}
}
