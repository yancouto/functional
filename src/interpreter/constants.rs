use std::{collections::HashMap, convert::TryInto};

use crate::interpreter::{parse, tokenize, Node};

struct ConstantNode {
    term:   Box<Node>,
    number: u8,
}

lazy_static! {
    static ref ALL_CONSTANTS: HashMap<String, ConstantNode> = {
        let constants = [("TRUE", "a: b: a"), ("FALSE", "a: b: b")];
        constants
            .iter()
            .enumerate()
            .map(|(i, (name, value))| {
                (
                    name.to_string(),
                    ConstantNode {
                        term:   parse(
                            tokenize(value.chars()).expect("Failed to tokenize constant"),
                        )
                        .expect("Failed to parse constant"),
                        number: i.try_into().expect("Too many constants"),
                    },
                )
            })
            .collect()
    };
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantProvider {
    known_cts: u8,
}

impl ConstantProvider {
    pub fn new(known_cts: u8) -> Self { Self { known_cts } }

    pub fn get(&self, name: &str) -> Option<Box<Node>> {
        ALL_CONSTANTS
            .get(name)
            .filter(|n| n.number < self.known_cts)
            .map(|n| n.term.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::interpreter::interpret;
    #[test]
    fn test_constants() {
        // Can we load the constants without crashing?
        assert!(ALL_CONSTANTS.len() > 0);
    }

    #[test]
    fn test_provider() {
        let p0 = ConstantProvider::new(0);
        assert!(p0.get("TRUE").is_none());
        let p1 = ConstantProvider::new(1);
        assert!(p1.get("TRUE").is_some());
        assert!(p1.get("FALSE").is_none());
        let p2 = ConstantProvider::new(2);
        assert!(p2.get("TRUE").is_some());
        assert!(p2.get("FALSE").is_some());
    }

    #[test]
    fn test_constants_are_resolved() {
        ALL_CONSTANTS.iter().for_each(|(_, node)| {
            assert_eq!(interpret(node.term.clone(), true), Ok(node.term.clone()))
        });
    }
}
