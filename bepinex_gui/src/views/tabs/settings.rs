use std::sync::atomic::Ordering;

use eframe::egui::{CentralPanel, Context};

use crate::{
    config::{launch::AppLaunchConfig, Config},
    views::components,
};

use super::Tab;

pub struct SettingsTab {}

impl SettingsTab {
    pub fn new() -> Self {
        Self {}
    }

    fn render(&mut self, gui_config: &mut Config, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            let mut button_size = ui.available_size() / 3.;
            button_size.x = ui.available_width();

            render_close_window_when_game_loaded_checkbox(ui, button_size, gui_config);

            render_close_window_when_game_closes_checkbox(gui_config, ui, button_size);

            render_switch_theme_button(gui_config, ui, button_size);
        });
    }
}

fn render_close_window_when_game_loaded_checkbox(
    ui: &mut eframe::egui::Ui,
    button_size: eframe::epaint::Vec2,
    gui_config: &mut Config,
) {
    if components::checkbox(
        &mut gui_config.close_window_when_game_loaded,
        "Close this window when the game is loaded",
        ui,
        button_size,
        20.,
    ) {
        _ = gui_config.save_bepinex_toml_cfg_file();
    }
}

fn render_close_window_when_game_closes_checkbox(
    gui_config: &mut Config,
    ui: &mut eframe::egui::Ui,
    button_size: eframe::epaint::Vec2,
) {
    let close_window_when_game_closes = &mut gui_config
        .close_window_when_game_closes
        .load(Ordering::Relaxed);

    if components::checkbox(
        close_window_when_game_closes,
        "Close this window when the game closes",
        ui,
        button_size,
        20.,
    ) {
        gui_config
            .close_window_when_game_closes
            .store(*close_window_when_game_closes, Ordering::Relaxed);

        _ = gui_config.save_bepinex_toml_cfg_file();
    }
}

fn render_switch_theme_button(
    gui_config: &mut Config,
    ui: &mut eframe::egui::Ui,
    button_size: eframe::epaint::Vec2,
) {
    let is_dark_mode = gui_config.dark_mode;

    if components::colored_button(
        if is_dark_mode {
            "Switch to light theme 🌞"
        } else {
            "Switch to dark theme 🌙"
        },
        ui,
        button_size,
        20.,
        Some(ui.style().visuals.widgets.noninteractive.bg_fill),
    ) {
        gui_config.dark_mode = !gui_config.dark_mode;
        gui_config.theme_just_changed = true;
    }
}

impl Tab for SettingsTab {
    fn name(&self) -> &str {
        "Settings"
    }

    fn update_top_panel(
        &mut self,
        _data: &AppLaunchConfig,
        _gui_config: &mut Config,
        _ui: &mut eframe::egui::Ui,
    ) {
    }

    fn update(
        &mut self,
        _data: &AppLaunchConfig,
        gui_config: &mut Config,
        ctx: &eframe::egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        self.render(gui_config, ctx);
    }
}
