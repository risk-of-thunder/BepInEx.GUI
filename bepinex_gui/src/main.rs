// Disable console
// #![windows_subsystem = "windows"]

#[macro_use]
extern crate lazy_static;

use eframe::egui::*;
use eframe::*;
use std::env;
use sysinfo::Pid;

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
mod thunderstore_communities;

fn main() {
    tracing_subscriber::fmt::init();

    let mut args: Vec<String> = env::args().collect();

    check_args_and_fill_if_needed(&mut args);

    let game_folder_full_path = &args[3];
    let bepinex_root_full_path = &args[4];
    let target_process_id = args[5].parse::<Pid>().unwrap();
    let log_socket_port_receiver = args[6].parse::<u16>().unwrap();

    let gui = bepinex_gui::BepInExGUI::new(
        game_folder_full_path,
        bepinex_root_full_path,
        target_process_id,
        log_socket_port_receiver,
    );

    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(993., 519.));
    win_option.initial_window_pos_centered = true;
    win_option.window_title =
        Some(settings::APP_NAME.to_string() + " " + &args[1] + " - " + &args[2]);

    eframe::run_native(
        settings::APP_NAME,
        win_option,
        Box::new(|cc| Box::new(gui.init(cc))),
    );
}

fn check_args_and_fill_if_needed(args: &mut Vec<String>) {
    if args.len() == 1 {
        args.push("5.4.19".to_string()); // BepInEx Version String
        args.push("Risk of Rain 2".to_string()); // Target Process Name
        args.push("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Risk of Rain 2".to_string());
        args.push(
            "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Risk of Rain 2\\BepInEx"
                .to_string(),
        );
        args.push("17584".to_string()); // Target Process Id
        args.push("27090".to_string()); // Socket port used for comm with the bep gui patcher
    } else if args.len() != 7 {
        panic!("PROBLEM WITH ARGS {:?}", args);
    }
}
