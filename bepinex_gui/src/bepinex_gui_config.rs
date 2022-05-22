use eframe::emath::*;
use serde::*;

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
