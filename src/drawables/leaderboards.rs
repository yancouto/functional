use std::collections::{BTreeSet, HashMap, HashSet};

use crate::{gamestates::base::TickData, interpreter::AccStats, prelude::*};

#[derive(Debug)]
enum State {
    Loading,
    Loaded { data_points: HashSet<AccStats> },
}

#[derive(Debug)]
pub struct Leaderboards {
    location:    Rect,
    player_data: Option<AccStats>,
    state:       State,
}

fn distribute<F: Fn(AccStats) -> u32>(
    data: &HashSet<AccStats>,
    get: F,
    size: u32,
) -> HashMap<AccStats, u32> {
    if data.is_empty() {
        return HashMap::new();
    }
    let min = data.iter().cloned().map(&get).min().unwrap();
    let max = data.iter().cloned().map(&get).max().unwrap();
    let step = (max - min + 1).max(size) / size;
    data.iter().map(|a| (*a, (get(*a) - min) / step)).collect()
}

impl Leaderboards {
    pub fn new(location: Rect, player_data: Option<AccStats>) -> Self {
        Self {
            location,
            player_data,
            state: State::Loading,
        }
    }

    fn draw_ld(&self, data: &mut TickData, data_points: &HashSet<AccStats>) {
        let off_i = 7;
        let off_j = 7;
        let size_w = self.location.size.w - 2 - off_i;
        let size_h = self.location.size.h - 2 - off_j;
        let x_axis = distribute(&data_points, |a| a.functions.into(), size_w as u32);
        let y_axis = distribute(&data_points, |a| a.reductions_x100, size_h as u32);
    }

    pub fn draw(&self, data: &mut TickData) {
        data.title_box("Leaderboards", self.location);
        match &self.state {
            State::Loading => data.print(
                Pos::new(self.location.pos.i + 2, self.location.left() + 2),
                "Coming soon...",
            ),
            State::Loaded { data_points } => self.draw_ld(data, data_points),
        }
    }
}
