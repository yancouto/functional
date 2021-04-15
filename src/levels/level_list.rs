use serde::Deserialize;

use super::{Level, TestCase};
use crate::prelude::*;
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct JLevel {
    name:        String,
    description: String,
    extra_info:  Option<String>,
    test_cases:  Vec<(String, String)>,
    solutions:   Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct JSection {
    name:   String,
    levels: Vec<JLevel>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct JLevelConfig {
    sections: Vec<JSection>,
}

const RAW_LEVEL_CONFIG: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/level_config.json"));

fn load_all() -> Vec1<Section> {
    let config: JLevelConfig = serde_json::from_slice(RAW_LEVEL_CONFIG).expect("Invalid json");
    Vec1::try_from_vec(
        config
            .sections
            .into_iter()
            .map(|s| Section {
                name:   s.name,
                levels: Vec1::try_from_vec(
                    s.levels
                        .into_iter()
                        .map(|l| Level {
                            name:        l.name,
                            description: l.description,
                            extra_info:  l.extra_info,
                            test_cases:  l
                                .test_cases
                                .into_iter()
                                .map(|t| TestCase::from(&t.0, &t.1))
                                .collect(),
                            solutions:   l.solutions,
                        })
                        .collect(),
                )
                .unwrap(),
            })
            .collect(),
    )
    .unwrap()
}

pub struct Section {
    pub name:   String,
    pub levels: Vec1<Level>,
}

lazy_static! {
    pub static ref LEVELS: Vec1<Section> = load_all();
}

#[cfg(test)]
mod test {
    use super::{super::get_result, LEVELS};
    use crate::save_system::LevelResult;

    #[test]
    fn test_level_load() {
        // Can we load the levels without crashing?
        assert!(LEVELS.len() > 0);
    }

    #[test]
    fn test_solutions() {
        LEVELS.iter().flat_map(|s| s.levels.as_vec()).for_each(|l| {
            l.solutions
                .iter()
                .for_each(|s| assert_eq!(get_result(&l.test(s.chars())), LevelResult::Success))
        });
    }
}
