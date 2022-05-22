use std::{cell::RefCell, rc::Rc};

use clipboard::*;
use eframe::{egui::*, *};

use crate::{
    bepinex_gui_config::BepInExGUIConfig,
    bepinex_log::{self, BepInExLog, LogLevel},
    check_if_dev, colors, egui_utils,
};

use super::Tab;

pub struct ConsoleTab {
    pub mods: Rc<RefCell<Option<Vec<String>>>>,
    pub logs: Rc<RefCell<Option<Vec<BepInExLog>>>>,
    pub log_text_filter: String,
    pub log_level_filter: LogLevel,
    pub selected_index_in_mods_combo_box: usize,
    pub button_currently_down: bool,
    pub first_index_of_log_that_is_selected: u32,
    pub smallest_index_of_hovered_log: u32,
    pub biggest_index_of_hovered_log: u32,
}

impl ConsoleTab {
    pub fn new(
        mods: Rc<RefCell<Option<Vec<String>>>>,
        logs: Rc<RefCell<Option<Vec<BepInExLog>>>>,
    ) -> Self {
        Self {
            mods: mods,
            logs: logs,
            log_text_filter: Default::default(),
            log_level_filter: LogLevel::All,
            selected_index_in_mods_combo_box: 0,
            button_currently_down: false,
            first_index_of_log_that_is_selected: std::u32::MAX,
            smallest_index_of_hovered_log: std::u32::MAX,
            biggest_index_of_hovered_log: std::u32::MAX,
        }
    }

    fn render(&mut self, gui_config: &BepInExGUIConfig, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            if self.logs.borrow_mut().as_mut().unwrap().is_empty() {
                ui.vertical_centered_justified(|ui| {
                    let loading_text = "Loading ⌛";
                    let text_size = egui_utils::compute_text_size(ui, loading_text, true, false);
                    ui.add_space(ui.available_height() / 2. - text_size.y);
                    ui.heading(loading_text);
                });
            } else {
                ui.spacing_mut().scroll_bar_width = 16.;
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        self.render_logs(gui_config, ui);
                    });
            }
        });
    }

    fn render_logs(&mut self, gui_config: &BepInExGUIConfig, ui: &mut eframe::egui::Ui) {
        let clip_rect = ui.painter().clip_rect();
        ui.interact(clip_rect, ui.id(), Sense::drag());
        let is_button_down = ui.ctx().input().pointer.primary_down();
        let is_button_up = !ui.ctx().input().pointer.button_down(PointerButton::Primary);

        if is_button_up {
            self.button_currently_down = false;
        }

        let info_log_color = if gui_config.dark_mode {
            Color32::WHITE
        } else {
            Color32::BLACK
        };

        let mut i = 0;
        for log in self.logs.borrow_mut().as_mut().unwrap().iter() {
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

                if self.button_currently_down {
                    egui_utils::scroll_when_trying_to_select_stuff_above_or_under_rect(
                        ui, clip_rect,
                    );
                }
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

                    let selected_logs: Vec<String> = self.logs.borrow_mut().as_mut().unwrap()
                        [start_index..end_index + 1]
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
    }

    fn render_footer(&mut self, ctx: &Context) {
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
}

impl Tab for ConsoleTab {
    fn name(&self) -> &str {
        "Console"
    }

    fn update_top_panel(&mut self, gui_config: &mut BepInExGUIConfig, ui: &mut eframe::egui::Ui) {
        egui::menu::bar(ui, move |ui| {
            // controls
            ui.with_layout(Layout::left_to_right(), move |ui| {
                let cur_cursor_rect = ui.cursor();

                let mut mods_borrow = self.mods.borrow_mut();
                let mods = mods_borrow.as_mut().unwrap();

                ui.label(RichText::new("Log Filtering: ").font(FontId::proportional(20.0)));
                let mods_combo_box = ComboBox::from_id_source("combo_box_mods_log_filter")
                    .show_index(
                        ui,
                        &mut self.selected_index_in_mods_combo_box,
                        mods.len(),
                        |i| mods[i].to_owned(),
                    );
                if mods_combo_box.changed() {
                    if self.selected_index_in_mods_combo_box == 0 {
                        self.log_text_filter = "".to_string();
                    } else {
                        self.log_text_filter =
                            mods[self.selected_index_in_mods_combo_box].to_string();
                    }
                }

                ui.style_mut().visuals.extreme_bg_color = if gui_config.dark_mode {
                    colors::DARK_GRAY
                } else {
                    colors::LIGHT_GRAY
                };
                ui.add_sized(
                    mods_combo_box.rect.size(),
                    TextEdit::singleline(&mut self.log_text_filter)
                        .text_color(if gui_config.dark_mode {
                            Color32::WHITE
                        } else {
                            Color32::BLACK
                        })
                        .hint_text(WidgetText::from("Filter Text").color(colors::FADED_LIGHT_GRAY)),
                );

                // restore cursor so that we can center label easily
                ui.set_cursor(cur_cursor_rect);

                // let label_size = compute_text_size(ui, settings::APP_NAME, true, false);
                // ui.add_space(ui.available_width() / 2. - label_size.x);
                // ui.heading(settings::APP_NAME);

                let theme_btn_text = if gui_config.dark_mode { "🌞" } else { "🌙" };
                let theme_btn_size = egui_utils::compute_text_size(ui, theme_btn_text, true, false);

                ui.add_space(ui.available_width() - theme_btn_size.x);
                if ui
                    .add(Button::new(
                        RichText::new(theme_btn_text).text_style(egui::TextStyle::Heading),
                    ))
                    .clicked()
                {
                    gui_config.dark_mode ^= true;
                }
            });
        });
    }

    fn update(
        &mut self,
        gui_config: &mut BepInExGUIConfig,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
    ) {
        self.render_footer(ctx);

        self.render(gui_config, ctx);
    }

    fn require_dev_check(&self) -> bool {
        true
    }
}
