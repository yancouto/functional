use super::{Level, TestCase};

lazy_static! {
    pub static ref LEVELS: Vec<Level> = vec![Level {
        name: "identity".to_string(),
        description: "Some description".to_string(),
        test_cases: vec![
            TestCase::from("f: f A", "A"),
            TestCase::from("f: f B", "B"),
            TestCase::from("f: f (x: x)", "x:x")
        ]
    }];
}
