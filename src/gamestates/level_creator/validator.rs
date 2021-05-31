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
        Err(ValidationError::EmptyTitle)?;
    } else if workshop.description.is_empty() {
        Err(ValidationError::EmptyDescription)?;
    }
    let mut vm = JsonnetVm::new();
    let _str = match vm.evaluate_file(config) {
        Ok(str) => str.to_string(),
        Err(err) => Err(ValidationError::JsonnetError(err.to_string()))?,
    };
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

#[cfg(test)]
mod test {
    use std::io::Write;

    use super::*;

    fn workshop() -> WorkshopConfig {
        WorkshopConfig {
            title: "a".to_string(),
            description: "b".to_string(),
            ..Default::default()
        }
    }

    fn validate_with_json(json: &str) -> Result<(), ValidationError> {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        write!(file.as_file_mut(), "{}", json).unwrap();
        validate(workshop(), file.path().to_owned())
    }

    #[test]
    fn validation_errors() {
        assert_matches!(
            validate(WorkshopConfig::default(), PathBuf::new()),
            Err(ValidationError::EmptyTitle)
        );
        assert_matches!(
            validate(
                WorkshopConfig {
                    title: "a".to_string(),
                    ..Default::default()
                },
                PathBuf::new()
            ),
            Err(ValidationError::EmptyDescription)
        );
        assert_matches!(
            validate_with_json("not a json"),
            Err(ValidationError::JsonnetError(..))
        );
    }

    #[test]
    fn validation_ok() {
        assert_matches!(validate_with_json("{}"), Ok(()));
        assert_matches!(validate_with_json(r#"{"a": "b"}"#), Ok(()));
        assert_matches!(validate_with_json(r#"{a: "b"}"#), Ok(()));
    }
}
