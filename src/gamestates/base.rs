use std::{collections::HashSet, time::Duration};

use crate::prelude::*;

struct GSData {
    cur:  Box<dyn GameState>,
    time: Duration,
}

#[derive(Default)]
struct EventTickData {
    left_click: bool,
}

pub struct TickData<'a> {
    /// Time since start of gamestate
    pub time:         Duration,
    pub console:      &'a mut Box<dyn bl::Console>,
    /// Was the LMB pressed this frame?
    pub left_click:   bool,
    /// Was any key pressed this frame?
    pub pressed_key:  Option<bl::VirtualKeyCode>,
    /// (i, j)
    pub mouse_pos:    Pos,
    /// Is pressing ctrl
    pub ctrl:         bool,
    /// Current keys pressed
    pub keys_pressed: &'a HashSet<bl::VirtualKeyCode>,
}

impl<'a> TickData<'a> {
    fn new(
        data: &GSData,
        event_data: EventTickData,
        console: &'a mut Box<dyn bl::Console>,
        ctx: &mut bl::BTerm,
        input: &'a bl::Input,
    ) -> Self {
        let mouse = pixel_to_char_pos(&ctx, ctx.mouse_pos, &console);
        TickData {
            time: data.time,
            console,
            pressed_key: ctx.key,
            mouse_pos: Pos::new(mouse.1, mouse.0),
            left_click: event_data.left_click,
            ctrl: ctx.control,
            keys_pressed: input.key_pressed_set(),
        }
    }
}

pub struct GameStateManager {
    all_gs: Vec1<GSData>,
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
        log::info!("Starting on gamestate {}", first.name());
        Self {
            all_gs: Vec1::new(GSData {
                cur:  first,
                time: Duration::default(),
            }),
        }
    }

    fn process_events(&mut self, ctx: &mut bl::BTerm) -> EventTickData {
        let mut input = bl::INPUT.lock();
        let mut data = EventTickData::default();
        while let Some(e) = input.pop() {
            parking_lot::MutexGuard::unlocked(&mut input, || {
                self.all_gs.last_mut().cur.on_event(e.clone());
                match e {
                    // Blib stops tracking close events when we activate event queue
                    bl::BEvent::CloseRequested => {
                        ctx.quit();
                    },
                    bl::BEvent::MouseClick {
                        button: 0,
                        pressed: true,
                    } => {
                        data.left_click = true;
                    },
                    _ => {},
                }
            })
        }
        data
    }

    pub fn tick(&mut self, ctx: &mut bl::BTerm) {
        let event_data = self.process_events(ctx);
        let time_passed = Duration::from_secs_f32(ctx.frame_time_ms / 1000.);
        self.all_gs.last_mut().time += time_passed;
        let event = with_current_console(ctx.active_console, |console| {
            let input = bl::INPUT.lock();
            console.cls();
            let tick_data = TickData::new(self.all_gs.last(), event_data, console, ctx, &input);
            self.all_gs.last_mut().cur.tick(tick_data)
        });
        match event {
            GameStateEvent::None => {},
            GameStateEvent::Switch(new) => {
                log::info!(
                    "Switching top gamestate from {} to {}",
                    self.all_gs.last().cur.name(),
                    new.name()
                );
                let new = GSData {
                    cur:  new,
                    time: Duration::default(),
                };
                if self.all_gs.pop().is_err() {
                    // Only a single gamestate
                    self.all_gs = Vec1::new(new);
                } else {
                    self.all_gs.push(new);
                }
            },
            GameStateEvent::Push(new) => {
                log::info!("Pushing gamestate {} to stack", new.name());
                let new = GSData {
                    cur:  new,
                    time: Duration::default(),
                };
                self.all_gs.push(new);
            },
            GameStateEvent::Pop => match self.all_gs.pop() {
                Err(_) => log::error!("Trying to pop only gamestate, ignoring."),
                Ok(gs) => log::info!("Popped gamestate {}", gs.cur.name()),
            },
        }
    }
}

pub enum GameStateEvent {
    /// Don't do anything, continue running this gamestate.
    None,
    /// Remove this gamestate and add a new one in its place.
    Switch(Box<dyn GameState>),
    /// Push a new gamestate on top of this one. Only the top gamestate will be
    /// called tick and on_event.
    Push(Box<dyn GameState>),
    /// Remove this gamestate and go back to previous. Ignored if this is the
    /// only gamestate.
    Pop,
}

pub trait GameState {
    fn name(&self) -> &'static str;
    fn tick(&mut self, data: TickData) -> GameStateEvent;
    fn on_event(&mut self, _event: bl::BEvent) {}
}

// COPIED from bracket lib

#[cfg(feature = "curses")]
fn pixel_to_char_pos(&self, pos: (i32, i32), _console: &Box<dyn Console>) -> (i32, i32) { pos }

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
