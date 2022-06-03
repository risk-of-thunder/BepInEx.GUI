use std::path::PathBuf;

pub(crate) const APP_NAME: &str = "BepInExGUI";

pub(crate) fn get_directory_full_path() -> Option<PathBuf> {
    if let Some(proj_dirs) = directories_next::ProjectDirs::from("", "", APP_NAME) {
        Some(proj_dirs.data_dir().to_path_buf())
    } else {
        None
    }
}

pub(crate) fn get_log_file_full_path() -> Option<PathBuf> {
    if let Some(directory_full_path) = get_directory_full_path() {
        Some(directory_full_path.join("log.txt"))
    } else {
        None
    }
}
