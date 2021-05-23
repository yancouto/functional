use crossbeam::channel;

use super::LeaderboardLoadError;
use crate::{gamestates::base::TickData, interpreter::AccStats, prelude::*};

#[cfg(feature = "steam")]
pub type FriendResult = Result<Vec<steamworks::LeaderboardEntry>, LeaderboardLoadError>;

#[cfg(not(feature = "steam"))]
pub type FriendResult = ();

#[derive(Debug, Clone)]
struct Entry {
    friend: String,
    stats:  AccStats,
}

#[derive(Debug)]
enum State {
    Waiting,
    Failed(LeaderboardLoadError),
    Success(Vec<Entry>),
}

#[derive(Debug)]
pub struct FriendLeaderboard {
    rect:  Rect,
    send:  channel::Sender<FriendResult>,
    recv:  channel::Receiver<FriendResult>,
    state: State,
}

impl FriendLeaderboard {
    pub fn new(rect: Rect) -> Self {
        let (send, recv) = channel::bounded(2);
        Self {
            rect,
            send,
            recv,
            state: State::Waiting,
        }
    }

    pub fn get_sender(&self) -> channel::Sender<FriendResult> { self.send.clone() }

    fn draw_lb(&self, entries: &Vec<Entry>, data: &mut TickData) {
        let step = if (entries.len() as i32) < (self.rect.size.h - 5) / 2 {
            2
        } else {
            1
        };
        let mut i = self.rect.pos.i + 2;
        let j = self.rect.pos.j + 2;
        data.print(Pos { i, j }, "Name - reductions - functions");
        for entry in &entries[0..entries.len().min(self.rect.size.h as usize - 6)] {
            i += step;
            data.print(
                Pos { i, j },
                &format!(
                    "{} - {:.2} - {}",
                    entry.friend,
                    entry.stats.reductions_x100 as f32 / 100.0,
                    entry.stats.functions
                ),
            );
        }
        if entries.len() > self.rect.size.h as usize - 6 {
            i += step;
            data.print(Pos { i, j }, "...");
        }
    }

    pub fn draw(&mut self, data: &mut TickData) {
        data.text_box(
            "Friends leaderboards",
            &match &self.state {
                State::Waiting if data.steam_client.is_none() =>
                    "Use Steam version of the game for leaderboards".to_string(),
                State::Waiting => "Loading...".to_string(),
                State::Failed(err) => format!("Failed: {}", err),
                State::Success(..) => "".to_string(),
            },
            self.rect,
            true,
        );
        match &self.state {
            State::Waiting => match self.recv.try_recv() {
                Ok(Ok(entries)) => {
                    self.state = State::Success(
                        entries
                            .into_iter()
                            .filter_map(|e| {
                                debug_assert!(e.details.len() == 1);
                                e.details.first().map(|functions| Entry {
                                    friend: format!("{:?}", e.user),
                                    stats:  AccStats {
                                        reductions_x100: e.score as u32,
                                        functions:       *functions as u16,
                                    },
                                })
                            })
                            .collect(),
                    );
                },
                Ok(Err(e)) => {
                    self.state = State::Failed(e);
                },
                Err(channel::TryRecvError::Empty) => {},
                Err(channel::TryRecvError::Disconnected) => {
                    self.state = State::Failed(LeaderboardLoadError::ChannelDisconnected);
                },
            },
            State::Failed(_) => {},
            State::Success(entries) => self.draw_lb(entries, data),
        }
    }
}
