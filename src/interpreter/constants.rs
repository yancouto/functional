use std::{collections::HashMap, convert::TryInto};

use crate::{
    interpreter::{parse, tokenize, Node}, levels::SectionName
};

struct ConstantNode {
    term:    Box<Node>,
    section: SectionName,
    number:  u8,
}

impl ConstantNode {
    fn accepts(&self, (s, n): (SectionName, u8)) -> bool { self.section == s && self.number < n }
}

lazy_static! {
    static ref ALL_CONSTANTS: HashMap<String, ConstantNode> = {
        let constants = hashmap! {
            SectionName::Boolean => vec![
                ("TRUE", "a: b: a"),
                ("FALSE", "a: b: b"),
                ("NOT", "b:x:y: b y x"),
                ("AND", "a:b: x:y: a (b x y) y")
            ]
        };
        constants
            .into_iter()
            .flat_map(|(section, consts)| {
                consts
                    .into_iter()
                    .enumerate()
                    .map(move |(i, (name, value))| {
                        (
                            name.to_string(),
                            ConstantNode {
                                term: parse(
                                    tokenize(value.chars()).expect("Failed to tokenize constant"),
                                )
                                .expect("Failed to parse constant"),
                                section,
                                number: i.try_into().expect("Too many constants"),
                            },
                        )
                    })
            })
            .collect()
    };
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantProvider {
    // TODO: probably need multiple sections at some point
    known_cts: (SectionName, u8),
}

impl ConstantProvider {
    pub fn new(known_cts: (SectionName, u8)) -> Self { Self { known_cts } }

    #[allow(dead_code)]
    pub fn none() -> Self {
        Self {
            known_cts: (SectionName::Basic, 0),
        }
    }

    pub fn all() -> Self {
        // TODO: fix this when we add more sections
        Self::new((SectionName::Boolean, 100))
    }

    pub fn get(&self, name: &str) -> Option<Box<Node>> {
        ALL_CONSTANTS
            .get(name)
            .filter(|n| n.accepts(self.known_cts))
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
        // TODO: test that constants names are compatible with the levels
    }

    #[test]
    fn test_provider() {
        let p0 = ConstantProvider::new((SectionName::Boolean, 0));
        assert!(p0.get("TRUE").is_none());
        let p1 = ConstantProvider::new((SectionName::Boolean, 1));
        assert!(p1.get("TRUE").is_some());
        assert!(p1.get("FALSE").is_none());
        let p2 = ConstantProvider::new((SectionName::Boolean, 2));
        assert!(p2.get("TRUE").is_some());
        assert!(p2.get("FALSE").is_some());
    }

    #[test]
    fn test_constants_are_resolved() {
        ALL_CONSTANTS.iter().for_each(|(_, node)| {
            assert_eq!(
                interpret(node.term.clone(), true, ConstantProvider::none()),
                Ok(node.term.clone())
            )
        });
    }
}
