use app_dirs::*;
use std::fs;
use std::{io, path::PathBuf};

const APP_INFO: AppInfo = AppInfo {
    name: "functional",
    author: "yancouto",
};

#[derive(Debug)]
pub struct SaveProfile {
    path: PathBuf,
}

impl SaveProfile {
    fn load(path: PathBuf) -> Self {
        log::debug!("Loading save profile from {:?}", path);
        Self { path }
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

    pub fn write_level(&self, level_name: &str, solution: u8, code: &str) {
        log::debug!("Writing solution {} of level {}", solution, level_name);
        if let Err(err) = self.write_level_impl(level_name, solution, code) {
            log::warn!("Error writing level: {:?}", err);
        }
    }

    fn read_level_impl(&self, level_name: &str, solution: u8) -> io::Result<String> {
        let path = self
            .path
            .join(format!("levels/{}/{}.code", level_name, solution));
        Ok(String::from_utf8_lossy(&fs::read(path)?).into_owned())
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
