// Comment for enabling console
#![windows_subsystem = "windows"]

use bepinex_gui_init_config::BepInExGUIInitConfig;
use eframe::egui::*;
use eframe::*;
use std::env;

mod bepinex_gui;
mod bepinex_gui_config;
mod bepinex_gui_init_config;
mod bepinex_log;
mod bepinex_mod;
mod colors;
mod egui_utils;
mod file_explorer_utils;
mod internal_logger;
mod network;
mod panic_handler;
mod process;
mod settings;
mod tab;
mod thunderstore;
mod window;

fn main() {
    internal_logger::init();

    panic_handler::init();

    let args: Vec<String> = env::args().collect();

    let gui = bepinex_gui::BepInExGUI::new(
        BepInExGUIInitConfig::from(&args).unwrap_or_else(BepInExGUIInitConfig::default),
    );

    let mut window_option = NativeOptions::default();
    window_option.initial_window_size = Some(Vec2::new(993., 519.));
    window_option.initial_centered = true;

    match eframe::run_native(
        settings::APP_NAME,
        window_option,
        Box::new(|cc| Box::new(gui.init(cc))),
    ) {
        Ok(_) => {}
        Err(res) => tracing::error!("{:?}", res),
    }
}
