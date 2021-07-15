use super::{
    base::*, level_creator::LevelCreatorLevelListState, level_selection::LevelSelectionState, profile_selection::ProfileSelectionState, settings::SettingsState, user_created_levels::UserCreatedLevelsState
};
use crate::{
    drawables::XiEditor, gamestates::playground::PlaygroundState, interpreter::ConstantProvider, prelude::*, save_system::SaveProfile, utils::vec_with_cursor::VecWithCursor
};
enum MenuItem {
    Play,
    Settings,
    LevelCreator,
    UserCreatedLevels,
    ChangeProfile,
    Playground,
    Quit,
}

impl MenuItem {
    fn name(&self) -> &'static str {
        match self {
            MenuItem::Play => "play",
            MenuItem::Settings => "settings",
            MenuItem::ChangeProfile => "change profile",
            MenuItem::Playground => "playground",
            MenuItem::Quit => "quit game",
            MenuItem::LevelCreator =>
                if !cfg!(feature = "demo") {
                    "level creator"
                } else {
                    "level creator (FULL GAME ONLY)"
                },
            MenuItem::UserCreatedLevels =>
                if !cfg!(feature = "demo") {
                    "user created levels"
                } else {
                    "user created levels (FULL GAME ONLY)"
                },
        }
    }

    fn on_click(&self, menu: &MainMenuState, data: &mut TickData) -> GameStateEvent {
        SFX::Select.play();
        match self {
            MenuItem::Play =>
                GameStateEvent::Switch(box LevelSelectionState::new(menu.save_profile.clone())),
            MenuItem::Settings => GameStateEvent::Push(box SettingsState::new()),
            MenuItem::ChangeProfile => GameStateEvent::Switch(box ProfileSelectionState::new()),
            MenuItem::Playground => GameStateEvent::Push(box PlaygroundState::<XiEditor>::new(
                String::new(),
                ConstantProvider::all(),
            )),
            #[cfg(not(feature = "demo"))]
            MenuItem::LevelCreator => GameStateEvent::Push(box LevelCreatorLevelListState::new(
                menu.save_profile.clone(),
            )),
            MenuItem::Quit => {
                data.quit();
                GameStateEvent::None
            },
            #[cfg(not(feature = "demo"))]
            MenuItem::UserCreatedLevels => GameStateEvent::Push(box UserCreatedLevelsState::new(
                menu.save_profile.clone(),
                data.steam_client.as_deref(),
            )),
            #[cfg(feature = "demo")]
            _ => {
                SFX::Wrong.play();
                GameStateEvent::None
            },
        }
    }
}

pub struct MainMenuState {
    items:               VecWithCursor<MenuItem>,
    save_profile:        Arc<SaveProfile>,
    reload_achievements: bool,
}

impl MainMenuState {
    pub fn new(save_profile: Arc<SaveProfile>, reload_achievements: bool) -> Self {
        Self {
            items: vec1![
                MenuItem::Play,
                MenuItem::Playground,
                MenuItem::UserCreatedLevels,
                MenuItem::LevelCreator,
                MenuItem::Settings,
                MenuItem::ChangeProfile,
                MenuItem::Quit
            ]
            .into(),
            save_profile,
            reload_achievements,
        }
    }
}

const CURSOR_J: i32 = 3;
const START_I: i32 = 6;
const LINES_PER_SECTION: i32 = 3;

impl GameState for MainMenuState {
    fn name(&self) -> &'static str { "MainMenu" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        if self.reload_achievements {
            self.reload_achievements = false;
            #[cfg(feature = "steam")]
            if let Some(client) = data.steam_client.clone() {
                crate::utils::steam::update_all_achievements(client, self.save_profile.clone());
            }
        }
        data.print(
            Pos::new(2, CURSOR_J),
            &format!("Hello, {}.", self.save_profile.name()),
        );
        data.instructions(&[
            "Use UP/DOWN to navigate options",
            "Press ENTER to choose option",
        ]);
        for (i, item) in self.items.inner().iter().enumerate() {
            data.print(
                Pos::new(START_I + LINES_PER_SECTION * i as i32, CURSOR_J + 2),
                item.name(),
            );
        }
        let cursor_on = ((data.time.as_millis() / 500) % 2) == 0;
        if cursor_on {
            data.print(
                Pos::new(
                    START_I + LINES_PER_SECTION * self.items.cursor() as i32,
                    CURSOR_J,
                ),
                ">",
            );
        }
        if data.pressed_key == Some(bl::VirtualKeyCode::Return) {
            self.items.get().on_click(&self, &mut data)
        } else {
            if data.pressed_key == Some(bl::VirtualKeyCode::Down) {
                self.items.cursor_increment();
            } else if data.pressed_key == Some(bl::VirtualKeyCode::Up) {
                self.items.cursor_decrement();
            }
            GameStateEvent::None
        }
    }
}
