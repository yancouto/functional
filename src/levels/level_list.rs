use super::{Level, TestCase};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct JLevel {
    name: String,
    description: String,
    extra_info: Option<String>,
    test_cases: Vec<(String, String)>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct JSection {
    name: String,
    levels: Vec<JLevel>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct JLevelConfig {
    sections: Vec<JSection>,
}

const RAW_LEVEL_CONFIG: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/level_config.json"));

fn load_using_jsonnet() -> Vec<Level> {
    let config: JLevelConfig = serde_json::from_slice(RAW_LEVEL_CONFIG).expect("Invalid json");
    config
        .sections
        .into_iter()
        .flat_map(|s| s.levels)
        .map(|l| Level {
            name: l.name,
            description: l.description,
            extra_info: l.extra_info,
            test_cases: l
                .test_cases
                .into_iter()
                .map(|t| TestCase::from(&t.0, &t.1))
                .collect(),
        })
        .collect()
}

lazy_static! {
    pub static ref LEVELS: Vec<Level> = load_using_jsonnet();
}
