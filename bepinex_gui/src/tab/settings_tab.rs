use std::sync::atomic::Ordering;

use eframe::{
    egui::{CentralPanel, Checkbox, Context, RichText},
    epaint::FontId,
};

use crate::bepinex_gui_config::BepInExGUIConfig;

use super::Tab;

pub struct SettingsTab {}

impl SettingsTab {
    pub fn new() -> Self {
        Self {}
    }

    fn render(&mut self, gui_config: &mut BepInExGUIConfig, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            let mut button_size = ui.available_size() / 2.;
            button_size.x = ui.available_width();

            if ui
                .add_sized(
                    button_size,
                    Checkbox::new(
                        &mut gui_config.close_window_when_game_loaded,
                        RichText::new("Close Window When Game Loaded")
                            .font(FontId::proportional(20.)),
                    ),
                )
                .clicked()
            {
                _ = gui_config.save_csharp_cfg_file();
            }

            let close_window_when_game_closes = &mut gui_config
                .close_window_when_game_closes
                .load(Ordering::Relaxed);

            if ui
                .add_sized(
                    button_size,
                    Checkbox::new(
                        close_window_when_game_closes,
                        RichText::new("Close Window When Game Closes")
                            .font(FontId::proportional(20.)),
                    ),
                )
                .clicked()
            {
                gui_config
                    .close_window_when_game_closes
                    .store(*close_window_when_game_closes, Ordering::Relaxed);
                _ = gui_config.save_csharp_cfg_file();
            }
        });
    }
}

impl Tab for SettingsTab {
    fn name(&self) -> &str {
        "Settings"
    }

    fn update_top_panel(&mut self, _gui_config: &mut BepInExGUIConfig, _ui: &mut eframe::egui::Ui) {
    }

    fn update(
        &mut self,
        gui_config: &mut BepInExGUIConfig,
        ctx: &eframe::egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        self.render(gui_config, ctx);
    }
}
