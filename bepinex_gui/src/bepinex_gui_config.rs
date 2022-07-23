use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek, SeekFrom, Write},
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use eframe::emath::*;
use serde::*;

use crate::settings;

#[derive(Serialize, Deserialize)]
pub struct BepInExGUIConfig {
    pub dark_mode: bool,
    pub window_pos: Pos2,
    pub first_time: bool,
    pub is_dev: Arc<AtomicBool>,
    pub selected_tab_index: usize,

    // those fields are saved through the regular bepinex config system
    #[serde(skip)]
    pub close_window_when_game_loaded: bool,

    #[serde(skip)]
    pub close_window_when_game_closes: Arc<AtomicBool>,

    #[serde(skip)]
    pub bepinex_gui_csharp_cfg_full_path: PathBuf,
}

impl Default for BepInExGUIConfig {
    fn default() -> Self {
        Self {
            dark_mode: true,
            window_pos: Default::default(),
            first_time: true,
            is_dev: Arc::new(AtomicBool::new(false)),
            selected_tab_index: 0,
            close_window_when_game_loaded: false,
            close_window_when_game_closes: Arc::new(AtomicBool::new(true)),
            bepinex_gui_csharp_cfg_full_path: Default::default(),
        }
    }
}

impl BepInExGUIConfig {
    #[allow(dead_code)]
    pub fn get_file_full_path() -> Option<PathBuf> {
        if let Some(proj_dirs) = directories_next::ProjectDirs::from("", "", settings::APP_NAME) {
            let data_dir = proj_dirs.data_dir().to_path_buf();
            Some(data_dir.join("app.ron"))
        } else {
            None
        }
    }

    pub fn read_csharp_cfg_file(&mut self) -> io::Result<()> {
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

    pub fn save_csharp_cfg_file(&self) -> io::Result<()> {
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
