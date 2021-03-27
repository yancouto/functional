use app_dirs::*;
use std::path::PathBuf;

const APP_INFO: AppInfo = AppInfo {
    name: "functional",
    author: "Yan Couto",
};

pub struct SaveProfile {}

impl SaveProfile {
    fn load(path: PathBuf) -> Self {
        Self {}
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
