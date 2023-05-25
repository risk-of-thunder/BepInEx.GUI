use std::path::PathBuf;

use crate::app;

pub(crate) fn get_app_config_directory() -> Option<PathBuf> {
    if let Some(proj_dirs) = directories_next::ProjectDirs::from("", "", app::NAME) {
        Some(proj_dirs.data_dir().to_path_buf())
    } else {
        None
    }
}
