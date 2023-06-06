use sysinfo::Pid;

use eframe::CreationContext;
use eframe::{self, *};

use std::sync::{atomic::AtomicBool, Arc};
use std::time::Duration;

use crossbeam_channel::Receiver;

use views::tabs::{console::ConsoleTab, general::GeneralTab, settings::SettingsTab, Tab};

use crate::backend::{process, window};
use crate::config::launch::AppLaunchConfig;
use crate::config::Config;
use crate::data::bepinex_log::receiver::LogReceiver;
use crate::data::bepinex_log::BepInExLogEntry;
use crate::data::bepinex_mod::BepInExMod;
use crate::views::disclaimer::Disclaimer;
use crate::{theme, views};

pub const NAME: &str = "BepInExGUI";

pub struct BepInExGUI {
    pub app_launch_config: AppLaunchConfig,
    pub config: Config,

    pub disclaimer: Disclaimer,

    pub should_exit_app: Arc<AtomicBool>,

    pub tabs: Vec<Box<dyn Tab>>,

    pub log_receiver_thread: Option<LogReceiver>,

    pub is_window_title_set: bool,

    pub dark_theme: egui::Style,
}

const FPS_15: Duration = Duration::from_micros(66666);

impl App for BepInExGUI {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint_after(FPS_15);

        self.backend_update(frame);

        // #[cfg(debug_assertions)]
        // ctx.set_debug_on_hover(true);

        self.view_update(ctx, frame);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, NAME, &self.config);
        _ = self.config.save_bepinex_toml_cfg_file();
    }
}

impl BepInExGUI {
    pub fn new(init_config: AppLaunchConfig) -> Self {
        Self {
            app_launch_config: init_config,
            config: Default::default(),
            disclaimer: Disclaimer {
                first_time_showing_it: true,
                time_when_disclaimer_showed_up: None,
            },
            should_exit_app: Arc::new(AtomicBool::new(false)),
            tabs: vec![],
            log_receiver_thread: None,
            is_window_title_set: false,
            dark_theme: theme::get_dark_theme(),
        }
    }

    pub fn init(mut self, cc: &CreationContext) -> Self {
        if let Some(storage) = cc.storage {
            self.config = eframe::get_value(storage, NAME).unwrap_or_default();
        }

        theme::configure_fonts(&cc.egui_ctx);

        if self.config.dark_mode {
            cc.egui_ctx.set_style(self.dark_theme.clone());
        } else {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
        }

        self.start_thread_exit_gui_if_target_process_not_alive(
            self.app_launch_config.target_process_id(),
        );

        window::window_topmost_on_target_start::init(self.app_launch_config.target_process_id());

        let (general_tab_mod_r, console_tab_mod_r, log_r) =
            self.init_log_receiver(self.app_launch_config.log_socket_port_receiver());

        self.init_tabs(general_tab_mod_r, console_tab_mod_r, log_r);

        self.config.bepinex_gui_csharp_cfg_full_path = self
            .app_launch_config
            .bepinex_gui_csharp_cfg_full_path()
            .clone();

        _ = self.config.read_bepinex_toml_cfg_file();

        self
    }

    fn init_log_receiver(
        &mut self,
        log_socket_port_receiver: u16,
    ) -> (
        Receiver<BepInExMod>,
        Receiver<BepInExMod>,
        Receiver<BepInExLogEntry>,
    ) {
        let (general_tab_mod_s, general_tab_mod_r) = crossbeam_channel::unbounded();
        let (console_tab_mod_s, console_tab_mod_r) = crossbeam_channel::unbounded();
        let (log_s, log_r) = crossbeam_channel::unbounded();

        let log_receiver = LogReceiver::new(
            log_socket_port_receiver,
            vec![log_s],
            vec![general_tab_mod_s, console_tab_mod_s],
        );
        log_receiver.start_thread_loop();
        self.log_receiver_thread = Some(log_receiver);

        (general_tab_mod_r, console_tab_mod_r, log_r)
    }

    fn init_tabs(
        &mut self,
        general_tab_mod_r: Receiver<BepInExMod>,
        console_tab_mod_r: Receiver<BepInExMod>,
        log_r: Receiver<BepInExLogEntry>,
    ) {
        self.tabs.push(Box::new(GeneralTab::new(general_tab_mod_r)));
        self.tabs.push(Box::new(ConsoleTab::new(
            console_tab_mod_r,
            log_r,
            self.should_exit_app.clone(),
        )));
        self.tabs.push(Box::new(SettingsTab::new()));
    }

    fn start_thread_exit_gui_if_target_process_not_alive(&self, target_process_id: Pid) {
        process::spawn_thread_is_process_dead(
            target_process_id,
            self.config.close_window_when_game_closes.clone(),
            self.should_exit_app.clone(),
        )
    }
}
