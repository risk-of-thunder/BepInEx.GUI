use crate::bepinex_gui_config::BepInExGUIConfig;

pub(crate) mod console_tab;
pub(crate) mod general_tab;
pub(crate) mod settings_tab;

pub trait Tab {
    fn name(&self) -> &str;

    fn update_top_panel(&mut self, gui_config: &mut BepInExGUIConfig, ui: &mut eframe::egui::Ui);

    fn update(
        &mut self,
        gui_config: &mut BepInExGUIConfig,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
    );

    fn is_dev_only(&self) -> bool;
}
