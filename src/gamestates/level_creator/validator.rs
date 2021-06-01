use std::path::PathBuf;

use jsonnet::JsonnetVm;

use super::{super::base::*, UserLevelConfig, WorkshopConfig};
use crate::{
    interpreter::{
        parse, tokenize, ConstantProvider, InterpretError, Node, ParseError, TokenizeError
    }, levels::TestCase, prelude::*
};

#[derive(thiserror::Error, Debug)]
pub enum ValidationError {
    #[error("Level must have a non-empty title")]
    EmptyTitle,
    #[error("Level must have a non-empty description")]
    EmptyDescription,
    #[error("Error reading config: {0}")]
    JsonnetError(String),
    #[error("Error parsing JSON config: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Can't specify both extra_info and hint")]
    HasExtraInfoAndHint,
    #[error("Parsing error on {1}: {0}")]
    ParseError(ParseError, String),
    #[error("Tokenize error on {1}: {0}")]
    TokenizeError(TokenizeError, String),
    #[error("Error when interpreting {1}: {0}")]
    InterpretError(InterpretError, String),
    #[error("Test case #{test_idx} is not solved by solution #{sol_idx}.\nExpected: {expected}\nGot: {got}")]
    WrongSolution {
        sol_idx:  usize,
        test_idx: usize,
        expected: Box<Node>,
        got:      Box<Node>,
    },
    #[error("Wrong solution #{0} passes all tests.")]
    WrongSolutionIsCorrect(usize),
}

impl UserLevelConfig {
    fn parse<F: FnMut() -> String>(
        mut source: F,
        term: &str,
    ) -> Result<Box<Node>, ValidationError> {
        tokenize(term.chars())
            .map_err(|err| ValidationError::TokenizeError(err, source()))
            .and_then(|tokens| {
                parse(tokens).map_err(|err| ValidationError::ParseError(err, source()))
            })
    }

    fn validate(&self) -> Result<(), ValidationError> {
        if self.extra_info.is_some() && self.hint.is_some() {
            Err(ValidationError::HasExtraInfoAndHint)?
        }
        let mut idx = 0;
        let test_cases = self.test_cases.try_mapped_ref(|(application, result)| {
            idx += 1;
            Result::<_, ValidationError>::Ok(TestCase::from(
                Self::parse(|| format!("test case #{}'s application", idx), application)?,
                Self::parse(|| format!("test case #{}'s result", idx), result)?,
            ))
        })?;
        idx = 0;
        let solutions = self.solutions.try_mapped_ref(|sol| {
            idx += 1;
            Self::parse(|| format!("solution #{}", idx), sol)
        })?;
        let wrong_solutions = self
            .wrong_solutions
            .iter()
            .enumerate()
            .map(|(idx, sol)| Self::parse(|| format!("wrong solution #{}", idx + 1), sol))
            .collect::<Result<Vec<_>, ValidationError>>()?;

        // TODO: use extra_constants here
        let provider = ConstantProvider::all();

        solutions
            .into_par_iter()
            .enumerate()
            .try_for_each(|(si, s)| {
                test_cases.par_iter().enumerate().try_for_each(|(ti, t)| {
                    let run = t.test(s.clone(), provider.clone());
                    let expected = run.expected_result;
                    run.result
                        .map_err(|err| {
                            ValidationError::InterpretError(
                                err,
                                format!("solution #{} on test case #{}", si + 1, ti + 1),
                            )
                        })
                        .and_then(|int| {
                            if int.term == expected {
                                Ok(())
                            } else {
                                Err(ValidationError::WrongSolution {
                                    sol_idx: si + 1,
                                    test_idx: ti + 1,
                                    expected,
                                    got: int.term,
                                })
                            }
                        })
                })
            })?;

        wrong_solutions
            .into_par_iter()
            .enumerate()
            .try_for_each(|(wsi, ws)| {
                if test_cases
                    .par_iter()
                    .map(|t| t.test(ws.clone(), provider.clone()).is_correct())
                    .all(|b| b)
                {
                    Err(ValidationError::WrongSolutionIsCorrect(wsi))
                } else {
                    Ok(())
                }
            })?;

        Ok(())
    }
}

pub fn validate(workshop: WorkshopConfig, config: PathBuf) -> Result<(), ValidationError> {
    if workshop.title.is_empty() {
        Err(ValidationError::EmptyTitle)?;
    } else if workshop.description.is_empty() {
        Err(ValidationError::EmptyDescription)?;
    }
    let mut vm = JsonnetVm::new();
    let str = match vm.evaluate_file(config) {
        Ok(str) => str.to_string(),
        Err(err) => Err(ValidationError::JsonnetError(err.to_string()))?,
    };
    let config: UserLevelConfig = serde_json::from_str(&str)?;
    config.validate()
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
        assert_matches!(
            validate_with_json(r#"{"test_cases": [["f:A", "A"]]}"#),
            Err(ValidationError::SerdeError(..))
        );
        assert_matches!(
            validate_with_json(r#"{"test_cases": [["f:A", "A"]], "solutions": ["(a"]}"#),
            Err(ValidationError::ParseError(..))
        );
        assert_matches!(
            validate_with_json(r#"{"test_cases": [["f:A", "B"]], "solutions": ["x:x"]}"#),
            Err(ValidationError::WrongSolution {
                sol_idx:  1,
                test_idx: 1,
                got:      _,
                expected: _,
            })
        );
        assert_matches!(
            validate_with_json(
                r#"{"test_cases": [["f:A", "A"]], solutions: ["x:A"], wrong_solutions: self.solutions}"#
            ),
            Err(ValidationError::WrongSolutionIsCorrect(..))
        );
    }

    #[test]
    fn validation_ok() {
        assert_matches!(
            validate_with_json(r#"{"test_cases": [["f:A", "A"]], "solutions": ["x:x"]}"#),
            Ok(())
        );
        assert_matches!(
            validate_with_json(
                r#"{
                test_cases: [["f: f TRUE FALSE A B", "A"],["f: f FALSE FALSE A B", "B"]],
                solutions: ["a:b: x:y: a x (b x y)"],
                wrong_solutions: ["x:x"]}"#
            ),
            Ok(()),
        );
    }
}
