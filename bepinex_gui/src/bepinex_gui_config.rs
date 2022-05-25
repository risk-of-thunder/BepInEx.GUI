use std::path::PathBuf;

use eframe::emath::*;
use serde::*;

use crate::settings;

#[derive(Serialize, Deserialize)]
pub struct BepInExGUIConfig {
    pub dark_mode: bool,
    pub window_pos: Pos2,
    pub first_time: bool,
    pub is_dev: bool,
    pub selected_tab_index: usize,
}

impl Default for BepInExGUIConfig {
    fn default() -> Self {
        Self {
            dark_mode: true,
            window_pos: Default::default(),
            first_time: true,
            is_dev: false,
            selected_tab_index: 0,
        }
    }
}

impl BepInExGUIConfig {
    fn get_file_full_path() -> Option<PathBuf> {
        if let Some(proj_dirs) = directories_next::ProjectDirs::from("", "", settings::APP_NAME) {
            let data_dir = proj_dirs.data_dir().to_path_buf();
            Some(data_dir.join("app.ron"))
        } else {
            None
        }
    }
}
