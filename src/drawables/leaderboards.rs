use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::{gamestates::base::TickData, interpreter::AccStats, prelude::*};

#[derive(Debug)]
enum State {
    Loading,
    Loaded { data_points: HashMap<AccStats, u32> },
}

#[derive(Debug)]
pub struct Leaderboards {
    // whole rect, including borders of box
    location:    Rect,
    player_data: Option<AccStats>,
    state:       State,
}

struct Axis {
    min:  u32,
    step: u32,
    data: HashMap<AccStats, usize>,
}

fn distribute<F: Fn(AccStats) -> u32>(data: &HashMap<AccStats, u32>, get: F, size: u32) -> Axis {
    debug_assert!(size > 1);
    if data.is_empty() {
        return Axis {
            min:  0,
            step: 1,
            data: HashMap::new(),
        };
    }
    let min = data.keys().cloned().map(&get).min().unwrap();
    let max = data.keys().cloned().map(&get).max().unwrap();
    let step = ((max - min + 1) + size - 1) / size;
    Axis {
        min,
        step,
        data: data
            .keys()
            .map(|a| (*a, ((get(*a) - min) / step) as usize))
            .collect(),
    }
}

/// Return a number such that a few numbers are greater than that in the given matrix.
/// But not a lot. Used to display "common" values.
fn get_common(data: &Vec<Vec<u32>>) -> u32 {
    let mut tot = 0;
    let mut freqs = BTreeMap::new();
    for v in data {
        for val in v {
            if *val > 0 {
                *freqs.entry(*val).or_insert(0u32) += 1;
                tot += 1;
            }
        }
    }
    let mut ans = u32::MAX;
    // Stop when we're in the 25-50% range
    let (min, max, mut cur) = (tot * 25 / 100, tot * 50 / 100, 0);
    loop {
        match freqs.pop_last() {
            Some((val, freq)) =>
                if freq + cur > max {
                    break ans;
                } else {
                    ans = val;
                    cur += freq;
                    if cur >= min {
                        break ans;
                    }
                },
            None => break ans,
        }
    }
}

impl Leaderboards {
    pub fn new(location: Rect, player_data: Option<AccStats>) -> Self {
        Self {
            location,
            player_data,
            state: State::Loaded {
                data_points: vec![(100, 1, 1), (200, 2, 1), (300, 7, 3)]
                    .into_iter()
                    .map(|(x, y, z)| {
                        (
                            AccStats {
                                reductions_x100: x,
                                functions:       y,
                            },
                            z,
                        )
                    })
                    .collect(),
            },
        }
    }

    fn draw_ld(&self, data: &mut TickData, data_points: &HashMap<AccStats, u32>) {
        let off_i = 7;
        let off_j = 7;
        let size_w = self.location.size.w - 2 - off_i - 1;
        let size_h = self.location.size.h - 2 - off_j - 1;
        let x_axis = distribute(&data_points, |a| a.functions.into(), size_w as u32);
        let y_axis = distribute(&data_points, |a| a.reductions_x100, size_h as u32);
        let mut nums = vec![vec![0; size_w as usize]; size_h as usize];
        for (stat, freq) in data_points {
            if let Some(v) = nums
                .get_mut(y_axis.data[stat])
                .and_then(|x| x.get_mut(x_axis.data[stat]))
            {
                *v += freq;
            } else {
                debug_assert!(false);
            }
        }
        let (pi, pj) = if let Some(a) = &self.player_data {
            (y_axis.data[a] as i32, x_axis.data[a] as i32)
        } else {
            // invalid numbers
            (-1, -1)
        };
        let common = get_common(&nums);
        let Pos { i: top, j: left } = self.location.pos;
        let pos = |i, j| Pos {
            i: top + 2 + size_h - i,
            j: left + 1 + off_j + j,
        };
        for i in 0..size_h {
            for j in 0..size_w {
                let amt = nums[i as usize][j as usize];
                data.char(
                    pos(i, j),
                    if amt == 0 {
                        ' '
                    } else if (i, j) == (pi, pj) {
                        'x'
                    } else if amt >= common {
                        '•'
                    } else {
                        '·'
                    },
                );
            }
        }
        for i in 0..size_h {
            data.char(pos(i, -1), '│');
            if i < size_h - 1 && (i % 2) == 0 {
                let val = format!(
                    "{:.2}",
                    (y_axis.min as f32 + y_axis.step as f32 * i as f32) / 100.0
                );
                data.print(pos(i, -2 - val.len() as i32), &val);
            }
        }
        let mut last = -1;
        for j in 0..size_w {
            data.char(pos(-1, j), '─');
            if j > last {
                let val = (x_axis.min + x_axis.step * j as u32).to_string();
                if j + val.len() as i32 > size_w {
                    continue;
                }
                data.char(pos(-2, j), '↑');
                data.print(pos(-3, j), &val);
                last = j + val.len() as i32;
            }
        }
        data.char(pos(-1, -1), '└');
        data.char(pos(-1, size_w), '→');
        data.char(pos(size_h, -1), '↑');
        data.print(pos(size_h, -6), "reds");
        data.console.print_centered_at(
            pos(0, size_w / 2).j,
            self.location.bottom() - 1,
            "functions",
        );
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_distribute() {
        let with_vec = |v: Vec<u32>, size| {
            let stats: Vec<AccStats> = v
                .into_iter()
                .map(|x| AccStats {
                    reductions_x100: x,
                    functions:       0,
                })
                .collect();
            let ax = distribute(
                &stats.iter().map(|s| (*s, 1)).collect(),
                |a| a.reductions_x100,
                size,
            );
            (
                ax.min,
                ax.step,
                stats
                    .iter()
                    .map(|s| *ax.data.get(s).unwrap())
                    .collect::<Vec<_>>(),
            )
        };
        assert_eq!(with_vec(vec![1, 2, 3, 4], 4), (1, 1, vec![0, 1, 2, 3]));
        assert_eq!(with_vec(vec![1, 2, 3, 4], 30), (1, 1, vec![0, 1, 2, 3]));
        assert_eq!(with_vec(vec![1, 2, 3, 4], 2), (1, 2, vec![0, 0, 1, 1]));
        assert_eq!(with_vec(vec![1, 100, 102, 200], 20).2, vec![0, 9, 10, 19]);
    }

    #[test]
    fn test_get_common() {
        assert_eq!(get_common(&vec![vec![3, 0, 0, 3, 3, 0, 3]]), u32::MAX);
        assert_eq!(get_common(&vec![vec![0, 0, 1, 1, 2]]), 2);
        assert_eq!(get_common(&vec![vec![0, 0, 1, 1, 2, 3, 3]]), 3);
        assert_eq!(get_common(&vec![vec![1, 1, 1, 1, 1, 1, 2, 3]]), 2);
    }
}
