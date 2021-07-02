use super::{base::*, save_loader::SaveLoaderState};
use crate::{
    drawables::{BasicTextEditor, TextEditor, *}, prelude::*, save_system::{edit_and_save, get_save_dir, load_common}
};

pub struct ProfileSelectionState {
    editor:         BasicTextEditor,
    known_profiles: Vec<String>,
}

impl ProfileSelectionState {
    pub fn new() -> Self {
        let known_profiles = match get_save_dir().read_dir() {
            Ok(dir) => dir
                .into_iter()
                .filter_map(|maybe_entry| match maybe_entry {
                    Ok(entry) =>
                        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                            entry.file_name().to_str().map(|s| s.to_string())
                        } else {
                            None
                        },
                    Err(_) => None,
                })
                .collect(),
            Err(_) => vec![],
        };
        Self {
            editor: BasicTextEditor::new(
                "Enter profile name:".to_string(),
                Rect::centered(20, 1),
                String::new(),
            ),
            known_profiles,
        }
    }
}

pub fn try_load_default_profile() -> Box<dyn GameState> {
    let common = load_common();
    match common.default_profile {
        Some(user) => SaveLoaderState::try_load(user),
        // Maybe also go to ProfileSelection if the save was deleted
        None => box ProfileSelectionState::new(),
    }
}

enum ValidationError {
    TooLong,
    NonASCII,
    NonAlphaNumeric,
}

impl ValidationError {
    fn str(&self) -> &'static str {
        match self {
            ValidationError::TooLong => "Must be at most 18 digits",
            ValidationError::NonASCII => "Characters must be ASCII",
            ValidationError::NonAlphaNumeric => "Characters must be alphanumeric or _",
        }
    }
}

fn validate(name: &str) -> Option<ValidationError> {
    if name.len() > 18 {
        Some(ValidationError::TooLong)
    } else if !name.is_ascii() {
        Some(ValidationError::NonASCII)
    } else if name.chars().any(|x| !x.is_ascii_alphanumeric() && x != '_') {
        Some(ValidationError::NonAlphaNumeric)
    } else {
        None
    }
}

impl GameState for ProfileSelectionState {
    fn name(&self) -> &'static str { "ProfileSelection" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        self.editor.draw(&mut data);
        data.instructions(&["Press ENTER to create or load profile"]);

        let mut i = H / 2 - self.known_profiles.len() as i32 * 3 / 2 - 3;
        let j = W - 30;
        if !self.known_profiles.is_empty() {
            data.print(Pos::new(i, j), "Known profiles:");
        }
        for profile in &self.known_profiles {
            i += 3;
            data.print(Pos::new(i, j), profile);
        }

        let name = self.editor.to_string();
        if let Some(err) = validate(&name) {
            data.console
                .print_color_centered(H / 2 + 5, light_red(), black(), err.str());
        } else {
            if data.pressed_key == Some(Key::Return) && !name.is_empty() {
                SFX::Confirm.play();
                edit_and_save(|c| {
                    c.default_profile = Some(name.clone());
                });
                return GameStateEvent::Switch(SaveLoaderState::try_load(name));
            }
        }
        GameStateEvent::None
    }

    fn on_event(&mut self, event: bl::BEvent, input: &bl::Input) {
        self.editor.on_event(&event, &input);
    }
}
