use app_dirs::*;
use std::path::PathBuf;

const APP_INFO: AppInfo = AppInfo {
    name: "functional",
    author: "Yan Couto",
};

#[derive(Debug)]
pub struct SaveProfile {
    path: PathBuf,
}

impl SaveProfile {
    fn load(path: PathBuf) -> Self {
        println!("Creating save file from {:?}", path);
        Self { path }
    }
}

impl Drop for SaveProfile {
    fn drop(&mut self) {
        println!("Dropping save file object from {:?}", self.path);
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
