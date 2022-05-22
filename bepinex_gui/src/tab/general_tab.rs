use crate::bepinex_gui_config::{self, BepInExGUIConfig};

use super::Tab;

pub struct GeneralTab {}

impl GeneralTab {
    pub fn new() -> Self {
        Self {}
    }
}

impl Tab for GeneralTab {
    fn name(&self) -> &str {
        "General"
    }

    fn update_top_panel(&mut self, gui_config: &mut BepInExGUIConfig, ui: &mut eframe::egui::Ui) {}

    fn update(
        &mut self,
        gui_config: &mut BepInExGUIConfig,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
    ) {
        ()
    }

    fn require_dev_check(&self) -> bool {
        false
    }
}
