use super::{main_menu::MainMenuState, profile_selection::try_load_default_profile};
use crate::{gamestates::base::*, prelude::*, save_system};

#[derive(Debug)]
pub struct SaveLoaderState {
    user:          String,
    err_text:      String,
    last_selected: usize,
}

impl SaveLoaderState {
    pub fn try_load(user: String) -> Box<dyn GameState> {
        let profile = save_system::load_profile(&user);
        match profile {
            Ok(p) => Box::new(MainMenuState::new(Arc::new(p), true)),
            // Save corrupted
            Err(err) => {
                SFX::Wrong.play();
                Box::new(Self {
                user,
                err_text: format!(
                    "Got the following error:\n{}\n\n These are your options:\n\n
                    - Reset save: Level complete/score data will be reset, but level code will not.\n\n
                    - Go back to profile selection: Doesn't change this save, you can select another user. \
                    If you're feeling adventureous, you might want to try to fix the save yourself.",
                    err
                ),
                last_selected: 0,
            })
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
            &mut self.last_selected,
        ) {
            SFX::Confirm.play();
            GameStateEvent::Switch(if i == 0 {
                save_system::reset_profile(&self.user);
                Self::try_load(self.user.clone())
            } else {
                save_system::edit_and_save(|c| c.default_profile.take());
                try_load_default_profile()
            })
        } else {
            GameStateEvent::None
        }
    }
}
