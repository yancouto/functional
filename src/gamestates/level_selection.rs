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
    section_cur: usize,
    sections: Vec<Section>,
}

impl LevelSelectionState {
    pub fn new() -> Self {
        let random_levels = |n: usize| {
            (0..n)
                .map(|i| Level(format!("level {}", i)))
                .collect::<Vec<Level>>()
        };
        LevelSelectionState {
            section_cur: 0,
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
        }
    }
}

const CURSOR_I: i32 = 3;
const START_J: i32 = 2;
const LINES_PER_SECTION: i32 = 3;

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
        if cursor_on {
            data.console.print(
                CURSOR_I,
                START_J + LINES_PER_SECTION * self.section_cur as i32,
                ">",
            );
        }
        GameStateEvent::None
    }

    fn on_event(&mut self, event: bl::BEvent) -> () {
        match event {
            bl::BEvent::KeyboardInput {
                key: bl::VirtualKeyCode::Down,
                pressed: true,
                ..
            } => {
                self.section_cur = (self.section_cur + 1) % self.sections.len();
            }
            bl::BEvent::KeyboardInput {
                key: bl::VirtualKeyCode::Up,
                pressed: true,
                ..
            } => {
                self.section_cur =
                    (self.section_cur + self.sections.len() - 1) % self.sections.len();
            }
            _ => {}
        }
    }
}
