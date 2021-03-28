use app_dirs::*;
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use std::{collections::HashMap, fs};
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

#[derive(Savefile, Debug, Clone, Copy)]
pub enum LevelResult {
    Success,
    Failure,
    NotTried,
}

impl Default for LevelResult {
    fn default() -> Self {
        Self::NotTried
    }
}

const CURRENT_SAVE_VERSION: u32 = 0;

#[derive(Savefile, Debug, Default, Clone)]
pub struct LevelInfo {
    pub result: LevelResult,
}

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
        let mut save_file = self.current_save_file.lock();
        save_file
            .level_info
            .entry(level_name.to_string())
            .or_default()
            .result = result;
        self.write("save.data", &*save_file);
    }

    pub fn get_levels_info(&self) -> MappedMutexGuard<HashMap<String, LevelInfo>> {
        MutexGuard::map(self.current_save_file.lock(), |s| &mut s.level_info)
    }

    pub fn reload(&self) {
        self.read("save.data", &mut *self.current_save_file.lock());
    }
}

impl SaveProfile {
    fn load(path: PathBuf) -> Self {
        log::debug!("Loading save profile from {:?}", path);
        // TODO: load save file
        let this = Self {
            path,
            current_save_file: Mutex::from(SaveFile::default()),
        };
        this.reload();
        this
    }

    fn read<T: savefile::WithSchema + savefile::Deserialize + Default>(
        &self,
        path: &str,
        data: &mut T,
    ) {
        log::debug!("Loading save file {}", path);
        // Better error message without having to implement a new error type?
        let result =
            savefile::load_file(self.path.join(path).to_str().unwrap(), CURRENT_SAVE_VERSION);
        match result {
            Ok(value) => *data = value,
            Err(savefile::SavefileError::IOError { io_error })
                if io_error.kind() == io::ErrorKind::NotFound =>
            {
                *data = Default::default()
            }
            Err(err) => log::error!("Failed to read save file {}: {:?}", path, err),
        }
    }

    fn write<T: savefile::WithSchema + savefile::Serialize>(&self, path: &str, data: &T) {
        log::debug!("Writing save file {}", path);
        // Better error message without having to implement a new error type?
        let result = savefile::save_file(
            self.path.join(path).to_str().unwrap(),
            CURRENT_SAVE_VERSION,
            data,
        );
        if let Err(err) = result {
            log::error!("Failed to write save file {}: {:?}", path, err);
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
