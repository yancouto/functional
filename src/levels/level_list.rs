use serde::Deserialize;

use super::{Level, TestCase};
use crate::prelude::*;
fn get_true() -> bool { true }
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JLevel {
    pub name:                   String,
    pub description:            String,
    pub extra_info:             Option<String>,
    pub test_cases:             Vec<(String, String)>,
    pub solutions:              Vec<String>,
    #[serde(default)]
    pub wrong_solutions:        Vec<String>,
    #[serde(default)]
    pub provides_constant:      bool,
    #[serde(default = "get_true")]
    pub show_constants:         bool,
    #[serde(default)]
    pub before_level_constants: Vec<(String, String)>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JSection {
    pub name:   SectionName,
    pub levels: Vec<JLevel>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JLevelConfig {
    pub sections: Vec<JSection>,
}

const RAW_LEVEL_CONFIG: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/level_config.json"));

pub fn raw_load_level_config() -> JLevelConfig {
    serde_json::from_slice(RAW_LEVEL_CONFIG).expect("Invalid json")
}

fn load_all() -> Vec1<Section> {
    let config = raw_load_level_config();
    Vec1::try_from_vec(
        config
            .sections
            .into_iter()
            .map(|s| {
                let section_name = s.name;
                Section {
                    name:   s.name,
                    levels: Vec1::try_from_vec(
                        s.levels
                            .into_iter()
                            .enumerate()
                            .map(|(idx, l)| Level {
                                idx,
                                name: l.name,
                                description: l.description,
                                extra_info: l.extra_info,
                                section: section_name,
                                test_cases: l
                                    .test_cases
                                    .into_iter()
                                    .map(|t| TestCase::from(&t.0, &t.1))
                                    .collect(),
                                solutions: l.solutions,
                                wrong_solutions: l.wrong_solutions,
                                show_constants: l.show_constants,
                            })
                            .collect(),
                    )
                    .unwrap(),
                }
            })
            .collect(),
    )
    .unwrap()
}

#[derive(Debug, strum::Display, PartialEq, Eq, Hash, Clone, Copy, Deserialize, PartialOrd, Ord)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SectionName {
    Basic,
    Boolean,
    #[strum(serialize = "pair and list")]
    #[serde(rename = "pair and list")]
    PairAndList,
}

pub struct Section {
    pub name:   SectionName,
    pub levels: Vec1<Level>,
}

lazy_static! {
    pub static ref LEVELS: Vec1<Section> = load_all();
}

#[cfg(test)]
mod test {
    use super::{super::get_result, LEVELS};
    use crate::{interpreter::ConstantProvider, save_system::LevelResult};

    #[test]
    fn test_level_load() {
        // Can we load the levels without crashing?
        assert!(LEVELS.len() > 0);
    }

    #[test]
    fn test_solutions() {
        LEVELS.iter().flat_map(|s| s.levels.as_vec()).for_each(|l| {
            l.solutions.iter().for_each(|s| {
                let r = l
                    .test(s.chars(), ConstantProvider::all())
                    .expect(&format!("Failed to compile solution {}", s));

                r.runs.iter().for_each(|r| {
                    assert!(
                        r.is_correct(),
                        "Code '{}' does not reduce to '{}' on level '{}', instead reduced to {:?}",
                        r.test_expression,
                        r.expected_result,
                        l.name,
                        r.result.clone().map(|r| format!("{}", r)),
                    )
                });

                assert_eq!(get_result(&Ok(r)), LevelResult::Success);
            })
        });
    }

    #[test]
    fn test_wrong_solutions() {
        LEVELS.iter().flat_map(|s| s.levels.as_vec()).for_each(|l| {
            l.wrong_solutions.iter().for_each(|s| {
                assert_ne!(
                    get_result(&l.test(s.chars(), ConstantProvider::all())),
                    LevelResult::Success,
                    "Code was solution {} on level {}",
                    s,
                    l.name
                )
            })
        });
    }

    #[test]
    fn test_out_of_space() {
        // TODO: test if description of levels fit in screen
    }
}
