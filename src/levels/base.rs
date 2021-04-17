use thiserror::Error;

use super::SectionName;
use crate::{
    interpreter::{
        interpret, parse, tokenize, ConstantProvider, InterpretError, Node, ParseError, TokenizeError
    }, save_system::LevelResult
};

#[derive(Debug)]
pub struct TestCase {
    /// Must be a function that receives the code and returns the result.
    application:     Box<Node>,
    /// Result of the application
    /// TODO: Maybe we need more complex checking?
    expected_result: Box<Node>,
}

#[derive(Debug)]
pub struct Level {
    pub name:        String,
    pub description: String,
    pub section:     SectionName,
    pub extra_info:  Option<String>,
    pub test_cases:  Vec<TestCase>,
    pub solutions:   Vec<String>,
}

pub fn parse_or_fail(str: &str) -> Box<Node> {
    parse(tokenize(str.chars()).expect("Failed to tokenize")).expect("Failed to parse")
}

#[derive(Debug, Clone)]
pub struct TestCaseRun {
    pub test_expression: Box<Node>,
    pub result:          Result<Box<Node>, InterpretError>,
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
            application:     parse_or_fail(application),
            expected_result: interpret(parse_or_fail(result), true, ConstantProvider::all())
                .expect("Failed to interpret result"),
        }
    }

    fn test_expression(&self, expression: Box<Node>) -> Box<Node> {
        box Node::Apply {
            left:  self.application.clone(),
            right: expression,
        }
    }

    fn test(&self, expression: Box<Node>, provider: ConstantProvider) -> TestCaseRun {
        let test_expression = self.test_expression(expression);
        let result = interpret(test_expression.clone(), true, provider);
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
}

#[derive(Debug)]
pub struct TestCaseRuns {
    pub runs: Vec<TestCaseRun>,
    pub code: Box<Node>,
}

pub type TestRunResults = Result<TestCaseRuns, LevelTestError>;

pub fn get_result(results: &TestRunResults) -> LevelResult {
    let r = match &results {
        Err(_) => false,
        Ok(runs) => runs.runs.iter().all(|run| run.is_correct()),
    };
    if r {
        LevelResult::Success
    } else {
        LevelResult::Failure
    }
}

impl Level {
    pub fn test<S: IntoIterator<Item = char>>(
        &self,
        code: S,
        provider: ConstantProvider,
    ) -> TestRunResults {
        let node = parse(tokenize(code)?)?;
        Ok(TestCaseRuns {
            runs: self
                .test_cases
                .iter()
                .map(|t| t.test(node.clone(), provider))
                .collect(),
            code: node,
        })
    }
}
