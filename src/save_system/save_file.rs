use app_dirs::*;
use std::{collections::HashMap, fs, option::NoneError, sync::Mutex};
use std::{io, path::PathBuf};

pub const APP_INFO: AppInfo = AppInfo {
    name: "functional",
    author: "yancouto",
};

#[derive(Debug)]
pub struct SaveProfile {
    path: PathBuf,
    current_save_file: Mutex<SaveFile>,
}

#[derive(Debug)]
pub enum LevelResult {
    Success,
    Failure,
}

const CURRENT_SAVE_VERSION: u32 = 0;

#[derive(Savefile, Debug, Default)]
struct LevelInfo {}

#[derive(Savefile, Debug, Default)]
struct SaveFile {
    level_info: HashMap<String, LevelInfo>,
}

impl SaveProfile {
    pub fn write_level(&self, level_name: &str, solution: u8, code: &str) {
        log::debug!("Writing solution {} of level {}", solution, level_name);
        if let Err(err) = self.write_level_impl(level_name, solution, code) {
            log::warn!("Error writing level: {:?}", err);
        }
    }

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
        let save_file = self.current_save_file.lock().unwrap();
        self.write("save.data", &*save_file);
    }
}

impl SaveProfile {
    fn load(path: PathBuf) -> Self {
        log::debug!("Loading save profile from {:?}", path);
        // TODO: load save file
        Self {
            path,
            current_save_file: Mutex::from(SaveFile::default()),
        }
    }

    fn write<T: savefile::WithSchema + savefile::Serialize>(&self, path: &str, data: &T) {
        log::debug!("Writing save file {}", path);
        // Better error message without having to implement a new error type?
        let result: Result<(), NoneError> = try {
            savefile::save_file(self.path.join(path).to_str()?, CURRENT_SAVE_VERSION, data).ok()?;
        };
        if result.is_err() {
            log::error!("Failed to write save file");
        }
    }

    fn write_level_impl(&self, level_name: &str, solution: u8, code: &str) -> io::Result<()> {
        let path = self
            .path
            .join(format!("levels/{}/{}.code", level_name, solution));
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, code)
    }

    fn read_level_impl(&self, level_name: &str, solution: u8) -> io::Result<String> {
        let path = self
            .path
            .join(format!("levels/{}/{}.code", level_name, solution));
        Ok(String::from_utf8_lossy(&fs::read(path)?).into_owned())
    }
}

impl Drop for SaveProfile {
    fn drop(&mut self) {
        log::debug!("Closing save profile at {:?}", self.path);
    }
}

/// Will create a folder if it doesn't exist
pub fn load_profile(name: &str) -> SaveProfile {
    let path = app_dir(
        AppDataType::UserConfig,
        &APP_INFO,
        &format!("savegames/{}", name),
    )
    .expect("Failed to load save file");
    SaveProfile::load(path)
}
