use std::{collections::HashMap, fs, io, path::PathBuf};

use directories::*;
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use savefile::SavefileError;

use crate::{interpreter::AccStats, levels::Level, prelude::*};

lazy_static! {
    pub static ref PROJECT_DIR: ProjectDirs = {
        let dirs =
            ProjectDirs::from("", "yancouto", "functional").expect("failed to create root dirs");
        std::fs::create_dir_all(dirs.cache_dir()).expect("Failed to create cache dir");
        std::fs::create_dir_all(dirs.data_dir()).expect("Failed to create data dir");
        dirs
    };
}

#[derive(Debug)]
pub struct SaveProfile {
    name:              String,
    path:              PathBuf,
    current_save_file: Mutex<SaveFile>,
}

#[derive(Savefile, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelResult {
    Success { stats: AccStats },
    Failure,
    NotTried,
}

impl Default for LevelResult {
    fn default() -> Self { Self::NotTried }
}

impl LevelResult {
    fn get_best(self, other: LevelResult) -> LevelResult {
        debug_assert!(!matches!(other, LevelResult::NotTried));
        match self {
            LevelResult::Success { stats } =>
                if let LevelResult::Success { stats: stats2 } = other {
                    LevelResult::Success {
                        stats: stats.best(stats2),
                    }
                } else {
                    self
                },
            LevelResult::Failure => other,
            LevelResult::NotTried => other,
        }
    }

    pub fn is_success(&self) -> bool { matches!(self, &LevelResult::Success { .. }) }
}

const CURRENT_SAVE_VERSION: u32 = 0;
const SAVE_FILE: &str = "save.data";

#[derive(Savefile, Debug, Default, Clone)]
pub struct LevelInfo {
    pub result: LevelResult,
}

#[derive(Savefile, Debug, Default)]
struct SaveFile {
    level_info: HashMap<String, LevelInfo>,
}

impl SaveProfile {
    #[cfg(test)]
    pub fn fake(completed_levels: Vec<&str>) -> Self {
        Self {
            name:              "test".to_string(),
            path:              PathBuf::new(),
            current_save_file: Mutex::new(SaveFile {
                level_info: completed_levels
                    .into_iter()
                    .map(|l| {
                        (
                            l.to_string(),
                            LevelInfo {
                                result: LevelResult::Success {
                                    stats: AccStats {
                                        reductions_x100: 100,
                                        functions:       100,
                                    },
                                },
                            },
                        )
                    })
                    .collect(),
            }),
        }
    }

    pub fn name(&self) -> &str { &self.name }

    pub fn write_level(&self, level: &Level, solution: u8, code: &str) {
        log::debug!(
            "Writing solution {} of level {}",
            solution,
            level.base().name
        );
        self.write_level_impl(&level, solution, code)
            .debug_expect("Error writing level");
    }

    pub fn level_code_file(&self, level: &Level, solution: u8) -> Option<PathBuf> {
        level
            .uuid()
            .map(|id| self.path.join(format!("levels/{}/{}.code", id, solution)))
    }

    pub fn mark_level_as_tried(&self, level: &Level, result: LevelResult) {
        if let Some(id) = level.uuid() {
            let mut save_file = self.current_save_file.lock();
            let stored_result = &mut save_file.level_info.entry(id).or_default().result;
            let new_result = stored_result.get_best(result);
            if *stored_result != new_result {
                *stored_result = new_result;
                self.write(SAVE_FILE, &*save_file);
            }
        }
    }

    pub fn get_levels_info(&self) -> MappedMutexGuard<HashMap<String, LevelInfo>> {
        MutexGuard::map(self.current_save_file.lock(), |s| &mut s.level_info)
    }

    pub fn reload(&self) -> Result<(), SavefileError> {
        *self.current_save_file.lock() = self.read(SAVE_FILE)?;
        Ok(())
    }
}

/// On file not found, return default value.
fn read<T: savefile::WithSchema + savefile::Deserialize + Default>(
    path: PathBuf,
    version: u32,
) -> Result<T, SavefileError> {
    match savefile::load_file(path.to_str().unwrap(), version) {
        Ok(value) => Ok(value),
        Err(SavefileError::IOError { io_error }) if io_error.kind() == io::ErrorKind::NotFound =>
            Ok(Default::default()),
        Err(err) => {
            log::error!("Failed to read save file {:?}: {:?}", path, err);
            Err(err)
        },
    }
}

fn write<T: savefile::WithSchema + savefile::Serialize>(path: PathBuf, version: u32, data: &T) {
    log::debug!("Writing save file {:?}", path);
    savefile::save_file(path.to_str().unwrap(), version, data)
        .debug_expect("Failed to write save file");
}

impl SaveProfile {
    fn load(path: PathBuf, name: String) -> Result<Self, SavefileError> {
        log::debug!("Loading save profile from {:?}", path);
        let this = Self {
            path,
            current_save_file: Mutex::from(SaveFile::default()),
            name,
        };
        this.reload()?;
        Ok(this)
    }

    fn read<T: savefile::WithSchema + savefile::Deserialize + Default>(
        &self,
        path: &str,
    ) -> Result<T, SavefileError> {
        log::debug!("Loading save file {}", path);
        read(self.path.join(path), CURRENT_SAVE_VERSION)
    }

    fn write<T: savefile::WithSchema + savefile::Serialize>(&self, path: &str, data: &T) {
        log::debug!("Writing save file {}", path);
        // Better error message without having to implement a new error type?
        write(self.path.join(path), CURRENT_SAVE_VERSION, data);
    }

    fn write_level_impl(&self, level: &Level, solution: u8, code: &str) -> io::Result<()> {
        if let Some(path) = self.level_code_file(level, solution) {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, code)
        } else {
            Ok(())
        }
    }
}

impl Drop for SaveProfile {
    fn drop(&mut self) {
        log::debug!("Closing save profile at {:?}", self.path);
    }
}

fn get_save_profile(name: &str) -> PathBuf { PROJECT_DIR.data_dir().join("savegames").join(name) }

fn get_common_file() -> PathBuf { PROJECT_DIR.data_dir().join("common.data") }

/// Will create a folder if it doesn't exist
pub fn load_profile(name: &str) -> Result<SaveProfile, SavefileError> {
    SaveProfile::load(get_save_profile(name), name.to_string())
}

/// Deletes only save profile. Leaves code there.
pub fn reset_profile(name: &str) {
    fs::remove_file(get_save_profile(name).join(SAVE_FILE)).debug_unwrap();
}

const CURRENT_COMMON_VERSION: u32 = 1;
#[derive(Savefile, Debug)]
pub struct CommonConfig {
    pub default_profile: Option<String>,
    #[savefile_default_val = "7"]
    #[savefile_versions = "1.."]
    pub volume:          u8,
}

impl Default for CommonConfig {
    fn default() -> Self {
        Self {
            default_profile: None,
            volume:          7,
        }
    }
}

pub fn load_common() -> CommonConfig {
    read(get_common_file(), CURRENT_COMMON_VERSION).debug_unwrap_or_default()
}

pub fn edit_and_save<R, F: FnOnce(&mut CommonConfig) -> R>(edit_fn: F) {
    let mut config = load_common();
    edit_fn(&mut config);
    write(get_common_file(), CURRENT_COMMON_VERSION, &config);
}
