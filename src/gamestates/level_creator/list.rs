use std::path::PathBuf;

use crossbeam::channel::Receiver;

use super::super::base::*;
use crate::{
    drawables::{black, XiEditor}, gamestates::{level_creator, string_reader::StringReaderState}, prelude::*, save_system::{SaveProfile, PROJECT_DIR}, utils::vec_with_cursor::VecWithCursor
};

#[derive(Debug)]
pub struct LevelCreatorLevelListState {
    root:         PathBuf,
    levels:       Option<VecWithCursor<String>>,
    title_recv:   Option<Receiver<Option<String>>>,
    save_profile: Arc<SaveProfile>,
}

const CUSTOM_LEVELS: &str = "custom_levels";

impl LevelCreatorLevelListState {
    pub fn new(save_profile: Arc<SaveProfile>) -> Self {
        let root = PROJECT_DIR.data_dir().join(CUSTOM_LEVELS);
        log::info!("Looking for custom levels on {:?}", root);
        std::fs::create_dir_all(&root).debug_expect("Failed to create custom_levels dir");
        let mut this = Self {
            root,
            levels: None,
            title_recv: None,
            save_profile,
        };
        this.reload();
        this
    }

    fn reload(&mut self) {
        let mut new_vec = Vec::new();
        std::fs::read_dir(&self.root)
            .map(|dir| {
                for f in dir {
                    if let Ok(entry) = f {
                        if entry.file_type().map(|t| t.is_dir()).debug_unwrap_or(false) {
                            new_vec.push(entry.file_name().to_string_lossy().into_owned());
                        }
                    }
                }
            })
            .debug_unwrap();
        self.levels = Vec1::try_from_vec(new_vec).ok().map(|v| v.into());
    }

    fn go_to_level(&mut self, name: &str) -> GameStateEvent {
        SFX::Select.play();
        let dir = self.root.join(name);
        GameStateEvent::Push(box level_creator::EditorState::<XiEditor>::new(
            dir,
            self.save_profile.clone(),
        ))
    }

    fn create_level(&mut self, title: String) -> GameStateEvent {
        if !self
            .levels
            .as_ref()
            .map(|v| v.inner().iter().any(|s| *s == title))
            .unwrap_or(false)
        {
            let dir = self.root.join(&title);
            std::fs::create_dir_all(&dir).expect("Failed to create directory for level");
            self.reload();
        }
        self.go_to_level(&title)
    }
}

const CURSOR_J: i32 = 2;
const START_I: i32 = 3;
const STEP_I: i32 = 2;

impl GameState for LevelCreatorLevelListState {
    fn name(&self) -> &'static str { "LevelCreatorList" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        if let Some(vec) = &self.levels {
            let cursor_on = ((data.time.as_millis() / 500) % 2) == 0;
            let mut i = START_I;
            for (idx, level) in vec.inner().iter().enumerate() {
                data.print(Pos::new(i, CURSOR_J + 2), &level);
                if idx == vec.cursor() && cursor_on {
                    data.char(Pos::new(i, CURSOR_J), '>');
                }
                i += STEP_I;
            }
        } else {
            data.print(Pos::new(2, 2), "No custom levels, try creating one.");
        }

        data.instructions(&[
            "Use UP/DOWN to navigate levels",
            "Press the button or CTRL+N to create a new level",
            "Press ESC to go back",
        ]);

        if data.button("Create new level", Pos::new(H - 4, 1), black())
            || (data.ctrl && data.pressed_key == Some(Key::N))
        {
            let (state, recv) = StringReaderState::new("Level title".to_string(), 15);
            self.title_recv = Some(recv);
            return GameStateEvent::Push(box state);
        }
        // Receiver should be ready after one tick, so we can steal the option
        if let Some(title) = self.title_recv.take().and_then(|r| r.try_recv().ok()) {
            title
                .map(|t| self.create_level(t))
                .unwrap_or(GameStateEvent::None)
        } else if data.pressed_key == Some(Key::Return) {
            self.levels
                .as_ref()
                .map(|v| v.get().clone())
                .map(|name| self.go_to_level(&name))
                .unwrap_or(GameStateEvent::None)
        } else if data.pressed_key == Some(Key::Escape) {
            SFX::Back.play();
            GameStateEvent::Pop(1)
        } else {
            match self.levels.as_mut().zip(data.pressed_key) {
                Some((v, Key::Up)) => v.cursor_decrement(),
                Some((v, Key::Down)) => v.cursor_increment(),
                _ => {},
            }
            GameStateEvent::None
        }
    }
}
