use std::path::PathBuf;

use crate::app;

pub fn get_app_config_directory() -> Option<PathBuf> {
    directories_next::ProjectDirs::from("", "", app::NAME)
        .map(|proj_dirs| proj_dirs.data_dir().to_path_buf())
}
