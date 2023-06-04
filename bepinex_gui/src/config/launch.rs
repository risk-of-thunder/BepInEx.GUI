use std::path::PathBuf;

use sysinfo::Pid;

use crate::app;

pub struct AppLaunchConfig {
    target_name: String,
    game_folder_full_path: PathBuf,
    bepinex_log_output_file_full_path: PathBuf,
    bepinex_gui_csharp_cfg_full_path: PathBuf,
    target_process_id: Pid,
    // Socket port used for comm with the bep gui patcher
    log_socket_port_receiver: u16,
    window_title: String,
}

impl AppLaunchConfig {
    const ARG_COUNT: usize = 8;

    pub fn from(args: &Vec<String>) -> Option<Self> {
        if args.len() == Self::ARG_COUNT {
            let bepinex_version = &args[1];
            let target_name = &args[2];
            let window_title = app::NAME.to_owned() + " " + bepinex_version + " - " + target_name;

            Some(Self {
                target_name: target_name.into(),
                game_folder_full_path: (&args[3]).into(),
                bepinex_log_output_file_full_path: (&args[4]).into(),
                bepinex_gui_csharp_cfg_full_path: (&args[5]).into(),
                target_process_id: args[6].parse::<Pid>().unwrap(),
                log_socket_port_receiver: args[7].parse::<u16>().unwrap(),
                window_title,
            })
        } else {
            tracing::error!("Problem with args {:?} {:?}", args.len(), args);

            None
        }
    }

    pub fn default() -> Self {
        let bepinex_version_string = "5.4.19";
        let target_name = "Risk of Rain 2";

        Self {
            target_name : target_name.into(),
            game_folder_full_path: "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Risk of Rain 2".into(),
            bepinex_log_output_file_full_path: "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Risk of Rain 2\\BepInEx\\LogOutput.log".into(),
            bepinex_gui_csharp_cfg_full_path: "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Risk of Rain 2\\BepInEx\\config\\BepInEx.GUI.cfg".into(),
            target_process_id: Pid::from(17584),
            log_socket_port_receiver: 27090,
            window_title : app::NAME.to_owned() + " " + bepinex_version_string + " - " + target_name,
        }
    }

    pub fn target_name(&self) -> &str {
        self.target_name.as_ref()
    }

    pub const fn game_folder_full_path(&self) -> &PathBuf {
        &self.game_folder_full_path
    }

    pub const fn bepinex_log_output_file_full_path(&self) -> &PathBuf {
        &self.bepinex_log_output_file_full_path
    }

    pub const fn bepinex_gui_csharp_cfg_full_path(&self) -> &PathBuf {
        &self.bepinex_gui_csharp_cfg_full_path
    }

    pub const fn target_process_id(&self) -> Pid {
        self.target_process_id
    }

    pub const fn log_socket_port_receiver(&self) -> u16 {
        self.log_socket_port_receiver
    }

    pub fn window_title(&self) -> &str {
        self.window_title.as_ref()
    }
}
