use app_dirs::*;
use std::{borrow::Cow, fs};
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
        println!("Loading save profile from {:?}", path);
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
        println!("Writing solution {} of level {}", solution, level_name);
        if let Err(err) = self.write_level_impl(level_name, solution, code) {
            println!("Error writing level: {:?}", err);
        }
    }

    fn read_level_impl(&self, level_name: &str, solution: u8) -> io::Result<String> {
        let path = self
            .path
            .join(format!("levels/{}/{}.code", level_name, solution));
        Ok(String::from_utf8_lossy(&fs::read(path)?).into_owned())
    }

    pub fn read_level(&self, level_name: &str, solution: u8) -> String {
        println!("Reading solution {} of level {}", solution, level_name);
        self.read_level_impl(level_name, solution)
            .unwrap_or_else(|err| {
                println!("Error reading level: {:?}", err);
                String::new()
            })
    }
}

impl Drop for SaveProfile {
    fn drop(&mut self) {
        println!("Closing save profile at {:?}", self.path);
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
