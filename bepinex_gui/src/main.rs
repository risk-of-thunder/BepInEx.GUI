// #![windows_subsystem = "windows"]

use bepinex_gui::BepInExGUI;
use eframe::{egui::Vec2, run_native, NativeOptions};

fn main() {
    tracing_subscriber::fmt::init();

    let gui = BepInExGUI::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(993., 519.));
    run_native(
        "BepInEx GUI",
        win_option,
        Box::new(|cc| Box::new(gui.init(cc))),
    );
}
