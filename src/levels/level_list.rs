use super::Level;

lazy_static! {
    pub static ref LEVELS: Vec<Level> = vec![Level {
        name: "identity".to_string(),
        description: "Some description".to_string(),
        test_cases: vec![]
    }];
}
