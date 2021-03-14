use super::base::*;
use bracket_lib::prelude as bl;

#[derive(Debug)]
struct Level(String);

#[derive(Debug)]
struct Section {
    name: String,
    levels: Vec<Level>,
}

#[derive(Debug)]
pub struct LevelSelectionState {
    /// Index of selected section
    section_i: usize,
    sections: Vec<Section>,
    /// Index of selected level inside section
    level_i: Option<usize>,
}

impl LevelSelectionState {
    pub fn new() -> Self {
        let random_levels = |n: usize| {
            (0..n)
                .map(|i| Level(format!("level {}", i)))
                .collect::<Vec<Level>>()
        };
        let l = LevelSelectionState {
            section_i: 0,
            sections: vec![
                Section {
                    name: "basic".to_string(),
                    levels: random_levels(3),
                },
                Section {
                    name: "boolean".to_string(),
                    levels: random_levels(2),
                },
                Section {
                    name: "extra".to_string(),
                    levels: random_levels(5),
                },
                Section {
                    name: "cook's numerals".to_string(),
                    levels: random_levels(10),
                },
            ],
            level_i: None,
        };
        for section in &l.sections {
            if section.name.len() as i32 + CURSOR_I + 2 > MID_I {
                panic!("Too long name");
            }
        }
        l
    }
}

const CURSOR_I: i32 = 3;
const START_J: i32 = 2;
const LINES_PER_SECTION: i32 = 3;
const MID_I: i32 = 40;

impl GameState for LevelSelectionState {
    fn name(&self) -> &'static str {
        "LevelSelection"
    }

    fn tick(&mut self, data: TickData) -> GameStateEvent {
        for (i, section) in self.sections.iter().enumerate() {
            data.console.print(
                CURSOR_I + 2,
                START_J + LINES_PER_SECTION * i as i32,
                &section.name,
            );
        }
        let cursor_on = ((data.time.as_millis() / 500) % 2) == 0;
        if let Some(l_i) = self.level_i {
            for (i, level) in self.sections[self.section_i].levels.iter().enumerate() {
                data.console.print(
                    MID_I + CURSOR_I + 2,
                    START_J + LINES_PER_SECTION * i as i32,
                    &level.0,
                );
            }
        } else {
            data.console.print(
                MID_I + CURSOR_I,
                START_J + LINES_PER_SECTION * self.section_i as i32,
                "press RIGHT to open section",
            );
        }
        if cursor_on {
            data.console.print(
                CURSOR_I + self.level_i.map_or(0, |_| MID_I),
                START_J + LINES_PER_SECTION * self.level_i.unwrap_or(self.section_i) as i32,
                ">",
            );
        }
        GameStateEvent::None
    }

    fn on_event(&mut self, event: bl::BEvent) -> () {
        match event {
            bl::BEvent::KeyboardInput {
                key, pressed: true, ..
            } => match key {
                bl::VirtualKeyCode::Down => {
                    if let Some(li) = self.level_i {
                        self.level_i = Some((li + 1) % self.sections[self.section_i].levels.len());
                    } else {
                        self.section_i = (self.section_i + 1) % self.sections.len();
                    }
                }
                bl::VirtualKeyCode::Up => {
                    if let Some(li) = self.level_i {
                        let len = self.sections[self.section_i].levels.len();
                        self.level_i = Some((li + len - 1) % len);
                    } else {
                        self.section_i =
                            (self.section_i + self.sections.len() - 1) % self.sections.len();
                    }
                }
                bl::VirtualKeyCode::Right if self.level_i.is_none() => {
                    self.level_i = Some(0);
                }
                bl::VirtualKeyCode::Left if self.level_i.is_some() => {
                    self.level_i = None;
                }
                _ => {}
            },
            _ => {}
        }
    }
}
