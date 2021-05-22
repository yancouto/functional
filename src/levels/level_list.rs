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
    #[serde(default)]
    pub extra_info_is_hint:     bool,
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
    pub tests:    Vec<(String, String)>,
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
                            .map(|(idx, l)| {
                                if l.extra_info_is_hint {
                                    debug_assert!(l.extra_info.is_some());
                                }
                                Level {
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
                                    extra_info_is_hint: l.extra_info_is_hint,
                                }
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

#[derive(
    Debug,
    strum::Display,
    strum::EnumIter,
    PartialEq,
    Eq,
    Hash,
    Clone,
    Copy,
    Deserialize,
    PartialOrd,
    Ord,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SectionName {
    Basic,
    Boolean,
    #[strum(serialize = "pair and list")]
    #[serde(rename = "pair and list")]
    PairAndList,
    Recursion,
    Numerals,
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
    use std::{collections::HashSet, time::Duration};

    use rayon::prelude::*;
    use strum::IntoEnumIterator;

    use super::{super::get_result, *};
    use crate::{
        interpreter::{interpreter::test::interpret_ok, ConstantProvider}, save_system::LevelResult
    };

    #[test]
    fn test_level_load() {
        // Can we load the levels without crashing?
        assert!(LEVELS.len() > 0);
    }

    #[test]
    fn unique_names() {
        let names = LEVELS
            .iter()
            .flat_map(|s| s.levels.as_vec())
            .map(|l| l.name.clone())
            .collect::<HashSet<_>>();
        assert_eq!(
            names.len(),
            LEVELS.iter().flat_map(|s| s.levels.as_vec()).count(),
            "Some name is duplicated in the levels definition"
        );
    }

    #[test]
    fn test_jsonnet_tests() {
        raw_load_level_config()
            .tests
            .into_iter()
            .for_each(|(a, b)| {
                assert_eq!(interpret_ok(&a), interpret_ok(&b), "'{}' != '{}'", &a, &b)
            });
    }

    fn solution_section(section: SectionName) {
        LEVELS
            .par_iter()
            .filter(|s| s.name == section)
            .flat_map(|s| s.levels.as_vec().par_iter())
            .for_each(|l| {
                l.solutions.par_iter().for_each(|s| {
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
                        r.result.clone().map(|r| format!("{}", r.term)),
                    )
                    });

                    assert_matches!(get_result(&Ok(r)), LevelResult::Success { .. });
                })
            });
    }

    fn all_sections(sections: Vec<SectionName>) {
        assert_eq!(
            SectionName::iter().collect::<HashSet<_>>(),
            sections.into_iter().collect::<HashSet<_>>()
        );
    }

    // Need to do this because we want proper parallel subtests for each section
    macro_rules! solution_tests {
        ($($name:ident),*) => {
        $(
            #[test]
            #[allow(non_snake_case)]
            fn $name () {
                solution_section(SectionName::$name);
            }

        )*
            #[test]
            fn test_cover_all_sections() {
                all_sections(vec![$(SectionName::$name),*])
            }
        }
    }

    solution_tests!(Basic, Boolean, Numerals, PairAndList, Recursion);

    #[test]
    fn test_wrong_solutions() {
        LEVELS.iter().flat_map(|s| s.levels.as_vec()).for_each(|l| {
            l.wrong_solutions.iter().for_each(|s| {
                assert_matches!(
                    get_result(&l.test(s.chars(), ConstantProvider::all())),
                    LevelResult::Failure,
                    "Code was solution {} on level {}",
                    s,
                    l.name
                )
            })
        });
    }

    fn fake_bterm() -> bl::BTerm {
        bl::BTerm {
            width_pixels:           W as u32,
            height_pixels:          H as u32,
            original_height_pixels: H as u32,
            original_width_pixels:  W as u32,
            fps:                    30.0,
            frame_time_ms:          10.0,
            active_console:         0,
            key:                    None,
            mouse_pos:              (0, 0),
            left_click:             false,
            shift:                  false,
            control:                false,
            alt:                    false,
            web_button:             None,
            quitting:               false,
            post_scanlines:         false,
            post_screenburn:        false,
            screen_burn_color:      bl::RGB::from_u8(0, 1, 1),
        }
    }

    #[test]
    fn test_out_of_space() {
        // Extract this test stuff if we need more
        use crate::{
            drawables::BasicTextEditor, gamestates::{
                base::{with_current_console, EventTickData, GSData, TickData}, editor::EditorState
            }, save_system::SaveProfile
        };
        let fake_profile = Rc::new(SaveProfile::fake(vec![]));
        let mut term = fake_bterm();
        bl::BACKEND_INTERNAL
            .lock()
            .consoles
            .push(bl::DisplayConsole {
                console:      box bl::VirtualConsole::new(bl::Point::new(W, H)),
                shader_index: 0,
                font_index:   0,
            });
        LEVELS.iter().flat_map(|s| s.levels.as_vec()).for_each(|l| {
            let mut gs_data = GSData {
                cur:  box EditorState::<BasicTextEditor>::new(&l, fake_profile.clone()),
                time: Duration::new(0, 0),
            };
            with_current_console(0, |mut c| {
                let input = bl::INPUT.lock();
                let data = TickData::new(
                    &gs_data,
                    EventTickData::default(),
                    &mut c,
                    &mut term,
                    &input,
                    None,
                );
                // Should not panic
                gs_data.cur.tick(data);
            })
        });
    }
}
