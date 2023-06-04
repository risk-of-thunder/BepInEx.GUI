use std::sync::atomic::Ordering;

use eframe::Frame;

use crate::app::BepInExGUI;

pub mod file_explorer_utils;
pub mod network;
mod panic_handler;
pub mod process;
mod reset_app_if_window_hang;
pub mod thunderstore;
pub mod window;

impl BepInExGUI {
    pub(crate) fn backend_update(&mut self, frame: &mut Frame) {
        // Ideally this would be done in a init function, not constantly checked in an update function
        // L from eframe
        if !self.is_window_title_set {
            frame.set_window_title(self.app_launch_config.window_title());
            self.is_window_title_set = true;
        }

        if self.should_exit_app.load(Ordering::Relaxed) {
            frame.close();
        }
    }
}

pub fn init() {
    panic_handler::init();

    reset_app_if_window_hang::spawn_thread();
}
