use std::{
    collections::HashMap, convert::{TryFrom, TryInto}, path::PathBuf
};

use crossbeam::channel::{Receiver, Sender};
use jsonnet::JsonnetVm;
use serde::{Deserialize, Serialize};

use super::{super::base::*, UserLevelConfig, WorkshopConfig};
use crate::{
    drawables::{black, XiEditor}, gamestates::{base::GameStateEvent, editor::EditorState, level_creator::UploadingLevelState}, interpreter::{
        parse, tokenize, ConstantProvider, InterpretError, Node, ParseError, TokenizeError
    }, levels::{BaseLevel, Level, TestCase, UserCreatedLevel}, prelude::*, save_system::SaveProfile
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
    #[error("Constant name must only have upper case A-Z characters or underscores, instead given '{0}'")]
    InvalidConstantName(String),
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

    fn check_string_uppercase(name: &str) -> Result<(), ValidationError> {
        if name.chars().any(|c| c != '_' && !c.is_ascii_uppercase()) {
            Err(ValidationError::InvalidConstantName(name.to_string()))
        } else {
            Ok(())
        }
    }

    fn validate(self, workshop: WorkshopConfig) -> Result<ParsedUserLevelConfig, ValidationError> {
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
        let extra_constants = self
            .extra_constants
            .iter()
            .map(|(name, term)| {
                Self::check_string_uppercase(name)?;
                Ok((
                    name.clone(),
                    Self::parse(|| format!("constant '{}'", name), term)?,
                ))
            })
            .collect::<Result<HashMap<String, Box<Node>>, ValidationError>>()?;

        let parsed = Arc::new(UserCreatedLevel {
            base: BaseLevel {
                name:               String::new(),
                description:        String::new(),
                extra_info_is_hint: false,
                extra_info:         None,
                test_cases:         test_cases.clone(),
            },
            extra_constants,
            id: None,
        });
        let provider = ConstantProvider::new(Level::UserCreatedLevel(parsed.clone()), None);
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

        Ok(ParsedUserLevelConfig {
            name:               self.name.unwrap_or(workshop.title),
            description:        self.description.unwrap_or(workshop.description),
            extra_info_is_hint: self.hint.is_some(),
            extra_info:         self.extra_info.or(self.hint),
            test_cases:         self.test_cases,
            extra_constants:    self.extra_constants,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ParsedUserLevelConfig {
    name:               String,
    description:        String,
    extra_info:         Option<String>,
    extra_info_is_hint: bool,
    test_cases:         Vec1<(String, String)>,
    #[serde(default)]
    extra_constants:    Vec<(String, String)>,
}

impl TryFrom<ParsedUserLevelConfig> for UserCreatedLevel {
    type Error = ValidationError;

    fn try_from(config: ParsedUserLevelConfig) -> Result<Self, ValidationError> {
        Ok(Self {
            base:            BaseLevel {
                name:               config.name,
                description:        config.description,
                extra_info:         config.extra_info,
                extra_info_is_hint: config.extra_info_is_hint,
                test_cases:         {
                    let mut idx = 0;
                    config.test_cases.try_mapped(|(term, result)| {
                        idx += 1;
                        Result::<_, ValidationError>::Ok(TestCase::from(
                            UserLevelConfig::parse(
                                || format!("test case {}'s application", idx),
                                &term,
                            )?,
                            UserLevelConfig::parse(
                                || format!("test case {}'s result", idx),
                                &result,
                            )?,
                        ))
                    })?
                },
            },
            extra_constants: config
                .extra_constants
                .into_iter()
                .map(|(name, term)| {
                    UserLevelConfig::check_string_uppercase(&name)?;
                    Ok((
                        name.clone(),
                        UserLevelConfig::parse(|| format!("constant '{}'", name), &term)?,
                    ))
                })
                .collect::<Result<HashMap<String, Box<Node>>, ValidationError>>()?,
            id:              None,
        })
    }
}

impl TryFrom<ParsedUserLevelConfig> for Level {
    type Error = ValidationError;

    fn try_from(value: ParsedUserLevelConfig) -> Result<Self, Self::Error> {
        value
            .try_into()
            .map(|ucl| Level::UserCreatedLevel(Arc::new(ucl)))
    }
}

pub fn validate(
    workshop: WorkshopConfig,
    config: PathBuf,
) -> Result<ParsedUserLevelConfig, ValidationError> {
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
    config.validate(workshop)
}

#[derive(Debug)]
pub struct ValidationState {
    level:        Result<ParsedUserLevelConfig, ValidationError>,
    save_profile: Arc<SaveProfile>,
    config:       WorkshopConfig,
    id_send:      Sender<u64>,
    id_recv:      Option<Receiver<u64>>,
}

impl ValidationState {
    pub fn new(
        level: Result<ParsedUserLevelConfig, ValidationError>,
        save_profile: Arc<SaveProfile>,
        config: WorkshopConfig,
    ) -> (Self, Receiver<u64>) {
        let (send, recv) = crossbeam::channel::unbounded();
        (
            Self {
                level,
                save_profile,
                config,
                id_send: send,
                id_recv: None,
            },
            recv,
        )
    }
}

impl GameState for ValidationState {
    fn name(&self) -> &'static str { "Validation" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        let rect = Rect::centered(70, 60);
        data.text_box(
            "Level validation",
            &match &self.level {
                Ok(_) => "No validation errors, level looks good.".to_string(),
                Err(err) => format!("ERROR: {}", err),
            },
            rect,
            true,
        );
        let mut instructions = Vec::with_capacity(3);
        const PLAY: &str = "Play";
        const UPLOAD: &str = "Upload";
        if let Ok(uc) = &self.level {
            if data.button(PLAY, Pos::new(rect.bottom() - 3, rect.pos.j + 1), black())
                || (data.ctrl && data.pressed_key == Some(Key::Return))
            {
                if let Ok(level) = uc.clone().try_into() {
                    return GameStateEvent::Push(box EditorState::<XiEditor>::new(
                        level,
                        self.save_profile.clone(),
                    ));
                } else {
                    debug_unreachable!("Should not be error");
                }
            }
            if let Some(client) = data.steam_client.clone() {
                instructions.push("Press CTRL+U or UPLOAD to upload level to steam");
                if data.button(
                    UPLOAD,
                    Pos::new(rect.bottom() - 3, rect.pos.j + 3 + PLAY.len() as i32),
                    black(),
                ) || (data.ctrl && data.pressed_key == Some(Key::U))
                {
                    let (state, recv) =
                        UploadingLevelState::new(uc.clone(), client, self.config.clone());
                    self.id_recv = Some(recv);
                    return GameStateEvent::Push(box state);
                }
            }
            instructions.push("Press CTRL+ENTER or PLAY to test play level");
        }
        instructions.push("Press ESC to go back");
        data.instructions(&instructions);

        // Get id from uploader and return it to level creator editor
        if let Some(id) = self.id_recv.take().and_then(|r| r.try_recv().ok()) {
            self.id_send.send(id).debug_unwrap();
        }

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
        validate(workshop(), file.path().to_owned())?;
        Ok(())
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
        assert_matches!(
            validate_with_json(
                r#"{
                test_cases: [["f: f A", "A A"]],
                solutions: ["UNKNOWN"]}"#
            ),
            Err(ValidationError::WrongSolution { .. }),
        );
        assert_matches!(
            validate_with_json(
                r#"{
                test_cases: [["f: f A", "A"]],
                solutions: ["x:x"],
                extra_constants: [["A2", "x: x x"]]}"#
            ),
            Err(ValidationError::InvalidConstantName(..)),
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
        assert_matches!(
            validate_with_json(
                r#"{
                test_cases: [["f: f A", "A A"]],
                solutions: ["DOUBLE"],
                extra_constants: [["DOUBLE", "x: x x"]]}"#
            ),
            Ok(()),
        );
    }
}
