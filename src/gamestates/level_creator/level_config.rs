use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
/// UserLevelConfig is the format in which the JSON level config file must be specified.
/// See the field descriptions belows for more information.
pub struct UserLevelConfig {
    /// Name of the level shown in the UI. If not present, defaults to the same name as
    /// in the Workshop.
    name:            Option<String>,
    /// Description shown in the game UI. If not present, defaults to the same description
    /// as in the Workshop level.
    description:     Option<String>,
    /// If present, is displayed as extra info in the UI, next to the description.
    extra_info:      Option<String>,
    /// If present, is displayed as a hint in the UI, next to the description. A button must
    /// be clicked in order for the hint to be shown. Can't be specified if `extra_info` is
    /// specified.
    hint:            Option<String>,
    /// Each element of the vector is a test case. Each test case is represented as two strings.
    /// - The first must be a function that takes one argument, the user's solution to the problem.
    /// - The second is the reduction when the first function is applied to the user's solution.
    ///
    /// Example for the level "boolean or":
    /// `[["f: f TRUE FALSE", "TRUE"], ["f: FALSE FALSE", "FALSE"]]`
    ///
    /// You must specify at least one test case.
    test_cases:      Vec1<(String, String)>,
    /// Each of the strings must be a correct solution for the problem, and must pass all test
    /// cases. You must specify at least one valid solution.
    ///
    /// Example for the level "boolean or":
    /// `["a:b: x:y: a x (b x y)", "a:b: NOT (AND (NOT a) (NOT b))"]`
    solutions:       Vec1<String>,
    /// You may optionally specify wrong solutions, to make sure they do not pass all tests.
    wrong_solutions: Option<Vec<String>>,
}
