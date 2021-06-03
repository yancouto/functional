use super::{main_menu::MainMenuState, profile_selection::try_load_default_profile};
use crate::{gamestates::base::*, prelude::*, save_system};

#[derive(Debug)]
pub struct SaveLoaderState {
    user:     String,
    err_text: String,
}

impl SaveLoaderState {
    pub fn try_load(user: String) -> Box<dyn GameState> {
        let profile = save_system::load_profile(&user);
        match profile {
            Ok(p) => box MainMenuState::new(Arc::new(p)),
            // Save corrupted
            Err(err) => box Self {
                user,
                err_text: format!(
                    "Got the following error:\n{}\n\n These are your options:\n\n
                    - Reset save: Level complete/score data will be reset, but level code will not.\n\n
                    - Go back to profile selection: Doesn't change this save, you can select another user. \
                    If you're feeling adventureous, you might want to try to fix the save yourself.",
                    err
                ),
            },
        }
    }
}

impl GameState for SaveLoaderState {
    fn name(&self) -> &'static str { "SaveLoader" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        if let Some(i) = data.box_with_options(
            &format!("Loading failed for user {}", self.user),
            &self.err_text,
            Rect::centered(70, 35),
            &["Reset save", "Go back to profile selection"],
        ) {
            GameStateEvent::Switch(if i == 0 {
                save_system::reset_profile(&self.user);
                Self::try_load(self.user.clone())
            } else {
                save_system::write_common(Default::default());
                try_load_default_profile()
            })
        } else {
            GameStateEvent::None
        }
    }
}
