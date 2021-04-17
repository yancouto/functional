use super::{base::*, save_loader::SaveLoaderState};
use crate::{
    drawables::{BasicTextEditor, TextEditor, *}, prelude::*, save_system::{load_common, write_common, CommonConfig}
};

pub struct ProfileSelectionState {
    editor: BasicTextEditor,
}

pub fn try_load_default_profile() -> Box<dyn GameState> {
    let common = load_common();
    match common.default_profile {
        Some(user) => SaveLoaderState::try_load(user),
        // Maybe also go to ProfileSelection if the save was deleted
        None => box ProfileSelectionState {
            editor: BasicTextEditor::new("Enter profile name:".to_string(), Rect::centered(20, 1)),
        },
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
            ValidationError::NonAlphaNumeric => "Characters must be alphanumberic or _",
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
        // TODO: Add current profiles list
        self.editor.draw(&mut data);
        data.instructions(&["Press ENTER to create or load profile"]);
        let name = self.editor.to_string();
        if let Some(err) = validate(&name) {
            data.console
                .print_color_centered(H / 2 + 5, light_red(), black(), err.str());
        } else {
            if data.pressed_key == Some(Key::Return) && !name.is_empty() {
                write_common(CommonConfig {
                    default_profile: Some(name.clone()),
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
