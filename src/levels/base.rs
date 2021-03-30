use crate::interpreter::{
    interpret, parse, tokenize, InterpretError, Node, ParseError, TokenizeError,
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

#[derive(Error, Debug)]
pub enum TestCaseError {
    #[error("Failed to interpret test case")]
    InterpretError(#[from] InterpretError),
    #[error("Output of test is not expected")]
    WrongAnswer,
}

impl TestCase {
    pub fn from(application: &str, result: &str) -> Self {
        Self {
            application: parse_or_fail(application),
            expected_result: parse_or_fail(result),
        }
    }

    fn test(&self, expression: Box<Node>) -> Result<(), TestCaseError> {
        let result = interpret(
            Box::new(Node::Apply {
                left: self.application.clone(),
                right: expression,
            }),
            true,
        )?;
        if result == self.expected_result {
            Ok(())
        } else {
            Err(TestCaseError::WrongAnswer)
        }
    }
}

#[derive(Error, Debug)]
pub enum LevelTestError {
    #[error("While tokenizing input")]
    TokenizeError(#[from] TokenizeError),
    #[error("While parsing tokens")]
    ParseError(#[from] ParseError),
    #[error("Failed some test")]
    TestCaseError(#[from] TestCaseError),
}

impl Level {
    pub fn test<S: IntoIterator<Item = char>>(&self, code: S) -> Result<(), LevelTestError> {
        let node = parse(tokenize(code)?)?;
        for test_case in &self.test_cases {
            test_case.test(node.clone())?;
        }
        Ok(())
    }
}
