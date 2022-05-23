// Disable console
// #![windows_subsystem = "windows"]

#[macro_use]
extern crate lazy_static;

use eframe::egui::*;
use eframe::*;

mod bepinex_gui;
mod bepinex_gui_config;
mod bepinex_log;
mod check_if_dev;
mod colors;
mod egui_utils;
mod log_receiver_thread;
mod packet_protocol;
mod settings;
mod tab;

fn main() {
    tracing_subscriber::fmt::init();

    let gui = bepinex_gui::BepInExGUI::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(993., 519.));
    win_option.initial_window_pos_centered = true;
    eframe::run_native(
        settings::APP_NAME,
        win_option,
        Box::new(|cc| Box::new(gui.init(cc))),
    );
}
