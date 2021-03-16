use crate::interpreter::{self, interpret, parse, tokenize, Node};

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
    pub test_cases: Vec<TestCase>,
}

fn parse_or_fail(str: &str) -> Box<Node> {
    parse(tokenize(str.chars()).expect("Failed to tokenize")).expect("Failed to parse")
}

impl TestCase {
    pub fn from(application: &str, result: &str) -> Self {
        Self {
            application: parse_or_fail(application),
            expected_result: parse_or_fail(result),
        }
    }

    fn test(&self, expression: Box<Node>) -> bool {
        interpret(Box::new(Node::Apply {
            left: self.application.clone(),
            right: expression,
        }))
        .map_or(false, |result| result == self.expected_result)
    }
}

impl Level {
    pub fn test<S: IntoIterator<Item = char>>(&self, code: S) -> bool {
        let node = tokenize(code).ok().and_then(|tokens| parse(tokens).ok());
        if let Some(node) = node {
            !self.test_cases.iter().any(|tc| !tc.test(node.clone()))
        } else {
            false
        }
    }
}
