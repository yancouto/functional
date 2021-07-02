use std::{convert::TryInto, path::PathBuf};

use thiserror::Error;

use super::{
    base::*, editor::EditorState, level_creator::{ParsedUserLevelConfig, ValidationError, LEVEL_FILE}
};
use crate::{
    drawables::XiEditor, levels::{Level, UserCreatedLevel}, prelude::*, save_system::SaveProfile, utils::vec_with_cursor::VecWithCursor
};

type MaybeLevel = Result<Arc<UserCreatedLevel>, (u64, LoadLevelError)>;

pub struct UserCreatedLevelsState {
    save_profile: Arc<SaveProfile>,
    levels:       Option<VecWithCursor<MaybeLevel>>,
}

#[derive(Debug, Error)]
enum LoadLevelError {
    #[error("Item is not installed. Steam should install it automatically.")]
    ItemNotInstalled,
    #[error("Filesystem error: {0}")]
    FilesystemError(#[from] std::io::Error),
    #[error("Deserialization error: {0}")]
    Filesystem(#[from] serde_json::Error),
    #[error("Error validating config: {0}")]
    ValidationError(#[from] ValidationError),
}

impl UserCreatedLevelsState {
    #[cfg(feature = "steam")]
    pub fn new(save_profile: Arc<SaveProfile>, client: Option<&SteamClient>) -> Self {
        let levels: Vec<MaybeLevel> = client
            .map(|c| {
                c.ugc()
                    .subscribed_items()
                    .into_iter()
                    .map(|id| {
                        let lvl: Result<UserCreatedLevel, LoadLevelError> = try {
                            let info = c
                                .ugc()
                                .item_install_info(id)
                                .ok_or(LoadLevelError::ItemNotInstalled)?;
                            let json_file =
                                std::fs::File::open(PathBuf::from(&info.folder).join(LEVEL_FILE))?;
                            let uc: ParsedUserLevelConfig = serde_json::from_reader(json_file)?;
                            let mut uc: UserCreatedLevel = uc.try_into()?;
                            // Add the id so leaderboards can work
                            uc.id = Some(id.0);
                            uc
                        };
                        match lvl {
                            Ok(l) => Ok(Arc::new(l)),
                            Err(err) => Err((id.0, err)),
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();
        Self {
            save_profile,
            levels: Vec1::try_from_vec(levels).ok().map(Into::into),
        }
    }

    #[cfg(not(feature = "steam"))]
    pub fn new(save_profile: Arc<SaveProfile>, client: Option<&SteamClient>) -> Self {
        Self {
            save_profile,
            levels: None,
        }
    }
}

fn description(lvl: &MaybeLevel) -> String {
    match lvl {
        Ok(l) => l.base.name.clone(),
        Err((id, err)) => format!("Can't load level {}. {}", id, err),
    }
}

impl GameState for UserCreatedLevelsState {
    fn name(&self) -> &'static str { "User created levels" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        if data.pressed_key == Some(Key::Escape) {
            SFX::Back.play();
            GameStateEvent::Pop(1)
        } else if let Some(levels) = &mut self.levels {
            data.print(Pos::new(2, 2), "User created levels");
            let mut i = 5;
            for lvl in levels.inner() {
                data.print(Pos::new(i, 4), &description(lvl));
                i += 2;
            }
            let cursor_on = ((data.time.as_millis() / 500) % 2) == 0;
            if cursor_on {
                data.char(Pos::new(5 + levels.cursor() as i32 * 2, 2), '>');
            }
            if data.pressed_key == Some(bl::VirtualKeyCode::Return) {
                match levels.get() {
                    Ok(lvl) => {
                        SFX::Select.play();
                        GameStateEvent::Push(box EditorState::<XiEditor>::new(
                            Level::UserCreatedLevel(lvl.clone()),
                            self.save_profile.clone(),
                        ))
                    },
                    Err(_) => {
                        SFX::Wrong.play();
                        GameStateEvent::None
                    },
                }
            } else {
                if data.pressed_key == Some(bl::VirtualKeyCode::Down) {
                    levels.cursor_increment();
                } else if data.pressed_key == Some(bl::VirtualKeyCode::Up) {
                    levels.cursor_decrement();
                }
                GameStateEvent::None
            }
        } else {
            data.print(Pos::new(2, 2), "No levels loaded. Subscribe to levels on Steam workshop, and they will show up here!");
            GameStateEvent::None
        }
    }
}
