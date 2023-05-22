use sysinfo::Pid;

use eframe::{self, *};
use eframe::{egui::*, CreationContext};

use std::process::{exit, Command};
use std::{env, fs};
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::SystemTime,
};

use crossbeam_channel::Receiver;

use tab::{console_tab::ConsoleTab, general_tab::GeneralTab, settings_tab::SettingsTab, Tab};

use crate::{
    bepinex_gui_config::BepInExGUIConfig,
    bepinex_gui_init_config::BepInExGUIInitConfig,
    bepinex_log::{self, receiver::LogReceiver, BepInExLogEntry},
    bepinex_mod::BepInExMod,
    egui_utils, file_explorer_utils, process, settings, tab, thunderstore, window,
};

struct Disclaimer {
    pub first_time_showing_console_disclaimer: bool,
    pub time_when_disclaimer_showed_up: Option<SystemTime>,
}

pub struct BepInExGUI {
    init_config: BepInExGUIInitConfig,
    config: BepInExGUIConfig,

    disclaimer: Disclaimer,

    should_exit_app: Arc<AtomicBool>,

    tabs: Vec<Box<dyn Tab>>,

    log_receiver_thread: Option<LogReceiver>,

    is_window_title_set: bool,
}

impl App for BepInExGUI {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        // Ideally this would be done in a init function, not constantly checked in an update function
        // L from eframe
        if !self.is_window_title_set {
            frame.set_window_title(&self.init_config.window_title());
            self.is_window_title_set = true;
        }

        if self.should_exit_app.load(Ordering::Relaxed) {
            frame.close();
        }

        ctx.request_repaint();

        #[cfg(debug_assertions)]
        ctx.set_debug_on_hover(true);

        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        if self.config.first_time {
            self.show_first_time_disclaimer(ctx);
        } else {
            self.render_header(ctx, frame);

            let tab = &mut self.tabs[self.config.selected_tab_index];

            tab.update(&self.init_config, &mut self.config, ctx, frame);
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, settings::APP_NAME, &self.config);
        _ = self.config.save_bepinex_toml_cfg_file();
    }
}

impl BepInExGUI {
    pub fn new(init_config: BepInExGUIInitConfig) -> Self {
        Self {
            init_config,
            config: Default::default(),
            disclaimer: Disclaimer {
                first_time_showing_console_disclaimer: true,
                time_when_disclaimer_showed_up: None,
            },
            should_exit_app: Arc::new(AtomicBool::new(false)),
            tabs: vec![],
            log_receiver_thread: None,
            is_window_title_set: false,
        }
    }

    pub fn init(mut self, cc: &CreationContext) -> Self {
        if let Some(storage) = cc.storage {
            self.config = eframe::get_value(storage, settings::APP_NAME).unwrap_or_default();
        }

        self.configure_fonts(&cc.egui_ctx);

        self.start_thread_exit_gui_if_target_process_not_alive(
            self.init_config.target_process_id(),
        );

        spawn_thread_reset_bepgui_if_window_hang();

        window::window_topmost_on_target_start::init(self.init_config.target_process_id());

        let (mod_r, log_r) = self.init_log_receiver(self.init_config.log_socket_port_receiver());

        self.init_tabs(mod_r, log_r);

        self.config.bepinex_gui_csharp_cfg_full_path =
            self.init_config.bepinex_gui_csharp_cfg_full_path().clone();

        _ = self.config.read_bepinex_toml_cfg_file();

        self
    }

    fn init_log_receiver(
        &mut self,
        log_socket_port_receiver: u16,
    ) -> (Receiver<BepInExMod>, Receiver<BepInExLogEntry>) {
        let (mod_s, mod_r) = crossbeam_channel::unbounded();
        let (log_s, log_r) = crossbeam_channel::unbounded();

        let log_receiver = LogReceiver::new(log_socket_port_receiver, log_s, mod_s);
        log_receiver.start_thread_loop();
        self.log_receiver_thread = Some(log_receiver);

        (mod_r, log_r)
    }

    fn init_tabs(&mut self, mod_r: Receiver<BepInExMod>, log_r: Receiver<BepInExLogEntry>) {
        self.tabs.push(Box::new(GeneralTab::new(mod_r.clone())));
        self.tabs.push(Box::new(ConsoleTab::new(
            mod_r.clone(),
            log_r,
            self.should_exit_app.clone(),
        )));
        self.tabs.push(Box::new(SettingsTab::new()));
    }

    fn show_first_time_disclaimer(&mut self, ctx: &Context) {
        Window::new("Disclaimer").min_width(ctx.available_rect().size().x).anchor(Align2::CENTER_CENTER, Vec2::ZERO).show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.add(
            Label::new(RichText::new(
    r#"The console is now disabled by default.
If you notice issues with a mod while playing:
- Head to the Modding Discord by clicking on the button below.
- Post the log file by copying it to clipboard through the button below.
- Wait for help.
For mod developers that like the old conhost console, you can enable it back by opening the BepInEx/config/BepInEx.cfg and 
setting to true the "Enables showing a console for log output." config option."#).font(FontId::proportional(20.0))).wrap(true));

            if self.disclaimer.first_time_showing_console_disclaimer {
                self.disclaimer.time_when_disclaimer_showed_up = Some(SystemTime::now());
                self.disclaimer.first_time_showing_console_disclaimer = false;
            }

            if let Ok(_elapsed) = self.disclaimer.time_when_disclaimer_showed_up.unwrap().elapsed() {
                let elapsed = _elapsed.as_secs() as i64;
                const NEEDED_TIME_BEFORE_CLOSABLE:i64 = 9;
                let can_close = elapsed > NEEDED_TIME_BEFORE_CLOSABLE;
                if can_close {
                    if ui.button(RichText::new("Ok").font(FontId::proportional(20.0))).clicked() {
                        self.config.first_time = false;
                    }
                }
                else {
                    ui.label(RichText::new(((NEEDED_TIME_BEFORE_CLOSABLE + 1) - elapsed).to_string()).font(FontId::proportional(20.0)));
                }
            }
        });
    });
    }

    fn configure_fonts(&self, ctx: &Context) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MesloLGS".to_string(),
            FontData::from_static(include_bytes!("../assets/fonts/MesloLGS_NF_Regular.ttf")),
        );

        font_def
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_string());

        ctx.set_fonts(font_def);
    }

    fn start_thread_exit_gui_if_target_process_not_alive(&self, target_process_id: Pid) {
        process::spawn_thread_is_process_dead(
            target_process_id,
            self.config.close_window_when_game_closes.clone(),
            self.should_exit_app.clone(),
        )
    }

    fn render_header(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut button_size = ui.available_size() / 3.;
                button_size.y += 25.;

                ui.spacing_mut().item_spacing.x = 1.;
                ui.spacing_mut().item_spacing.y = 1.;

                let mut i = 0;
                for tab in &self.tabs {
                    if ui
                        .add_sized(
                            button_size,
                            Button::new(RichText::new(tab.name()).font(FontId::proportional(20.0))),
                        )
                        .clicked()
                    {
                        self.config.selected_tab_index = i;
                    }

                    i += 1;
                }
            });

            ui.add_space(10.);

            if !self.config.first_time_console_disclaimer {
                self.tabs[self.config.selected_tab_index].update_top_panel(
                    &self.init_config,
                    &mut self.config,
                    ui,
                );
                ui.add_space(10.);
            }
        });
    }
}

// Bad serialized app settings can sometimes make
// the gui window not respond
// bandaid fix that call winapi for checking if the window hung
// and reset the process with a cleaned settings file if so
fn spawn_thread_reset_bepgui_if_window_hang() {
    process::spawn_thread_check_if_process_is_hung(|| {
        if let Some(app_ron_file_path) = BepInExGUIConfig::get_app_ron_file_full_path() {
            let current_exe =
                env::current_exe().expect("Failed to retrieve current executable path");

            let args: Vec<String> = env::args().collect();

            let mut command = Command::new(current_exe);
            command.args(args[1..].iter());

            match fs::remove_file(app_ron_file_path) {
                Ok(_) => {}
                Err(err) => {
                    tracing::error!("{}", err);
                }
            }

            match command.spawn() {
                Ok(_) => {
                    exit(0);
                }
                Err(err) => {
                    tracing::error!("{}", err);
                }
            }
        }
    });
}

pub fn render_theme_button(gui_config: &mut BepInExGUIConfig, ui: &mut egui::Ui) {
    let theme_btn_text = if gui_config.dark_mode { "🌞" } else { "🌙" };

    let theme_btn_size = egui_utils::compute_text_size(ui, theme_btn_text, true, false, None);
    ui.add_space(ui.available_width() - theme_btn_size.x);

    let theme_btn_resp = ui.add(Button::new(
        RichText::new(theme_btn_text).text_style(egui::TextStyle::Heading),
    ));

    if theme_btn_resp.clicked() {
        gui_config.dark_mode ^= true;
    }
}

pub fn render_useful_buttons_footer(
    ui: &mut Ui,
    _ctx: &Context,
    game_folder_full_path: &PathBuf,
    bepinex_log_output_file_full_path: &PathBuf,
    target_process_id: Pid,
) {
    ui.horizontal_centered(|ui| {
        const FONT_SIZE: f32 = 18.;
        // let mut FONT_SIZE = 20. * (ui.available_width() / 900.);

        let mut button_size = ui.available_size() / 5.;
        let spacing = ui.available_width() / 8.;
        button_size.y += 25.;

        let placement_cursor = ui.cursor();
        ui.add_space(spacing * 0.5);

        render_open_game_folder_button(ui, button_size, game_folder_full_path, FONT_SIZE);

        ui.set_cursor(placement_cursor);
        ui.add_space(spacing * 3.);

        render_copy_log_file_button(
            ui,
            button_size,
            bepinex_log_output_file_full_path,
            FONT_SIZE,
        );

        ui.set_cursor(placement_cursor);
        ui.add_space(spacing * 5.5);

        render_open_modding_discord_button(ui, button_size, target_process_id, FONT_SIZE);
    });
    ui.add_space(25.);
}

fn render_open_game_folder_button(
    ui: &mut Ui,
    button_size: Vec2,
    game_folder_full_path: &PathBuf,
    font_size: f32,
) {
    if egui_utils::button("Open Game Folder", ui, button_size, font_size) {
        file_explorer_utils::open_path_in_explorer(game_folder_full_path);
    }
}

fn render_copy_log_file_button(
    ui: &mut Ui,
    button_size: Vec2,
    bepinex_log_output_file_full_path: &PathBuf,
    font_size: f32,
) {
    if egui_utils::button("Copy Log File", ui, button_size, font_size) {
        bepinex_log::file::open_file_explorer_to_log_file_and_zip_if_needed(
            bepinex_log_output_file_full_path,
        );
    }
}

fn render_open_modding_discord_button(
    ui: &mut Ui,
    button_size: Vec2,
    target_process_id: Pid,
    font_size: f32,
) {
    if egui_utils::button("Modding Discord", ui, button_size, font_size) {
        thunderstore::api::open_modding_discord(target_process_id);
    }
}
