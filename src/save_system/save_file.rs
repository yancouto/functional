use std::{collections::HashMap, fs, io, path::PathBuf};

use app_dirs::*;
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use savefile::SavefileError;

use crate::prelude::*;

pub const APP_INFO: AppInfo = AppInfo {
    name:   "functional",
    author: "yancouto",
};

#[derive(Debug)]
pub struct SaveProfile {
    name:              String,
    path:              PathBuf,
    current_save_file: Mutex<SaveFile>,
}

#[derive(Savefile, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelResult {
    Success {
        // This is the average number of reductions multiplied by 100
        reductions_x100: u32,
    },
    Failure,
    NotTried,
}

impl Default for LevelResult {
    fn default() -> Self { Self::NotTried }
}

impl LevelResult {
    fn get_best(self, other: LevelResult) -> LevelResult {
        debug_assert!(other != LevelResult::NotTried);
        match self {
            LevelResult::Success { reductions_x100 } => {
                if let LevelResult::Success { reductions_x100: r } = other {
                    LevelResult::Success {
                        reductions_x100: r.min(reductions_x100),
                    }
                } else {
                    self
                }
            },
            LevelResult::Failure => other,
            LevelResult::NotTried => other,
        }
    }
}

const CURRENT_SAVE_VERSION: u32 = 0;
const SAVE_FILE: &str = "save.data";

#[derive(Savefile, Debug, Default, Clone)]
pub struct LevelInfo {
    pub result:     LevelResult,
    pub best_score: Option<f32>,
}

#[derive(Savefile, Debug, Default)]
struct SaveFile {
    level_info: HashMap<String, LevelInfo>,
}

impl SaveProfile {
    #[cfg(test)]
    pub fn fake() -> Self {
        Self {
            name:              "test".to_string(),
            path:              PathBuf::new(),
            current_save_file: Mutex::new(SaveFile {
                level_info: HashMap::new(),
            }),
        }
    }

    pub fn name(&self) -> &str { &self.name }

    pub fn write_level(&self, level_name: &str, solution: u8, code: &str) {
        log::debug!("Writing solution {} of level {}", solution, level_name);
        self.write_level_impl(level_name, solution, code)
            .debug_expect("Error writing level");
    }

    pub fn level_code_file(&self, level_name: &str, solution: u8) -> PathBuf {
        self.path
            .join(format!("levels/{}/{}.code", level_name, solution))
    }

    #[allow(dead_code)]
    pub fn read_level(&self, level_name: &str, solution: u8) -> String {
        log::debug!("Reading solution {} of level {}", solution, level_name);
        self.read_level_impl(level_name, solution)
            .unwrap_or_else(|err| {
                if err.kind() != io::ErrorKind::NotFound {
                    log::warn!("Error reading level: {:?}", err);
                }
                String::new()
            })
    }

    pub fn mark_level_as_tried(&self, level_name: &str, result: LevelResult) {
        let mut save_file = self.current_save_file.lock();
        let stored_result = &mut save_file
            .level_info
            .entry(level_name.to_string())
            .or_default()
            .result;
        let new_result = stored_result.get_best(result);
        if *stored_result != new_result {
            *stored_result = result;
            self.write(SAVE_FILE, &*save_file);
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

    fn write_level_impl(&self, level_name: &str, solution: u8, code: &str) -> io::Result<()> {
        let path = self.level_code_file(level_name, solution);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, code)
    }

    fn read_level_impl(&self, level_name: &str, solution: u8) -> io::Result<String> {
        let path = self.level_code_file(level_name, solution);
        Ok(String::from_utf8_lossy(&fs::read(path)?).into_owned())
    }
}

impl Drop for SaveProfile {
    fn drop(&mut self) {
        log::debug!("Closing save profile at {:?}", self.path);
    }
}

fn get_save_profile(name: &str) -> PathBuf {
    app_dir(
        AppDataType::UserConfig,
        &APP_INFO,
        &format!("savegames/{}", name),
    )
    .expect("Failed to load save file")
}

fn get_common_file() -> Result<PathBuf, AppDirsError> {
    app_root(AppDataType::UserConfig, &APP_INFO).map(|p| p.join("common.data"))
}

/// Will create a folder if it doesn't exist
pub fn load_profile(name: &str) -> Result<SaveProfile, SavefileError> {
    SaveProfile::load(get_save_profile(name), name.to_string())
}

/// Deletes only save profile. Leaves code there.
pub fn reset_profile(name: &str) {
    fs::remove_file(get_save_profile(name).join(SAVE_FILE)).debug_unwrap();
}

const CURRENT_COMMON_VERSION: u32 = 0;
#[derive(Savefile, Debug, Default)]
pub struct CommonConfig {
    pub default_profile: Option<String>,
}

pub fn load_common() -> CommonConfig {
    get_common_file()
        .map(|p| read(p, CURRENT_COMMON_VERSION).debug_unwrap_or_default())
        .debug_unwrap_or_default()
}

pub fn write_common(common: CommonConfig) {
    write(get_common_file().unwrap(), CURRENT_COMMON_VERSION, &common);
}
