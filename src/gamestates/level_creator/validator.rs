use std::path::PathBuf;

use jsonnet::JsonnetVm;

use super::{super::base::*, WorkshopConfig};
use crate::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum ValidationError {
    #[error("Level must have a non-empty title")]
    EmptyTitle,
    #[error("Level must have a non-empty description")]
    EmptyDescription,
    #[error("Error reading config: {0}")]
    JsonnetError(String),
}

pub fn validate(workshop: WorkshopConfig, config: PathBuf) -> Result<(), ValidationError> {
    if workshop.title.is_empty() {
        return Err(ValidationError::EmptyTitle);
    } else if workshop.description.is_empty() {
        return Err(ValidationError::EmptyDescription);
    }
    let mut vm = JsonnetVm::new();
    vm.evaluate_file(config)
        .map_err(|err| ValidationError::JsonnetError(err.to_string()))?;
    Ok(())
}

#[derive(Debug)]
pub struct ValidationState {
    err: Option<ValidationError>,
}

impl ValidationState {
    pub fn new(err: Option<ValidationError>) -> Self { Self { err } }
}

impl GameState for ValidationState {
    fn name(&self) -> &'static str { "Validation" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        data.text_box(
            "Level validation",
            &match &self.err {
                Some(err) => format!("ERROR: {}", err),
                None => "No validation errors, level looks good.".to_string(),
            },
            Rect::centered(70, 60),
            true,
        );
        data.instructions(&["Press ESC to go back"]);
        if data.pressed_key == Some(Key::Escape) {
            GameStateEvent::Pop(1)
        } else {
            GameStateEvent::None
        }
    }

    fn clear_terminal(&self) -> bool { false }
}
