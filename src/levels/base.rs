use std::time::Instant;

use thiserror::Error;

use super::SectionName;
use crate::{
    interpreter::{
        accumulate_stats, count_functions, interpret, parse, tokenize, ConstantProvider, InterpretError, Interpreted, Node, ParseError, TokenizeError
    }, prelude::*, save_system::LevelResult
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
    pub name:            String,
    pub description:     String,
    pub section:         SectionName,
    /// index of the level in the section
    pub idx:             usize,
    pub extra_info:      Option<String>,
    pub test_cases:      Vec<TestCase>,
    pub solutions:       Vec<String>,
    pub wrong_solutions: Vec<String>,
    pub show_constants:  bool,
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
    pub fn from(application: &str, result: &str) -> Self {
        Self {
            application:     parse_or_fail(application),
            // fine to use all here since this is not user supplied
            expected_result: interpret(parse_or_fail(result), false, ConstantProvider::all())
                .expect("Failed to interpret result")
                .term,
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

impl Level {
    pub fn test<S: IntoIterator<Item = char>>(
        &self,
        code: S,
        provider: ConstantProvider,
    ) -> TestRunResults {
        let ts = Instant::now();
        let node = parse(tokenize(code)?)?;
        let ans = Ok(TestCaseRuns {
            runs: self
                .test_cases
                .par_iter()
                .map(|t| t.test(node.clone(), provider))
                .collect(),
            code: node,
        });
        log::info!(
            "Ran solution for level '{}' in {:?}",
            self.name,
            Instant::now() - ts
        );
        ans
    }
}
