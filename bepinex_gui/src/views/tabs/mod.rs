use crate::config::{launch::AppLaunchConfig, Config};

pub(crate) mod console;
pub(crate) mod general;
pub(crate) mod settings;

pub trait Tab {
    fn name(&self) -> &str;

    fn update_top_panel(
        &mut self,
        data: &AppLaunchConfig,
        gui_config: &mut Config,
        ui: &mut eframe::egui::Ui,
    );

    fn update(
        &mut self,
        data: &AppLaunchConfig,
        gui_config: &mut Config,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
    );
}
