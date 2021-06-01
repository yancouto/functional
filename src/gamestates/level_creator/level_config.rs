use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
/// UserLevelConfig is the format in which the JSON level config file must be specified.
/// See the field descriptions belows for more information.
pub struct UserLevelConfig {
    /// Name of the level shown in the UI. If not present, defaults to the same name as
    /// in the Workshop.
    pub name:            Option<String>,
    /// Description shown in the game UI. If not present, defaults to the same description
    /// as in the Workshop level.
    pub description:     Option<String>,
    /// If present, is displayed as extra info in the UI, next to the description.
    pub extra_info:      Option<String>,
    /// If present, is displayed as a hint in the UI, next to the description. A button must
    /// be clicked in order for the hint to be shown. Can't be specified if `extra_info` is
    /// specified.
    pub hint:            Option<String>,
    /// Each element of the vector is a test case. Each test case is represented as two strings.
    /// - The first must be a function that takes one argument, the user's solution to the problem.
    /// - The second is the reduction when the first function is applied to the user's solution.
    ///
    /// Example for the level "boolean or":
    /// `[["f: f TRUE FALSE A B", "A"], ["f: FALSE FALSE A B", "B"]]`
    ///
    /// You must specify at least one test case.
    pub test_cases:      Vec1<(String, String)>,
    /// Each of the strings must be a correct solution for the problem, and must pass all test
    /// cases. You must specify at least one valid solution.
    ///
    /// Example for the level "boolean or":
    /// `["a:b: x:y: a x (b x y)", "a:b: NOT (AND (NOT a) (NOT b))"]`
    pub solutions:       Vec1<String>,
    #[serde(default)]
    /// You may optionally specify wrong solutions, to make sure they do not pass all tests.
    pub wrong_solutions: Vec<String>,
    #[serde(default)]
    /// This is an optional list of additional constants the player may use. By default the
    /// user knows all constants from the game.
    pub extra_constants: Vec<(String, String)>,
}
