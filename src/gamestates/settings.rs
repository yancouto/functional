use super::base::*;
use crate::{
    audio::set_volume, prelude::*, save_system::{edit_and_save, load_common, CommonConfig}, utils::vec_with_cursor::VecWithCursor
};

pub struct SettingsState {
    items: VecWithCursor<SettingsItem>,
}

impl SettingsState {
    pub fn new() -> Self {
        let common = load_common();

        Self {
            items: vec1![SettingsItem::Volume {
                current: common.volume,
            }]
            .into(),
        }
    }
}

enum SettingsItem {
    Volume { current: u8 },
}

impl SettingsItem {
    fn draw(&self, i: i32, data: &mut TickData) {
        // TODO: If adding more items, print something next to the item showing it is selected
        match self {
            Self::Volume { current } => {
                data.print(Pos::new(i, 3), &format!("<  Volume: {}  >", current));
            },
        }
    }

    // Called only when selected
    fn update(&mut self, data: &mut TickData) {
        match self {
            Self::Volume { current } => {
                let prev = *current;
                match data.pressed_key {
                    Some(Key::Left) => *current = (*current).max(1) - 1,
                    Some(Key::Right) => *current = (*current).min(9) + 1,
                    _ => {},
                }
                if prev != *current {
                    set_volume(*current);
                    SFX::Confirm.play();
                }
            },
        }
    }

    fn save(&self, config: &mut CommonConfig) {
        match self {
            Self::Volume { current } => {
                set_volume(*current);
                config.volume = *current;
            },
        }
    }
}

impl GameState for SettingsState {
    fn name(&self) -> &'static str { "Settings" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        let mut i = 1;
        for item in self.items.inner() {
            i += 3;
            item.draw(i, &mut data);
        }
        self.items.get_mut().update(&mut data);

        data.instructions(&["Press ESC to go back", "Use LEFT and RIGHT to tune options"]);

        match data.pressed_key {
            Some(Key::Up) => self.items.cursor_decrement(),
            Some(Key::Down) => self.items.cursor_increment(),
            Some(Key::Escape) => {
                edit_and_save(|c| self.items.inner().iter().for_each(|i| i.save(c)));
                SFX::Back.play();
                return GameStateEvent::Pop(1);
            },
            _ => {},
        }

        GameStateEvent::None
    }
}
