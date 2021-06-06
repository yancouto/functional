use std::{collections::HashSet, time::Duration};

use crate::prelude::*;

pub struct GSData {
    pub cur:  Box<dyn GameState>,
    pub time: Duration,
}

#[derive(Default)]
pub struct EventTickData {
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
    /// Steam client
    pub steam_client: Option<Arc<SteamClient>>,
    ctx:              &'a mut bl::BTerm,
}

impl<'a> TickData<'a> {
    pub fn new(
        data: &GSData,
        event_data: EventTickData,
        console: &'a mut Box<dyn bl::Console>,
        ctx: &'a mut bl::BTerm,
        input: &'a bl::Input,
        steam_client: Option<Arc<SteamClient>>,
    ) -> Self {
        TickData {
            time: data.time,
            console,
            pressed_key: ctx.key,
            mouse_pos: Pos::from_xy(input.mouse_tile_pos(ctx.active_console)),
            left_click: event_data.left_click,
            ctrl: ctx.control,
            keys_pressed: input.key_pressed_set(),
            steam_client,
            ctx,
        }
    }

    pub fn quit(&mut self) { self.ctx.quit(); }
}

#[cfg(feature = "steam")]
pub type SteamClient = steamworks::Client;

#[cfg(not(feature = "steam"))]
pub type SteamClient = ();

pub struct GameStateManager {
    all_gs:       Vec1<GSData>,
    steam_client: Option<Arc<SteamClient>>,
}

// Will we ever need two consoles?
pub fn with_current_console<F, R>(active_console: usize, f: F) -> R
where
    F: FnOnce(&mut Box<dyn bl::Console>) -> R,
{
    f(&mut bl::BACKEND_INTERNAL.lock().consoles[active_console].console)
}

impl GameStateManager {
    pub fn new(first: Box<dyn GameState>, client: Option<SteamClient>) -> Self {
        log::info!("Starting on gamestate {}", first.name());
        let this = Self {
            all_gs:       Vec1::new(GSData {
                cur:  first,
                time: Duration::default(),
            }),
            steam_client: client.map(Arc::new),
        };
        this.entered_gamestate();
        this
    }

    fn process_events(&mut self, ctx: &mut bl::BTerm) -> EventTickData {
        let mut input = bl::INPUT.lock();
        let mut data = EventTickData::default();
        while let Some(e) = input.pop() {
            self.all_gs.last_mut().cur.on_event(e.clone(), &input);
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
        }
        data
    }

    fn entered_gamestate(&self) {
        #[cfg(feature = "steam")]
        if let Some(client) = &self.steam_client {
            let ret = client
                .friends()
                .set_rich_presence("steam_display", Some("#StatusFull"));
            let ret2 = client.friends().set_rich_presence(
                "text",
                Some(&format!("On {}", self.all_gs.last().cur.name())),
            );
            debug_assert!(ret && ret2);
        }
    }

    pub fn tick(&mut self, ctx: &mut bl::BTerm) {
        let event_data = self.process_events(ctx);
        let time_passed = Duration::from_secs_f32(ctx.frame_time_ms / 1000.);
        self.all_gs.last_mut().time += time_passed;
        let event = with_current_console(ctx.active_console, |console| {
            let input = bl::INPUT.lock();
            if self.all_gs.last().cur.clear_terminal() {
                console.cls();
            }
            let tick_data = TickData::new(
                self.all_gs.last(),
                event_data,
                console,
                ctx,
                &input,
                self.steam_client.clone(),
            );
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
                self.entered_gamestate();
            },
            GameStateEvent::Push(new) => {
                log::info!("Pushing gamestate {} to stack", new.name());
                let new = GSData {
                    cur:  new,
                    time: Duration::default(),
                };
                self.all_gs.push(new);
                self.entered_gamestate();
            },
            GameStateEvent::Pop(n) => {
                debug_assert_ne!(n, 0, "Can't pop 0 gamestates, use None.");
                for _ in 0..n.into() {
                    match self.all_gs.pop() {
                        Err(_) => {
                            log::error!("Trying to pop only gamestate, ignoring.");
                            debug_unreachable!();
                        },
                        Ok(gs) => log::info!("Popped gamestate {}", gs.cur.name()),
                    }
                }
                self.entered_gamestate();
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
    /// Remove this many gamestates and go back to previous. Ignored if there are not
    /// enough gamestates to pop.
    Pop(u8),
}

pub trait GameState {
    fn name(&self) -> &'static str;
    fn tick(&mut self, data: TickData) -> GameStateEvent;
    fn on_event(&mut self, _event: bl::BEvent, _input: &bl::Input) {}
    fn clear_terminal(&self) -> bool { true }
}
