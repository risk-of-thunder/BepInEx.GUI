use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek, SeekFrom, Write},
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use serde::*;

use crate::{app, data::bepinex_log::LogLevel};

pub mod launch;

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)]
    pub theme_just_changed: bool,

    pub dark_mode: bool,

    // For showing or not the disclaimer that explains how to report bugs / post log file in the discord
    pub first_time: bool,

    // For showing or not the disclaimer that explains the console tab purpose
    pub first_time_console_disclaimer: bool,

    // For remembering the last selected tab
    pub selected_tab_index: usize,

    // For remembering the selected log level filter (Console tab)
    pub log_level_filter: LogLevel,

    // For remembering if the console should scroll to the bottom when a new log arrive
    pub log_auto_scroll_to_bottom: bool,

    // Skipped because those fields are saved through the regular bepinex config system
    #[serde(skip)]
    pub close_window_when_game_loaded: bool,

    #[serde(skip)]
    pub close_window_when_game_closes: Arc<AtomicBool>,

    #[serde(skip)]
    pub bepinex_gui_csharp_cfg_full_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme_just_changed: true,
            dark_mode: true,
            first_time: true,
            first_time_console_disclaimer: true,
            selected_tab_index: 0,
            log_level_filter: LogLevel::All,
            log_auto_scroll_to_bottom: true,
            close_window_when_game_loaded: false,
            close_window_when_game_closes: Arc::new(AtomicBool::new(true)),
            bepinex_gui_csharp_cfg_full_path: Default::default(),
        }
    }
}

impl Config {
    pub fn read_bepinex_toml_cfg_file(&mut self) -> io::Result<()> {
        let file = File::open(&self.bepinex_gui_csharp_cfg_full_path)?;
        let reader = BufReader::new(file);

        let mut current_settings_category_name: &str;

        for line_ in reader.lines() {
            match line_ {
                Ok(line) => {
                    if line.starts_with('[') {
                        current_settings_category_name = line.split('[').collect::<Vec<&str>>()[1]
                            .split(']')
                            .collect::<Vec<&str>>()[0];
                        tracing::info!(
                            "current_settings_category_name: {}",
                            current_settings_category_name
                        );
                    } else if line.starts_with("##") {
                    } else if line.starts_with("# ") {
                    } else if line.contains('=') {
                        let setting = line.split('=').collect::<Vec<&str>>();
                        let setting_name = setting[0].trim();
                        let settings_current_value = setting[1].trim();

                        let bool_setting = settings_current_value.parse::<bool>();
                        if let Ok(bool_value) = bool_setting {
                            tracing::info!("{:?}: {:?}", setting_name, bool_value);
                            if setting_name == "Close Window When Game Loaded" {
                                self.close_window_when_game_loaded = bool_value;
                            } else if setting_name == "Close Window When Game Closes" {
                                self.close_window_when_game_closes
                                    .store(bool_value, Ordering::Relaxed);
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }

        Ok(())
    }

    pub fn save_bepinex_toml_cfg_file(&self) -> io::Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.bepinex_gui_csharp_cfg_full_path)?;
        let reader = BufReader::new(&file);

        let mut lines: Vec<String> = Vec::new();

        for line_ in reader.lines() {
            match line_ {
                Ok(line_) => {
                    let mut line = line_.to_string();
                    if line_.contains('=') {
                        let setting = line_.split('=').collect::<Vec<&str>>();
                        let setting_name = setting[0].trim();

                        if setting_name == "Close Window When Game Loaded" {
                            line = format!(
                                "Close Window When Game Loaded = {}",
                                self.close_window_when_game_loaded
                            );
                        } else if setting_name == "Close Window When Game Closes" {
                            line = format!(
                                "Close Window When Game Closes = {}",
                                self.close_window_when_game_closes.load(Ordering::Relaxed)
                            );
                        }
                    }

                    line += "\n";
                    lines.push(line);
                }
                Err(err) => return Err(err),
            }
        }

        file.seek(SeekFrom::Start(0))?;
        file.set_len(0)?;

        for line in &lines {
            match file.write(line.as_bytes()) {
                Ok(_) => {}
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }
}

pub fn get_app_ron_file_full_path() -> Option<PathBuf> {
    if let Some(proj_dirs) = directories_next::ProjectDirs::from("", "", app::NAME) {
        let data_dir = proj_dirs.data_dir().to_path_buf();
        Some(data_dir.join("app.ron"))
    } else {
        None
    }
}
