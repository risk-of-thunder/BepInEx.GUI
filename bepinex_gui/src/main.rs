// Uncomment for disabling console
#![windows_subsystem = "windows"]

use eframe::egui::*;
use eframe::*;
use std::env;
use std::fs::File;
use std::sync::Mutex;
use sysinfo::Pid;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, Registry};

mod bepinex_gui;
mod bepinex_gui_config;
mod bepinex_log;
mod bepinex_mod;
mod colors;
mod egui_utils;
mod log_receiver_thread;
mod packet_protocol;
mod settings;
mod tab;
mod thunderstore_communities;

fn main() {
    init_logger();

    let mut args: Vec<String> = env::args().collect();

    check_args_and_fill_if_needed(&mut args);

    let bepinex_version = &args[1];
    let target_name = &args[2];
    let game_folder_full_path = &args[3];
    let bepinex_root_full_path = &args[4];
    let bepinex_log_output_file_full_path = &args[5];
    let bepinex_gui_csharp_cfg_full_path = &args[6];
    let target_process_id = args[7].parse::<Pid>().unwrap();
    let log_socket_port_receiver = args[8].parse::<u16>().unwrap();

    let gui = bepinex_gui::BepInExGUI::new(
        target_name.into(),
        game_folder_full_path.into(),
        bepinex_root_full_path.into(),
        bepinex_log_output_file_full_path.into(),
        bepinex_gui_csharp_cfg_full_path.into(),
        target_process_id,
        log_socket_port_receiver,
    );

    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(993., 519.));
    win_option.initial_window_pos_centered = true;
    win_option.window_title =
        Some(settings::APP_NAME.to_string() + " " + bepinex_version + " - " + target_name);

    eframe::run_native(
        settings::APP_NAME,
        win_option,
        Box::new(|cc| Box::new(gui.init(cc))),
    );
}

fn init_logger() {
    if let Some(log_file_path) = settings::get_log_file_full_path() {
        if let Ok(file) = File::create(log_file_path) {
            let subscriber = Registry::default()
                .with(
                    fmt::Layer::default()
                        .with_writer(Mutex::new(file))
                        .with_ansi(false)
                        .with_line_number(true),
                )
                .with(
                    fmt::Layer::default()
                        .with_writer(std::io::stdout)
                        .with_line_number(true),
                );

            let _ = tracing::subscriber::set_global_default(subscriber);
        } else {
            tracing_subscriber::fmt::init();
        }
    } else {
        tracing_subscriber::fmt::init();
    }
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
        args.push(
            "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Risk of Rain 2\\BepInEx\\config\\BepInEx.GUI.cfg"
                .to_string(),
        );
        args.push(
            "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Risk of Rain 2\\BepInEx\\LogOutput.log"
                .to_string(),
        );
        args.push("17584".to_string()); // Target Process Id
        args.push("27090".to_string()); // Socket port used for comm with the bep gui patcher
    } else if args.len() != 9 {
        tracing::error!("PROBLEM WITH ARGS {:?}", args);
        panic!("PROBLEM WITH ARGS {:?}", args);
    }
}
