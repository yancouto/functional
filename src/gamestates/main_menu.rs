use super::{
    base::*, level_creator::LevelCreatorLevelListState, level_selection::LevelSelectionState, profile_selection::ProfileSelectionState
};
use crate::{
    drawables::XiEditor, gamestates::playground::PlaygroundState, interpreter::ConstantProvider, prelude::*, save_system::SaveProfile, utils::vec_with_cursor::VecWithCursor
};

enum MenuItem {
    Play,
    Settings,
    LevelCreator,
    ChangeProfile,
    Playground,
    Quit,
}

impl MenuItem {
    fn name(&self) -> &'static str {
        match self {
            MenuItem::Play => "play",
            MenuItem::Settings => "settings (TODO!)",
            MenuItem::ChangeProfile => "change profile",
            MenuItem::Playground => "playground",
            MenuItem::Quit => "quit game",
            MenuItem::LevelCreator => "level creator",
        }
    }

    fn on_click(&self, menu: &MainMenuState, data: &mut TickData) -> GameStateEvent {
        match self {
            MenuItem::Play =>
                GameStateEvent::Switch(box LevelSelectionState::new(menu.save_profile.clone())),
            MenuItem::Settings => GameStateEvent::None,
            MenuItem::ChangeProfile => GameStateEvent::Switch(box ProfileSelectionState::new()),
            MenuItem::Playground => GameStateEvent::Push(box PlaygroundState::<XiEditor>::new(
                String::new(),
                ConstantProvider::all(),
            )),
            MenuItem::LevelCreator => GameStateEvent::Push(box LevelCreatorLevelListState::new()),
            MenuItem::Quit => {
                data.quit();
                GameStateEvent::None
            },
        }
    }
}

pub struct MainMenuState {
    items:        VecWithCursor<MenuItem>,
    save_profile: Arc<SaveProfile>,
}

impl MainMenuState {
    pub fn new(save_profile: Arc<SaveProfile>) -> Self {
        Self {
            items: vec1![
                MenuItem::Play,
                MenuItem::Playground,
                MenuItem::LevelCreator,
                MenuItem::Settings,
                MenuItem::ChangeProfile,
                MenuItem::Quit
            ]
            .into(),
            save_profile,
        }
    }
}

const CURSOR_J: i32 = 3;
const START_I: i32 = 6;
const LINES_PER_SECTION: i32 = 3;

impl GameState for MainMenuState {
    fn name(&self) -> &'static str { "MainMenu" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
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
