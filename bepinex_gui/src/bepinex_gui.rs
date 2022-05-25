use eframe::{egui::*, CreationContext};
use sysinfo::{Pid, Process, System, SystemExt};
use tab::settings_tab::SettingsTab;

use tab::console_tab::ConsoleTab;

use tab::general_tab::GeneralTab;

use eframe::*;

use eframe;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::SystemTime;
use std::{cell::RefCell, sync::mpsc::Receiver};

use std::rc::Rc;

use tab::Tab;

use crate::check_if_dev::QUESTION_ANSWERS_LENGTH;
use crate::log_receiver_thread::LogReceiverThread;
use crate::{bepinex_gui_config::BepInExGUIConfig, bepinex_log::BepInExLog, tab};
use crate::{check_if_dev, colors, settings};

pub struct BepInExGUI {
    pub(crate) config: BepInExGUIConfig,
    pub(crate) game_folder_full_path: String,
    pub(crate) bepinex_root_full_path: String,
    pub(crate) target_process_id: Pid,
    pub(crate) show_dev_check_window: bool,
    pub(crate) dev_check_current_answer: Vec<String>,
    pub(crate) time_when_disclaimer_showed_up: Option<SystemTime>,
    pub(crate) tabs: Vec<Box<dyn Tab>>,
    pub(crate) mods: Rc<RefCell<Option<Vec<String>>>>,
    pub(crate) logs: Rc<RefCell<Option<Vec<BepInExLog>>>>,
    pub(crate) logs_receiver: Option<Receiver<BepInExLog>>,
    pub(crate) log_receiver_thread: Option<LogReceiverThread>,
    pub(crate) log_socket_port_receiver: u16,
}

impl App for BepInExGUI {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();

        #[cfg(debug_assertions)]
        ctx.set_debug_on_hover(true);

        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        if let Some(storage) = frame.storage_mut() {}

        self.update_receive_logs_from_channel();

        if self.config.first_time {
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
        } else {
            self.render_header(ctx, frame);

            let tab = &mut self.tabs[self.config.selected_tab_index];

            if tab.require_dev_check() && !self.config.is_dev {
                self.show_dev_check_window = true;
            } else {
                self.show_dev_check_window = false;
            }

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
    }
}

impl BepInExGUI {
    pub fn new(
        game_folder_full_path: &String,
        bepinex_root_full_path: &String,
        target_process_id: Pid,
        log_socket_port_receiver: u16,
    ) -> Self {
        Self {
            config: Default::default(),
            game_folder_full_path: game_folder_full_path.to_string(),
            bepinex_root_full_path: bepinex_root_full_path.to_string(),
            target_process_id: target_process_id,
            show_dev_check_window: false,
            dev_check_current_answer: vec!["".to_string(); *QUESTION_ANSWERS_LENGTH],
            time_when_disclaimer_showed_up: None,
            tabs: vec![],
            mods: Rc::new(RefCell::new(Some(vec!["".to_string()]))),
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

        self.init_log_receiver(self.log_socket_port_receiver);

        self.init_tabs();

        self
    }

    fn init_log_receiver(&mut self, log_socket_port_receiver: u16) {
        let (logs_sender, logs_receiver) = channel();
        self.logs_receiver = Some(logs_receiver);

        let log_receiver = LogReceiverThread::new(log_socket_port_receiver, logs_sender.clone());
        log_receiver.start_thread_loop();
        self.log_receiver_thread = Some(log_receiver);
    }

    fn init_tabs(&mut self) {
        self.tabs.push(Box::new(GeneralTab::new()));
        self.tabs.push(Box::new(ConsoleTab::new(
            self.mods.clone(),
            self.logs.clone(),
            self.target_process_id,
            PathBuf::from(self.game_folder_full_path.to_string()),
            PathBuf::from(self.bepinex_root_full_path.to_string()),
        )));
        self.tabs.push(Box::new(SettingsTab::new()));
    }

    fn update_receive_logs_from_channel(&mut self) {
        let mut logs_borrow = self.logs.borrow_mut();
        let logs = logs_borrow.as_mut().unwrap();

        if let Some(receiver) = &self.logs_receiver {
            match receiver.try_recv() {
                Ok(log) => {
                    logs.push(log);
                }
                Err(_) => {}
            }
        }
    }

    pub(crate) fn configure_fonts(&self, ctx: &Context) {
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

    pub(crate) fn render_header(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut spacing = ui.available_size() / 3.;
                spacing.y += 20.;

                ui.spacing_mut().item_spacing.x = 1.;
                ui.spacing_mut().item_spacing.y = 1.;

                let mut i = 0;
                for tab in &self.tabs {
                    if ui
                        .add_sized(
                            spacing,
                            Button::new(RichText::new(tab.name()).font(FontId::proportional(20.0))),
                        )
                        .clicked()
                    {
                        self.config.selected_tab_index = i;
                    }

                    i += 1;
                }
            });

            ui.separator();
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
                    self.config.is_dev = true;
                }
            }
        }
    }
}
