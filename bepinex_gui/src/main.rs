// Disable console
// #![windows_subsystem = "windows"]

use bepinex_log::{BepInExLog, LogLevel};
use eframe::emath::*;

pub mod packet_protocol;

use clipboard::*;
use eframe::egui::*;
use eframe::*;
use log_receiver::*;
use serde::*;
use std::sync::mpsc::*;

const LIGHT_GRAY: Color32 = Color32::from_rgb(230, 230, 230);
const DARK_GRAY: Color32 = Color32::from_rgb(60, 60, 60);

#[derive(Serialize, Deserialize)]
pub struct BepInExGUIConfig {
    pub dark_mode: bool,
    pub window_pos: Pos2,
    pub first_time: bool,
}

impl Default for BepInExGUIConfig {
    fn default() -> Self {
        Self {
            dark_mode: true,
            window_pos: Default::default(),
            first_time: true,
        }
    }
}

mod bepinex_log;

pub struct BepInExGUI {
    pub config: BepInExGUIConfig,
    pub logs_receiver: Option<Receiver<BepInExLog>>,
    pub log_receiver: Option<LogReceiver>,
    pub logs: Vec<BepInExLog>,
    pub log_text_filter: String,
    pub log_level_filter: LogLevel,
    pub mods: Vec<String>,
    pub selected_index_in_mods_combo_box: usize,
    pub button_currently_down: bool,
    pub first_index_of_log_that_is_selected: u32,
    pub smallest_index_of_hovered_log: u32,
    pub biggest_index_of_hovered_log: u32,
}

mod settings;

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

        self.render_header(ctx, frame);
        self.render_footer(ctx);

        self.update_console(ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, &settings::APP_NAME, &self.config);
    }
}

impl BepInExGUI {
    pub fn new() -> BepInExGUI {
        BepInExGUI {
            config: Default::default(),
            logs_receiver: None,
            log_receiver: None,
            logs: vec![],
            log_text_filter: Default::default(),
            log_level_filter: LogLevel::All,
            button_currently_down: false,
            first_index_of_log_that_is_selected: std::u32::MAX,
            smallest_index_of_hovered_log: std::u32::MAX,
            biggest_index_of_hovered_log: std::u32::MAX,
            mods: vec!["".to_string()],
            selected_index_in_mods_combo_box: 0,
        }
    }

    pub fn init(mut self, cc: &CreationContext) -> Self {
        if let Some(storage) = cc.storage {
            self.config = eframe::get_value(storage, settings::APP_NAME).unwrap_or_default();
        }
        self.configure_fonts(&cc.egui_ctx);

        self.init_log_receiver();

        self
    }

    fn init_log_receiver(&mut self) {
        let (logs_sender, logs_receiver) = channel();
        self.logs_receiver = Some(logs_receiver);

        let log_receiver = LogReceiver::new(logs_sender.clone());
        log_receiver.log_receiver_thread_loop();
        self.log_receiver = Some(log_receiver);
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

    pub(crate) fn render_footer(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                let mut spacing = ui.available_size() / 8.;
                spacing.y += 15.;

                ui.add_space(spacing.x / 2.);

                ui.spacing_mut().item_spacing.x = spacing.x;

                if ui
                    .add_sized(spacing, Button::new("Open Game Folder"))
                    .clicked()
                {
                    eprintln!("a");
                }
                ui.add_sized(spacing, Button::new("Open Log Folder"));
                ui.add_sized(spacing, Button::new("Open BepInEx Folder"));
                ui.add_sized(spacing, Button::new("Modding Discord"));
            });
            ui.add_space(25_f32);
        });
    }

    fn update_console(&mut self, ctx: &Context) {
        self.update_receive_logs_from_channel();

        self.render_console(ctx);
    }

    fn render_console(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            if self.logs.is_empty() {
                ui.vertical_centered_justified(|ui| {
                    let loading_text = "Loading ⌛";
                    let text_size = compute_text_size(ui, loading_text, true, false);
                    ui.add_space(ui.available_height() / 2. - text_size.y);
                    ui.heading(loading_text);
                });
            } else {
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        self.render_logs(ui);
                    });
            }
        });
    }

    pub fn render_logs(&mut self, ui: &mut eframe::egui::Ui) {
        let clip_rect = ui.painter().clip_rect();
        ui.interact(clip_rect, ui.id(), Sense::drag());
        let is_button_down = ui.ctx().input().pointer.primary_down();
        let is_button_up = !ui.ctx().input().pointer.button_down(PointerButton::Primary);

        if is_button_up {
            self.button_currently_down = false;
        }

        let info_log_color = if self.config.dark_mode {
            Color32::WHITE
        } else {
            Color32::BLACK
        };

        let mut i = 0;
        for log in &self.logs {
            if !log
                .data
                .to_lowercase()
                .contains(&self.log_text_filter.to_lowercase())
            {
                continue;
            }

            let log_color = match log.level {
                bepinex_log::LogLevel::None => Color32::RED,
                bepinex_log::LogLevel::Fatal => Color32::RED,
                bepinex_log::LogLevel::Error => Color32::RED,
                bepinex_log::LogLevel::Warning => Color32::YELLOW,
                bepinex_log::LogLevel::Message => info_log_color,
                bepinex_log::LogLevel::Info => info_log_color,
                bepinex_log::LogLevel::Debug => info_log_color,
                bepinex_log::LogLevel::All => info_log_color,
            };

            let selectable_response = ui.add(SelectableLabel::new(
                i >= self.smallest_index_of_hovered_log && i <= self.biggest_index_of_hovered_log,
                RichText::new(log.data.clone()).color(log_color),
            ));

            let mut log_rect = selectable_response.rect;
            log_rect.max.x = clip_rect.max.x;
            if is_button_down {
                if ui.rect_contains_pointer(log_rect) {
                    if !self.button_currently_down {
                        self.button_currently_down = true;
                        self.first_index_of_log_that_is_selected = i;
                        self.smallest_index_of_hovered_log = i;
                        self.biggest_index_of_hovered_log = i;
                    }

                    if i <= self.first_index_of_log_that_is_selected {
                        self.smallest_index_of_hovered_log = i;
                    }

                    if i >= self.first_index_of_log_that_is_selected {
                        self.biggest_index_of_hovered_log = i;
                    }
                }

                self.scroll_when_trying_to_select_stuff_above_or_under_rect(ui, clip_rect);
            }

            i += 1;
        }

        if ui.ctx().input().modifiers.command && ui.ctx().input().key_pressed(Key::C) {
            match ClipboardProvider::new() {
                Ok(ctx_) => {
                    let mut ctx: ClipboardContext = ctx_;
                    let (start_index, end_index) = if self.first_index_of_log_that_is_selected
                        < self.biggest_index_of_hovered_log
                    {
                        (
                            self.first_index_of_log_that_is_selected as usize,
                            self.biggest_index_of_hovered_log as usize,
                        )
                    } else {
                        (
                            self.smallest_index_of_hovered_log as usize,
                            self.first_index_of_log_that_is_selected as usize,
                        )
                    };

                    let selected_logs: Vec<String> = self.logs[start_index..end_index + 1]
                        .iter()
                        .map(|x| x.data.clone())
                        .collect();
                    let selected_logs_string = selected_logs.join("\n");
                    match ctx.set_contents(selected_logs_string) {
                        Ok(_) => {}
                        Err(err) => {
                            eprintln!("Failed copying to clipboard logs {}", err);
                        }
                    }
                }
                Err(_) => {}
            }
        }

        /*for a in &self.articles {
            ui.add_space(PADDING);
            // render title
            let title = format!("▶ {}", a.title);
            if self.config.dark_mode {
                ui.colored_label(WHITE, title);
            } else {
                ui.colored_label(BLACK, title);
            }
            // render desc
            ui.add_space(PADDING);
            let desc =
                Label::new(RichText::new(&a.desc).text_style(eframe::egui::TextStyle::Button));
            ui.add(desc);

            // render hyperlinks
            if self.config.dark_mode {
                ui.style_mut().visuals.hyperlink_color = CYAN;
            } else {
                ui.style_mut().visuals.hyperlink_color = RED;
            }
            ui.add_space(PADDING);
            ui.with_layout(
                Layout::right_to_left().with_cross_align(eframe::emath::Align::Min),
                |ui| {
                    ui.add(Hyperlink::from_label_and_url("read more ⤴", &a.url));
                },
            );
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }*/
    }

    fn scroll_when_trying_to_select_stuff_above_or_under_rect(&self, ui: &mut Ui, clip_rect: Rect) {
        if self.button_currently_down && !ui.rect_contains_pointer(clip_rect) {
            let mut scroll = Vec2::new(0., 0.);
            let dist = clip_rect.bottom() - ui.input().pointer.interact_pos().unwrap().y;

            if dist < 0. {
                scroll.y = dist;
                scroll.y *= 0.005;
            } else if dist > 0. {
                scroll.y = clip_rect.top() - ui.input().pointer.interact_pos().unwrap().y;
                scroll.y *= 0.005;
            }

            ui.scroll_with_delta(scroll);
        }
    }

    pub fn update_receive_logs_from_channel(&mut self) {
        if let Some(receiver) = &self.logs_receiver {
            match receiver.try_recv() {
                Ok(log) => {
                    self.logs.push(log);
                }
                Err(_) => {}
            }
        }
    }

    pub(crate) fn render_header(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut spacing = ui.available_size() / 3.;
                spacing.y += 20.;

                ui.spacing_mut().item_spacing.x = 1.;
                ui.spacing_mut().item_spacing.y = 1.;

                ui.add_sized(spacing, Button::new("General"));
                ui.add_sized(spacing, Button::new("Console"));
                ui.add_sized(spacing, Button::new("Settings"));
            });

            ui.separator();
            ui.add_space(10.);

            egui::menu::bar(ui, |ui| {
                // controls
                ui.with_layout(Layout::left_to_right(), |ui| {
                    let cur_cursor_rect = ui.cursor();

                    ui.label(RichText::new("Log Filtering: ").font(FontId::proportional(20.0)));
                    let mods_combo_box = ComboBox::from_id_source("combo_box_mods_log_filter")
                        .show_index(
                            ui,
                            &mut self.selected_index_in_mods_combo_box,
                            self.mods.len(),
                            |i| self.mods[i].to_owned(),
                        );
                    if mods_combo_box.changed() {
                        if self.selected_index_in_mods_combo_box == 0 {
                            self.log_text_filter = "".to_string();
                            self.mods.push("Test".to_string());
                        } else {
                            self.log_text_filter =
                                self.mods[self.selected_index_in_mods_combo_box].to_string();
                        }
                    }

                    ui.style_mut().visuals.extreme_bg_color = if self.config.dark_mode {
                        DARK_GRAY
                    } else {
                        LIGHT_GRAY
                    };
                    ui.add_sized(
                        mods_combo_box.rect.size(),
                        TextEdit::singleline(&mut self.log_text_filter)
                            .text_color(if self.config.dark_mode {
                                Color32::WHITE
                            } else {
                                Color32::BLACK
                            })
                            .hint_text(WidgetText::from("Filter Text")),
                    );

                    // restore cursor so that we can center label easily
                    ui.set_cursor(cur_cursor_rect);

                    // let label_size = compute_text_size(ui, settings::APP_NAME, true, false);
                    // ui.add_space(ui.available_width() / 2. - label_size.x);
                    // ui.heading(settings::APP_NAME);

                    let theme_btn_text = if self.config.dark_mode {
                        "🌞"
                    } else {
                        "🌙"
                    };
                    let theme_btn_size = compute_text_size(ui, theme_btn_text, true, false);

                    ui.add_space(ui.available_width() - theme_btn_size.x);
                    if ui
                        .add(Button::new(
                            RichText::new(theme_btn_text).text_style(egui::TextStyle::Heading),
                        ))
                        .clicked()
                    {
                        self.config.dark_mode ^= true;
                    }
                });
            });
            ui.add_space(10.);
        });
    }
}

fn compute_text_size(ui: &mut Ui, text: &str, is_heading: bool, is_wrap: bool) -> Vec2 {
    let label = if is_heading {
        Label::new(RichText::new(text).heading()).wrap(is_wrap)
    } else {
        Label::new(RichText::new(text)).wrap(is_wrap)
    };

    let label_layout_in_ui = label.layout_in_ui(ui);
    let text_size = label_layout_in_ui.1.size();

    text_size
}

mod log_receiver;

fn main() {
    tracing_subscriber::fmt::init();

    let gui = BepInExGUI::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(993., 519.));
    eframe::run_native(
        "BepInEx GUI",
        win_option,
        Box::new(|cc| Box::new(gui.init(cc))),
    );
}
