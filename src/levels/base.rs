use crate::{
    interpreter::{interpret, parse, tokenize, InterpretError, Node, ParseError, TokenizeError},
    save_system::LevelResult,
};
use thiserror::Error;

#[derive(Debug)]
pub struct TestCase {
    /// Must be a function that receives the code and returns the result.
    application: Box<Node>,
    /// Result of the application
    /// TODO: Maybe we need more complex checking?
    expected_result: Box<Node>,
}

#[derive(Debug)]
pub struct Level {
    pub name: String,
    pub description: String,
    pub extra_info: Option<String>,
    pub test_cases: Vec<TestCase>,
}

fn parse_or_fail(str: &str) -> Box<Node> {
    parse(tokenize(str.chars()).expect("Failed to tokenize")).expect("Failed to parse")
}

#[derive(Debug)]
pub struct TestCaseRun {
    pub result: Result<Box<Node>, InterpretError>,
    pub expected_result: Box<Node>,
}

impl TestCaseRun {
    fn is_correct(&self) -> bool {
        self.result
            .as_ref()
            .map_or(false, |r| *r == self.expected_result)
    }
}

impl TestCase {
    pub fn from(application: &str, result: &str) -> Self {
        Self {
            application: parse_or_fail(application),
            expected_result: parse_or_fail(result),
        }
    }

    fn test(&self, expression: Box<Node>) -> TestCaseRun {
        let result = interpret(
            Box::new(Node::Apply {
                left: self.application.clone(),
                right: expression,
            }),
            true,
        );
        TestCaseRun {
            result,
            expected_result: self.expected_result.clone(),
        }
    }
}

#[derive(Error, Debug)]
pub enum LevelTestError {
    #[error("While tokenizing input")]
    TokenizeError(#[from] TokenizeError),
    #[error("While parsing tokens")]
    ParseError(#[from] ParseError),
}

pub type TestRunResults = Result<Vec<TestCaseRun>, LevelTestError>;

pub fn get_result(results: &TestRunResults) -> LevelResult {
    let r = match &results {
        Err(_) => false,
        Ok(runs) => runs.iter().all(|run| run.is_correct()),
    };
    if r {
        LevelResult::Success
    } else {
        LevelResult::Failure
    }
}

impl Level {
    pub fn test<S: IntoIterator<Item = char>>(&self, code: S) -> TestRunResults {
        let node = parse(tokenize(code)?)?;
        Ok(self
            .test_cases
            .iter()
            .map(|t| t.test(node.clone()))
            .collect())
    }
}
