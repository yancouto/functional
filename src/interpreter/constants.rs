use std::collections::HashMap;

use crate::{
    interpreter::{parse, tokenize, Node}, levels::{raw_load_level_config, Level, SectionName, LEVELS}
};
struct ConstantNode {
    term:           Box<Node>,
    section:        SectionName,
    lvl_discovered: Option<usize>,
}

impl ConstantNode {
    fn can_be_used(&self, level: &'static Level) -> bool {
        self.section < level.section
            || (self.section == level.section
                && self.lvl_discovered.map(|l| l < level.idx).unwrap_or(true))
    }
}

fn parse_constant(term: &str) -> Box<Node> {
    parse(tokenize(term.chars()).expect("Failed to tokenize constant"))
        .expect("Failed to parse constant")
}

lazy_static! {
    static ref ALL_CONSTANTS: HashMap<String, ConstantNode> = {
        raw_load_level_config()
            .sections
            .into_iter()
            .flat_map(|section| {
                let section_name = section.name;
                let section_constants = section.section_constants;
                section
                    .levels
                    .into_iter()
                    .filter(|l| l.provides_constant)
                    .enumerate()
                    .map(move |(i, level)| {
                        (
                            level.name.to_uppercase(),
                            ConstantNode {
                                term:           parse_constant(&level.solutions[0]),
                                section:        section_name,
                                lvl_discovered: Some(i),
                            },
                        )
                    })
                    .chain(section_constants.into_iter().map(move |(name, term)| {
                        (
                            name,
                            ConstantNode {
                                term:           parse_constant(&term),
                                section:        section_name,
                                lvl_discovered: None,
                            },
                        )
                    }))
            })
            .collect()
    };
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantProvider {
    // TODO: probably need multiple sections at some point
    // None currently means use all constants
    current_level: Option<&'static Level>,
}

impl ConstantProvider {
    pub fn new(current_level: &'static Level) -> Self {
        Self {
            current_level: Some(current_level),
        }
    }

    #[allow(dead_code)]
    pub fn none() -> Self { Self::new(LEVELS.first().levels.first()) }

    pub fn all() -> Self {
        // WARNING: Can't use LEVELS here, as it depends on this function
        Self {
            current_level: None,
        }
    }

    pub fn get(&self, name: &str) -> Option<Box<Node>> {
        ALL_CONSTANTS
            .get(name)
            .filter(|n| self.current_level.map(|l| n.can_be_used(l)).unwrap_or(true))
            .map(|n| n.term.clone())
    }

    pub fn all_known_constants<'a>(&'a self) -> Vec<&'static str> {
        ALL_CONSTANTS
            .iter()
            .filter_map(|(k, v)| {
                if self.current_level.map(|l| v.can_be_used(l)).unwrap_or(true) {
                    Some(k.as_str())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Level {
    pub fn all_known_constants(&'static self) -> Vec<&'static str> {
        if self.show_constants {
            ConstantProvider::new(&self).all_known_constants()
        } else {
            vec![]
        }
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
        let p0 = ConstantProvider::new(&LEVELS[1].levels[0]);
        assert!(p0.get("TRUE").is_some());
        assert!(p0.get("IF").is_none());
        let p1 = ConstantProvider::new(&LEVELS[1].levels[1]);
        assert!(p1.get("TRUE").is_some());
        assert!(p1.get("IF").is_some());
        assert!(p1.get("NOT").is_none());
        let p2 = ConstantProvider::new(&LEVELS[1].levels[2]);
        assert!(p2.get("NOT").is_some());
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
