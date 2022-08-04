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
        if std::fs::create_dir_all(&directory_full_path).is_ok() {
            return Some(directory_full_path.join("log.txt"));
        }
    }

    None
}

pub(crate) fn get_tmp_zip_log_full_path() -> Option<PathBuf> {
    if let Some(directory_full_path) = get_directory_full_path() {
        if std::fs::create_dir_all(&directory_full_path).is_ok() {
            return Some(directory_full_path.join("log_file.zip"));
        }
    }

    None
}
