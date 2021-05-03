use std::collections::HashMap;

use crate::{
    interpreter::{parse, tokenize, Node}, levels::{raw_load_level_config, Level, SectionName, LEVELS}
};
struct ConstantNode {
    term:           Box<Node>,
    section:        SectionName,
    lvl_discovered: usize,
    before_level:   bool,
}

impl ConstantNode {
    fn can_be_used(&self, level: &'static Level) -> bool {
        (self.section, self.lvl_discovered, !self.before_level) <= (level.section, level.idx, false)
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
                section
                    .levels
                    .into_iter()
                    .enumerate()
                    .flat_map(move |(i, level)| {
                        let mut v: Vec<_> = level
                            .before_level_constants
                            .into_iter()
                            .map(|(a, b)| (a, b, true))
                            .collect();
                        if level.provides_constant {
                            v.push((
                                level.name.to_ascii_uppercase(),
                                level.solutions[0].clone(),
                                false,
                            ));
                        }
                        v.into_iter().map(move |(name, term, before_level)| {
                            (
                                name,
                                ConstantNode {
                                    term: parse_constant(&term),
                                    section: section_name,
                                    lvl_discovered: i,
                                    before_level,
                                },
                            )
                        })
                    })
            })
            .collect()
    };
}

#[derive(Debug, Clone, Copy)]
enum Numerals {
    None,
    Default,
}

impl Numerals {
    fn get_default_num(x: u16, str: &mut String) {
        if x == 0 {
            str.push_str("x: x");
        } else {
            str.push_str("p: p (a:b: b) (");
            Self::get_default_num(x - 1, str);
            str.push(')');
        }
    }

    fn get_num(self, x: u16) -> Option<Box<Node>> {
        match self {
            Numerals::None => None,
            Numerals::Default => {
                let mut term = String::with_capacity(x as usize * 16);
                Self::get_default_num(x, &mut term);
                Some(parse_constant(&term))
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantProvider {
    // TODO: probably need multiple sections at some point
    // None currently means use all constants
    current_level: Option<&'static Level>,
    numerals:      Numerals,
}

impl ConstantProvider {
    pub fn new(current_level: &'static Level) -> Self {
        Self {
            current_level: Some(current_level),
            numerals:      if current_level.section >= SectionName::Numerals {
                Numerals::Default
            } else {
                Numerals::None
            },
        }
    }

    #[allow(dead_code)]
    pub fn none() -> Self { Self::new(LEVELS.first().levels.first()) }

    pub fn all() -> Self {
        // WARNING: Can't use LEVELS here, as it depends on this function
        Self {
            current_level: None,
            numerals:      Numerals::Default,
        }
    }

    pub fn get(&self, name: &str) -> Option<Box<Node>> {
        if let Ok(x) = name.parse::<u16>() {
            self.numerals.get_num(x)
        } else {
            ALL_CONSTANTS
                .get(name)
                .filter(|n| self.current_level.map(|l| n.can_be_used(l)).unwrap_or(true))
                .map(|n| n.term.clone())
        }
    }

    pub fn all_known_constants<'a>(&'a self) -> Vec<&'static str> {
        let mut ans: Vec<_> = ALL_CONSTANTS
            .iter()
            .filter_map(|(k, v)| {
                if self.current_level.map(|l| v.can_be_used(l)).unwrap_or(true) {
                    Some(k.as_str())
                } else {
                    None
                }
            })
            .collect();
        match self.numerals {
            Numerals::None => {},
            Numerals::Default => ans.append(&mut vec!["0", "1", "..."]),
        }
        ans
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
    use crate::interpreter::{
        interpret, interpreter::test::{interpret_ok, interpret_ok_full}
    };
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
        let interpret_clean =
            |n: Box<Node>| interpret(n, false, ConstantProvider::none()).map(|i| i.term);
        ALL_CONSTANTS.iter().for_each(|(_, node)| {
            assert_eq!(interpret_clean(node.term.clone()), Ok(node.term.clone()))
        });
        vec![0u16, 2, 20].into_iter().for_each(|n| {
            let constant = Numerals::Default.get_num(n).unwrap();
            assert_eq!(interpret_clean(constant.clone()), Ok(constant));
        })
    }

    #[test]
    fn numerals() {
        assert!(Numerals::None.get_num(0).is_none());
        assert!(Numerals::None.get_num(2).is_none());
        assert_eq!(Numerals::Default.get_num(0), Some(interpret_ok("a: a")));
        assert_eq!(
            Numerals::Default.get_num(2),
            Some(interpret_ok_full("PAIR FALSE (PAIR FALSE (a:a))", true))
        );
    }
}
