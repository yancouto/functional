use crate::math::Pos;
use bracket_lib::prelude as bl;
use std::{borrow::BorrowMut, time::Duration};

struct GSData {
    cur: Box<dyn GameState>,
    time: Duration,
}

pub struct TickData<'a> {
    /// Time since start of gamestate
    pub time: Duration,
    pub console: &'a mut Box<dyn bl::Console>,
    /// Is left mouse button pressed?
    pub left_click: bool,
    /// Was any key pressed this frame?
    pub pressed_key: Option<bl::VirtualKeyCode>,
    /// (i, j)
    pub mouse_pos: Pos,
}

impl<'a> TickData<'a> {
    fn new(data: &GSData, console: &'a mut Box<dyn bl::Console>, ctx: &mut bl::BTerm) -> Self {
        let mouse = pixel_to_char_pos(&ctx, ctx.mouse_pos, &console);
        TickData {
            time: data.time,
            console,
            left_click: ctx.left_click,
            pressed_key: ctx.key,
            mouse_pos: Pos::new(mouse.1, mouse.0),
        }
    }
}

pub struct GameStateManager {
    cur_gs: GSData,
}

// Will we ever need two consoles?
fn with_current_console<F, R>(active_console: usize, f: F) -> R
where
    F: FnOnce(&mut Box<dyn bl::Console>) -> R,
{
    f(&mut bl::BACKEND_INTERNAL.lock().consoles[active_console].console)
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

// COPIED from bracket lib

#[cfg(feature = "curses")]
fn pixel_to_char_pos(&self, pos: (i32, i32), _console: &Box<dyn Console>) -> (i32, i32) {
    pos
}

#[cfg(not(feature = "curses"))]
fn pixel_to_char_pos(
    ctx: &bl::BTerm,
    pos: (i32, i32),
    console: &Box<dyn bl::Console>,
) -> (i32, i32) {
    let max_sizes = console.get_char_size();
    let (scale, center_x, center_y) = console.get_scale();

    // Scaling now works by projecting the mouse position to 0..1 in both dimensions and then
    // multiplying by the console size (with clamping).
    let font_size = (
        ctx.width_pixels as f32 / max_sizes.0 as f32,
        ctx.height_pixels as f32 / max_sizes.1 as f32,
    );
    let offsets = (
        center_x as f32 * font_size.0 * (scale - 1.0),
        center_y as f32 * font_size.1 * (scale - 1.0),
    );

    let w = ctx.width_pixels as f32 * scale;
    let h = ctx.height_pixels as f32 * scale;
    let extent_x = (pos.0 as f32 + offsets.0) / w;
    let extent_y = (pos.1 as f32 + offsets.1) / h;
    let mouse_x = f32::min(extent_x * max_sizes.0 as f32, max_sizes.0 as f32 - 1.0);
    let mouse_y = f32::min(extent_y * max_sizes.1 as f32, max_sizes.1 as f32 - 1.0);

    (i32::max(0, mouse_x as i32), i32::max(0, mouse_y as i32))
}

// END copied from bracket lib
