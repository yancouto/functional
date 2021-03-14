use crate::interpreter::Node;

pub struct TestCase {
    /// Must be a function that receives the code and returns the result.
    application: Box<Node>,
    /// Result of the application
    /// TODO: Maybe we need more complex checking?
    expected_result: Box<Node>,
}

pub struct Level {
    pub name: String,
    pub description: String,
    pub test_cases: Vec<TestCase>,
}
