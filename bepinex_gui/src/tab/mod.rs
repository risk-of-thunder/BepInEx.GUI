use crate::{bepinex_gui_config::BepInExGUIConfig, bepinex_gui_init_config::BepInExGUIInitConfig};

pub(crate) mod console_tab;
pub(crate) mod general_tab;
pub(crate) mod settings_tab;

pub trait Tab {
    fn name(&self) -> &str;

    fn update_top_panel(
        &mut self,
        data: &BepInExGUIInitConfig,
        gui_config: &mut BepInExGUIConfig,
        ui: &mut eframe::egui::Ui,
    );

    fn update(
        &mut self,
        data: &BepInExGUIInitConfig,
        gui_config: &mut BepInExGUIConfig,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
    );
}
