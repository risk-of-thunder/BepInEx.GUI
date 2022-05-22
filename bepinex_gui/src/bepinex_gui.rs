use eframe::{egui::*, CreationContext};
use tab::settings_tab::SettingsTab;

use tab::console_tab::ConsoleTab;

use tab::general_tab::GeneralTab;

use eframe::*;

use eframe;

use std::sync::mpsc::channel;
use std::{cell::RefCell, sync::mpsc::Receiver};

use std::rc::Rc;

use tab::Tab;

use crate::log_receiver_thread::LogReceiverThread;
use crate::settings;
use crate::{bepinex_gui_config::BepInExGUIConfig, bepinex_log::BepInExLog, check_if_dev, tab};

pub struct BepInExGUI {
    pub(crate) config: BepInExGUIConfig,
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

        self.render_header(ctx, frame);

        self.tabs[self.config.selected_tab_index].update(&mut self.config, ctx, frame);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, &settings::APP_NAME, &self.config);
    }
}

impl BepInExGUI {
    pub fn new() -> Self {
        Self {
            config: Default::default(),
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

        let x = check_if_dev::give_random_dev_question_answer();

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

            self.tabs[self.config.selected_tab_index].update_top_panel(&mut self.config, ui);

            ui.add_space(10.);
        });
    }
}
