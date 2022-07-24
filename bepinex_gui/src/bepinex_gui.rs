use clipboard::{ClipboardContext, ClipboardProvider};
use eframe::{egui::*, CreationContext};
use sysinfo::{Pid, SystemExt};
use tab::settings_tab::SettingsTab;

use tab::console_tab::ConsoleTab;

use tab::general_tab::GeneralTab;

use eframe::*;

use eframe;

use core::time;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::SystemTime;
use std::{cell::RefCell, sync::mpsc::Receiver};
use std::{io, thread};

use std::rc::Rc;

use tab::Tab;

use crate::bepinex_mod::BepInExMod;
use crate::check_if_dev::QUESTION_ANSWERS_LENGTH;
use crate::log_receiver_thread::LogReceiverThread;
use crate::{bepinex_gui_config::BepInExGUIConfig, bepinex_log::BepInExLog, tab};
use crate::{check_if_dev, colors, egui_utils, settings, thunderstore_communities};

pub struct BepInExGUI {
    config: BepInExGUIConfig,
    target_name: String,
    game_folder_full_path: PathBuf,
    bepinex_root_full_path: PathBuf,
    bepinex_gui_csharp_cfg_full_path: PathBuf,
    target_process_id: Pid,
    show_dev_check_window: bool,
    dev_check_current_answer: Vec<String>,
    time_when_disclaimer_showed_up: Option<SystemTime>,
    should_exit_app: Arc<AtomicBool>,
    tabs: Vec<Box<dyn Tab>>,
    mods: Rc<RefCell<Option<Vec<BepInExMod>>>>,
    logs: Rc<RefCell<Option<Vec<BepInExLog>>>>,
    logs_receiver: Option<Receiver<BepInExLog>>,
    log_receiver_thread: Option<LogReceiverThread>,
    log_socket_port_receiver: u16,
}

impl App for BepInExGUI {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        if self.should_exit_app.load(Ordering::Relaxed) {
            frame.quit();
        }
        ctx.request_repaint();

        #[cfg(debug_assertions)]
        ctx.set_debug_on_hover(true);

        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        self.update_receive_logs_from_channel();

        if self.config.first_time {
            self.show_first_time_disclaimer(ctx);
        } else {
            self.render_header(ctx, frame);

            let tab = &mut self.tabs[self.config.selected_tab_index];

            let is_dev = self.config.is_dev.load(Ordering::Relaxed);
            self.show_dev_check_window = tab.is_dev_only() && !is_dev;

            if self.show_dev_check_window {
                CentralPanel::default().show(ctx, |_| {
                    Window::new("This tab requires a dev check to be used")
                        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                        .show(ctx, |ui| {
                            let questions_answers =
                                &check_if_dev::QUESTIONS_ANSWERS.lock().unwrap();
                            for i in 0..questions_answers.len() {
                                ui.heading(questions_answers[i].0);
                                ui.style_mut().visuals.extreme_bg_color = if self.config.dark_mode {
                                    colors::DARK_GRAY
                                } else {
                                    colors::LIGHT_GRAY
                                };

                                let answer_edit_line_size = Vec2::new(ui.available_width(), 20.);
                                if ui
                                    .add_sized(
                                        answer_edit_line_size,
                                        TextEdit::singleline(&mut self.dev_check_current_answer[i])
                                            .text_color(if self.config.dark_mode {
                                                Color32::WHITE
                                            } else {
                                                Color32::BLACK
                                            })
                                            .hint_text(
                                                WidgetText::from("Type Answer Here")
                                                    .color(colors::FADED_LIGHT_GRAY),
                                            ),
                                    )
                                    .clicked()
                                {
                                    self.dev_check_check_answers(questions_answers);
                                }
                            }

                            if ui
                                .button(RichText::new("Submit").font(FontId::proportional(20.0)))
                                .clicked()
                            {
                                self.dev_check_check_answers(questions_answers);
                            }
                        });
                });
            } else {
                tab.update(&mut self.config, ctx, frame);
            }
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, settings::APP_NAME, &self.config);
        _ = self.config.save_csharp_cfg_file();
    }
}

impl BepInExGUI {
    pub fn new(
        target_name: String,
        game_folder_full_path: PathBuf,
        bepinex_root_full_path: PathBuf,
        bepinex_gui_csharp_cfg_full_path: PathBuf,
        target_process_id: Pid,
        log_socket_port_receiver: u16,
    ) -> Self {
        Self {
            config: Default::default(),
            target_name,
            game_folder_full_path,
            bepinex_root_full_path,
            bepinex_gui_csharp_cfg_full_path,
            target_process_id: target_process_id,
            show_dev_check_window: false,
            dev_check_current_answer: vec!["".to_string(); *QUESTION_ANSWERS_LENGTH],
            time_when_disclaimer_showed_up: None,
            should_exit_app: Arc::new(AtomicBool::new(false)),
            tabs: vec![],
            mods: Rc::new(RefCell::new(Some(vec![BepInExMod {
                name: "".to_string(),
                version: "".to_string(),
            }]))),
            logs: Rc::new(RefCell::new(Some(vec![]))),
            logs_receiver: None,
            log_receiver_thread: None,
            log_socket_port_receiver: log_socket_port_receiver,
        }
    }

    pub fn init(mut self, cc: &CreationContext) -> Self {
        if let Some(storage) = cc.storage {
            self.config = eframe::get_value(storage, settings::APP_NAME).unwrap_or_default();
        }
        self.configure_fonts(&cc.egui_ctx);

        self.start_thread_exit_gui_if_target_process_not_alive(self.target_process_id);

        self.init_log_receiver(self.log_socket_port_receiver);

        self.init_tabs();

        self.config.bepinex_gui_csharp_cfg_full_path =
            self.bepinex_gui_csharp_cfg_full_path.clone();

        _ = self.config.read_csharp_cfg_file();

        self
    }

    fn init_log_receiver(&mut self, log_socket_port_receiver: u16) {
        let (logs_sender, logs_receiver) = channel();
        self.logs_receiver = Some(logs_receiver);

        let log_receiver = LogReceiverThread::new(
            log_socket_port_receiver,
            logs_sender.clone(),
            self.config.is_dev.clone(),
        );
        log_receiver.start_thread_loop();
        self.log_receiver_thread = Some(log_receiver);
    }

    fn init_tabs(&mut self) {
        self.tabs.push(Box::new(GeneralTab::new(
            self.mods.clone(),
            self.logs.clone(),
            self.target_process_id,
            self.target_name.clone(),
            self.game_folder_full_path.clone(),
            self.bepinex_root_full_path.clone(),
        )));
        self.tabs.push(Box::new(ConsoleTab::new(
            self.mods.clone(),
            self.logs.clone(),
            self.target_process_id,
            self.game_folder_full_path.clone(),
            self.bepinex_root_full_path.clone(),
        )));
        self.tabs.push(Box::new(SettingsTab::new()));
    }

    fn update_receive_logs_from_channel(&mut self) {
        let mut logs_borrow = self.logs.borrow_mut();
        let logs = logs_borrow.as_mut().unwrap();

        let mut mods_borrow = self.mods.borrow_mut();
        let mods = mods_borrow.as_mut().unwrap();

        if let Some(receiver) = &self.logs_receiver {
            match receiver.try_recv() {
                Ok(log) => {
                    if log.data.contains("Loading [") {
                        let split: Vec<&str> = log.data.split('[').collect();
                        let mod_info_text = split[2];
                        let mod_version_start_index_ = mod_info_text.rfind(' ');
                        if let Some(mod_version_start_index) = mod_version_start_index_ {
                            let mod_name = &mod_info_text[0..mod_version_start_index];
                            let mod_version = &mod_info_text
                                [mod_version_start_index + 1..mod_info_text.len() - 1];
                            mods.push(BepInExMod {
                                name: mod_name.to_string(),
                                version: mod_version.to_string(),
                            });
                        }
                    }

                    logs.push(log);
                }
                Err(_) => {}
            }
        }
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

            static mut FIRST_TIME_SHOW:bool = true;
            unsafe {
                if FIRST_TIME_SHOW {
                    self.time_when_disclaimer_showed_up = Some(SystemTime::now());
                    FIRST_TIME_SHOW = false;
                }
            }

            if let Ok(_elapsed) = self.time_when_disclaimer_showed_up.unwrap().elapsed() {
                let elapsed = _elapsed.as_secs() as i64;
                if 9 - elapsed >= 0 {
                    ui.label(RichText::new((10 - elapsed).to_string()).font(FontId::proportional(20.0)));
                }
                else {
                    if ui.button(RichText::new("Ok").font(FontId::proportional(20.0))).clicked() {
                        self.config.first_time = false;
                    }
                }
            }
        });
    });
    }

    fn configure_fonts(&self, ctx: &Context) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MesloLGS".to_string(),
            FontData::from_static(include_bytes!("../../assets/fonts/MesloLGS_NF_Regular.ttf")),
        );

        font_def
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_string());

        ctx.set_fonts(font_def);
    }

    fn start_thread_exit_gui_if_target_process_not_alive(&self, target_process_id: Pid) {
        let mut sys = sysinfo::System::new_all();

        let close_window_when_game_closes = self.config.close_window_when_game_closes.clone();
        let should_exit_app = self.should_exit_app.clone();
        thread::spawn(move || -> io::Result<()> {
            loop {
                if !sys.refresh_process(target_process_id)
                    && close_window_when_game_closes.load(Ordering::Relaxed)
                {
                    break;
                }

                thread::sleep(time::Duration::from_millis(2000));
            }

            tracing::info!("Target process is not alive, scheduling exit");
            should_exit_app.store(true, Ordering::Relaxed);

            Ok(())
        });
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

            if !self.show_dev_check_window {
                self.tabs[self.config.selected_tab_index].update_top_panel(&mut self.config, ui);
                ui.add_space(10.);
            }
        });
    }

    fn dev_check_check_answers(
        &mut self,
        questions_answers: &std::sync::MutexGuard<Vec<(&str, &str)>>,
    ) {
        let mut good_answer_count = 0;
        for i in 0..questions_answers.len() {
            if self.dev_check_current_answer[i].to_lowercase()
                == questions_answers[i].1.to_lowercase()
            {
                good_answer_count += 1;
                if good_answer_count == *QUESTION_ANSWERS_LENGTH {
                    self.config.is_dev.store(true, Ordering::Relaxed);
                }
            }
        }
    }
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
    bepinex_root_full_path: &PathBuf,
    logs: &Rc<RefCell<Option<Vec<BepInExLog>>>>,
    target_process_id: Pid,
) {
    ui.horizontal_centered(|ui| {
        const FONT_SIZE: f32 = 14.;
        // let mut FONT_SIZE = 20. * (ui.available_width() / 900.);

        let mut button_size = ui.available_size() / 16.;
        let spacing = ui.available_width() / 16.;
        button_size.y += 25.;

        let placement_cursor = ui.cursor();
        ui.add_space(spacing * 0.9);

        if ui
            .add_sized(
                button_size,
                Button::new(
                    RichText::new("Open Game Folder").font(FontId::proportional(FONT_SIZE)),
                ),
            )
            .clicked()
        {
            egui_utils::open_folder(game_folder_full_path);
        }

        ui.set_cursor(placement_cursor);
        ui.add_space(spacing * 5.);

        if ui
            .add_sized(
                button_size,
                Button::new(RichText::new("Open Log Folder").font(FontId::proportional(FONT_SIZE))),
            )
            .clicked()
        {
            egui_utils::open_folder(bepinex_root_full_path);
        }

        ui.set_cursor(placement_cursor);
        ui.add_space(spacing * 8.5);

        if ui
            .add_sized(
                button_size,
                Button::new(
                    RichText::new("Copy Log to Clipboard").font(FontId::proportional(FONT_SIZE)),
                ),
            )
            .clicked()
        {
            match ClipboardProvider::new() {
                Ok(ctx) => {
                    let mut clipboard: ClipboardContext = ctx;

                    let logs_borrow = logs.borrow();
                    let logs = logs_borrow.as_ref().unwrap();
                    let selected_logs_string: String = logs
                        .into_iter()
                        .map(|x| x.data.to_string())
                        .collect::<Vec<String>>()
                        .join("\n");
                    match clipboard.set_contents(selected_logs_string) {
                        Ok(_) => {}
                        Err(err) => {
                            tracing::error!("Failed copying logs to clipboard: {}", err);
                        }
                    }
                }
                Err(_) => {}
            }
        }

        ui.set_cursor(placement_cursor);
        ui.add_space(spacing * 13.);

        if ui
            .add_sized(
                button_size,
                Button::new(RichText::new("Modding Discord").font(FontId::proportional(FONT_SIZE))),
            )
            .clicked()
        {
            thunderstore_communities::open_modding_discord(target_process_id);
        }
    });
    ui.add_space(25.);
}
