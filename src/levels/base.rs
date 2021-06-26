use std::{collections::HashMap, time::Instant};

use thiserror::Error;

use super::SectionName;
use crate::{
    interpreter::{
        accumulate_stats, count_functions, interpret, parse, tokenize, traversers::all_constants, ConstantProvider, InterpretError, Interpreted, Node, ParseError, TokenizeError
    }, prelude::*, save_system::LevelResult
};

#[derive(Debug, Clone)]
pub struct TestCase {
    /// Must be a function that receives the code and returns the result.
    pub application: Box<Node>,
    /// Result of the application
    /// TODO: Maybe we need more complex checking?
    expected_result: Box<Node>,
}

// Every level has these fields, game levels and user created ones
#[derive(Debug)]
pub struct BaseLevel {
    pub name:               String,
    pub description:        String,
    pub extra_info:         Option<String>,
    pub extra_info_is_hint: bool,
    pub test_cases:         Vec1<TestCase>,
}

// One the game's core levels
#[derive(Debug)]
pub struct GameLevel {
    pub base:            BaseLevel,
    pub section:         SectionName,
    /// index of the level in the section
    pub idx:             usize,
    pub solutions:       Vec1<String>,
    pub wrong_solutions: Vec<String>,
    pub show_constants:  bool,
}

#[derive(Debug)]
pub struct UserCreatedLevel {
    pub base:            BaseLevel,
    pub extra_constants: HashMap<String, Box<Node>>,
    /// If this is a level the user subscribed through steam, the published file id
    /// otherwise None
    pub id:              Option<u64>,
}

// This should be lightweight and easy to clone
#[derive(Debug, Clone)]
pub enum Level {
    GameLevel(&'static GameLevel),
    UserCreatedLevel(Arc<UserCreatedLevel>),
}

impl From<&'static GameLevel> for Level {
    fn from(level: &'static GameLevel) -> Self { Self::GameLevel(level) }
}

impl Level {
    pub fn base(&self) -> &BaseLevel {
        match self {
            Level::GameLevel(gl) => &gl.base,
            Level::UserCreatedLevel(uc) => &uc.base,
        }
    }

    /// A unique identifier that may be used for this level for saves/leaderboards
    /// Might not exist if the level is user created and not uploaded to Steam
    pub fn uuid(&self) -> Option<String> {
        match &self {
            Level::GameLevel(gl) => Some(gl.base.name.clone()),
            Level::UserCreatedLevel(uc) => uc.id.map(|id| format!("{}", id)),
        }
    }
}

pub fn parse_or_fail(str: &str) -> Box<Node> {
    parse(tokenize(str.chars()).expect("Failed to tokenize")).expect("Failed to parse")
}

#[derive(Debug, Clone)]
pub struct TestCaseRun {
    pub test_expression: Box<Node>,
    pub result:          Result<Interpreted, InterpretError>,
    pub expected_result: Box<Node>,
}

impl TestCaseRun {
    pub fn is_correct(&self) -> bool {
        self.result
            .as_ref()
            .map_or(false, |r| r.term == self.expected_result)
    }
}

impl TestCase {
    pub fn from(application: Box<Node>, expected_result: Box<Node>) -> Self {
        Self {
            application,
            expected_result,
        }
    }

    pub fn from_or_fail(application: &str, result: &str) -> Self {
        Self::from(
            parse_or_fail(application),
            // fine to use all here since this is not user supplied
            interpret(parse_or_fail(result), false, ConstantProvider::all())
                .expect("Failed to interpret result")
                .term,
        )
    }

    fn test_expression(&self, expression: Box<Node>) -> Box<Node> {
        box Node::Apply {
            left:  self.application.clone(),
            right: expression,
        }
    }

    pub fn test(&self, expression: Box<Node>, provider: ConstantProvider) -> TestCaseRun {
        let test_expression = self.test_expression(expression);
        let result = interpret(test_expression.clone(), false, provider);
        TestCaseRun {
            test_expression,
            result,
            expected_result: self.expected_result.clone(),
        }
    }
}

#[derive(Error, Debug)]
pub enum LevelTestError {
    #[error("While tokenizing input: {0}")]
    TokenizeError(#[from] TokenizeError),
    #[error("While parsing tokens: {0}")]
    ParseError(#[from] ParseError),
    #[error("Constant {0} is not known")]
    UnknownConstant(String),
}

#[derive(Debug)]
pub struct TestCaseRuns {
    pub runs: Vec<TestCaseRun>,
    pub code: Box<Node>,
}

pub type TestRunResults = Result<TestCaseRuns, LevelTestError>;

pub fn get_result(results: &TestRunResults) -> LevelResult {
    let r = match &results {
        Err(_) => None,
        Ok(runs) => Some(runs.runs.iter().map(|run| {
            if run.is_correct() {
                run.result.as_ref().ok().map(|r| r.stats)
            } else {
                None
            }
        })),
    };

    r.zip(results.as_ref().ok())
        .and_then(|(maybe_stats, runs)| {
            let full_len = maybe_stats.len();
            let stats: Vec<_> = maybe_stats.filter_map(|x| x).collect();
            if full_len != stats.len() {
                None
            } else {
                Some(accumulate_stats(stats, count_functions(&runs.code)))
            }
        })
        .map(|stats| LevelResult::Success { stats })
        .unwrap_or(LevelResult::Failure)
}

fn check_constants(node: &Node, provider: ConstantProvider) -> Result<(), LevelTestError> {
    for constant in all_constants(&node) {
        if provider.get(&constant).is_none() {
            return Err(LevelTestError::UnknownConstant(constant.to_string()));
        }
    }
    Ok(())
}

impl Level {
    pub fn test<S: IntoIterator<Item = char>>(
        &self,
        code: S,
        provider: ConstantProvider,
    ) -> TestRunResults {
        let ts = Instant::now();
        let node = parse(tokenize(code)?)?;
        check_constants(&node, provider)?;
        // From here, we use all constants as the test cases may have unknown constants and that's fine
        let provider = ConstantProvider::all();
        let ans = Ok(TestCaseRuns {
            runs: self
                .base()
                .test_cases
                .par_iter()
                .map(|t| t.test(node.clone(), provider.clone()))
                .collect(),
            code: node,
        });
        log::info!(
            "Ran solution for level '{}' in {:?}",
            self.base().name,
            Instant::now() - ts
        );
        ans
    }
}
