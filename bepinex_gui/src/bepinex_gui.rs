use eframe::{egui::*, CreationContext};
use tab::settings_tab::SettingsTab;

use tab::console_tab::ConsoleTab;

use tab::general_tab::GeneralTab;

use eframe::*;

use eframe;

use std::sync::mpsc::channel;
use std::time::{Duration, SystemTime};
use std::{cell::RefCell, sync::mpsc::Receiver};

use std::rc::Rc;

use tab::Tab;

use crate::check_if_dev::QUESTION_ANSWERS_LENGTH;
use crate::log_receiver_thread::LogReceiverThread;
use crate::{bepinex_gui_config::BepInExGUIConfig, bepinex_log::BepInExLog, tab};
use crate::{check_if_dev, colors, settings};

pub struct BepInExGUI {
    pub(crate) config: BepInExGUIConfig,
    pub(crate) show_dev_check_window: bool,
    pub(crate) dev_check_current_answer: Vec<String>,
    pub(crate) dev_check_good_answer_count: usize,
    pub(crate) time_when_disclaimer_showed_up: SystemTime,
    pub(crate) tabs: Vec<Box<dyn Tab>>,
    pub(crate) mods: Rc<RefCell<Option<Vec<String>>>>,
    pub(crate) logs: Rc<RefCell<Option<Vec<BepInExLog>>>>,
    pub(crate) logs_receiver: Option<Receiver<BepInExLog>>,
    pub(crate) log_receiver_thread: Option<LogReceiverThread>,
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

        self.update_receive_logs_from_channel();

        if self.config.first_time {
            Window::new("One Time Only Disclaimer").min_width(ctx.available_rect().size().x).show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new(
r#"The console is now disabled by default.
If you notice issues with a mod while playing, 
head to the Modding Discord by clicking on the button below, 
post the log file by copying it to clipboard through the button below, and wait for help.
For mod developers that like the old conhost console, you can enable it back by opening the BepInEx/config/BepInEx.cfg and 
setting to true the "Enables showing a console for log output." config option."#));

                    static mut FIRST_TIME_SHOW:bool = true;
                    unsafe {
                        if FIRST_TIME_SHOW {
                            self.time_when_disclaimer_showed_up = SystemTime::now();
                            FIRST_TIME_SHOW = false;
                        }
                    }

                    if let Ok(_elapsed) = self.time_when_disclaimer_showed_up.elapsed() {
                        let elapsed = _elapsed.as_secs() as i64;
                        if 10 - elapsed >= 0 {
                            ui.label((10 - elapsed).to_string());
                        }
                        else {
                            if ui.button("Ok").clicked() {
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
                    Window::new("This tab requires a dev check to be used").show(ctx, |ui| {
                        let questions_answers = &check_if_dev::QUESTIONS_ANSWERS.lock().unwrap();
                        for i in 0..questions_answers.len() {
                            let question_resp = ui.heading(questions_answers[i].0);
                            ui.style_mut().visuals.extreme_bg_color = if self.config.dark_mode {
                                colors::DARK_GRAY
                            } else {
                                colors::LIGHT_GRAY
                            };
                            ui.add_sized(
                                question_resp.rect.size(),
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
                            );
                        }

                        if ui.button("Submit").clicked() {
                            for i in 0..questions_answers.len() {
                                if self.dev_check_current_answer[i].to_lowercase()
                                    == questions_answers[i].1.to_lowercase()
                                {
                                    self.dev_check_good_answer_count += 1;
                                    if self.dev_check_good_answer_count == *QUESTION_ANSWERS_LENGTH
                                    {
                                        self.config.is_dev = true;
                                    }
                                }
                            }
                        }
                    });
                });
            } else {
                tab.update(&mut self.config, ctx, frame);
            }
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, &settings::APP_NAME, &self.config);
    }
}

impl BepInExGUI {
    pub fn new() -> Self {
        Self {
            config: Default::default(),
            show_dev_check_window: false,
            dev_check_current_answer: vec!["".to_string(); *QUESTION_ANSWERS_LENGTH],
            dev_check_good_answer_count: 0,
            time_when_disclaimer_showed_up: SystemTime::now(),
            tabs: vec![],
            mods: Default::default(),
            logs: Default::default(),
            logs_receiver: None,
            log_receiver_thread: None,
        }
    }

    pub fn init(mut self, cc: &CreationContext) -> Self {
        if let Some(storage) = cc.storage {
            self.config = eframe::get_value(storage, settings::APP_NAME).unwrap_or_default();
        }
        self.configure_fonts(&cc.egui_ctx);

        self.init_log_receiver();

        self.mods = Rc::from(RefCell::from(Some(vec!["".to_string()])));
        self.logs = Rc::from(RefCell::from(Some(vec![])));

        self.init_tabs();

        self
    }

    pub(crate) fn init_log_receiver(&mut self) {
        let (logs_sender, logs_receiver) = channel();
        self.logs_receiver = Some(logs_receiver);

        let log_receiver = LogReceiverThread::new(logs_sender.clone());
        log_receiver.start_thread_loop();
        self.log_receiver_thread = Some(log_receiver);
    }

    pub(crate) fn init_tabs(&mut self) {
        self.tabs.push(Box::new(GeneralTab::new()));
        self.tabs.push(Box::new(ConsoleTab::new(
            self.mods.clone(),
            self.logs.clone(),
        )));
        self.tabs.push(Box::new(SettingsTab::new()));
    }

    pub(crate) fn update_receive_logs_from_channel(&mut self) {
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
                    if ui.add_sized(spacing, Button::new(tab.name())).clicked() {
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
}
