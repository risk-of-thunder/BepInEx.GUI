// Comment for enabling console
// #![windows_subsystem = "windows"]

use bepinex_gui_init_config::BepInExGUIInitConfig;
use eframe::egui::*;
use std::env;

mod bepinex_gui;
mod bepinex_gui_config;
mod bepinex_gui_init_config;
mod bepinex_log;
mod bepinex_mod;
mod egui_utils;
mod file_explorer_utils;
mod internal_logger;
mod network;
mod panic_handler;
mod process;
mod settings;
mod tab;
mod theme;
mod thunderstore;
mod window;

fn main() {
    internal_logger::init();

    panic_handler::init();

    let args: Vec<String> = env::args().collect();

    let gui = bepinex_gui::BepInExGUI::new(
        BepInExGUIInitConfig::from(&args).unwrap_or_else(BepInExGUIInitConfig::default),
    );

    let native_options = eframe::NativeOptions {
        min_window_size: Some(Vec2::new(884., 400.)),
        initial_window_size: Some(Vec2::new(993., 519.)),
        initial_centered: true,

        icon_data: Some(load_icon()),

        ..Default::default()
    };

    match eframe::run_native(
        settings::APP_NAME,
        native_options,
        Box::new(|cc| Box::new(gui.init(cc))),
    ) {
        Ok(_) => {}
        Err(res) => tracing::error!("{:?}", res),
    }
}

fn load_icon() -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../assets/ror2_discord_server_icon.png");
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
