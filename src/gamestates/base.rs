use bracket_lib::prelude as bl;
use std::{borrow::BorrowMut, time::Duration};

struct GSData {
    cur: Box<dyn GameState>,
    time: Duration,
}

pub struct TickData<'a> {
    /// Time since start of gamestate
    pub time: Duration,
    pub console: &'a mut dyn bl::Console,
    /// Is left mouse button pressed?
    pub left_click: bool,
    /// Was any key pressed this frame?
    pub pressed_key: Option<bl::VirtualKeyCode>,
}

impl<'a> TickData<'a> {
    fn new(data: &GSData, console: &'a mut dyn bl::Console, ctx: &mut bl::BTerm) -> Self {
        TickData {
            time: data.time,
            console,
            left_click: ctx.left_click,
            pressed_key: ctx.key,
        }
    }
}

pub struct GameStateManager {
    cur_gs: GSData,
}

// Will we ever need two consoles?
fn with_current_console<F, R>(active_console: usize, f: F) -> R
where
    F: FnOnce(&mut dyn bl::Console) -> R,
{
    f(bl::BACKEND_INTERNAL.lock().consoles[active_console]
        .console
        .as_mut())
}

impl GameStateManager {
    pub fn new(first: Box<dyn GameState>) -> Self {
        bl::INPUT.lock().activate_event_queue();
        println!("Starting on gamestate {}", first.name());
        Self {
            cur_gs: GSData {
                cur: first,
                time: Duration::default(),
            },
        }
    }

    fn process_events(&mut self, ctx: &mut bl::BTerm) {
        let mut input = bl::INPUT.lock();
        while let Some(e) = input.pop() {
            // Blib stops tracking close events when we activate event queue
            if let bl::BEvent::CloseRequested = e {
                ctx.quit();
            } else {
                self.cur_gs.cur.on_event(e);
            }
        }
    }

    pub fn tick(&mut self, ctx: &mut bl::BTerm) {
        self.process_events(ctx);
        self.cur_gs.time += Duration::from_secs_f32(ctx.frame_time_ms / 1000.);
        let event = with_current_console(ctx.active_console, |console| {
            console.cls();
            self.cur_gs
                .cur
                .tick(TickData::new(&self.cur_gs, console, ctx))
        });
        match event {
            GameStateEvent::None => {}
            GameStateEvent::Switch(new) => {
                println!(
                    "Switching gamestate from {} to {}",
                    self.cur_gs.cur.name(),
                    new.name()
                );
                self.cur_gs = GSData {
                    cur: new,
                    time: Duration::default(),
                };
            }
        }
    }
}

pub enum GameStateEvent {
    None,
    Switch(Box<dyn GameState>),
}

pub trait GameState {
    fn name(&self) -> &'static str;
    fn tick(&mut self, data: TickData) -> GameStateEvent;
    fn on_event(&mut self, _event: bl::BEvent) -> () {}
}
