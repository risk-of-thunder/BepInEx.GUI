use clipboard::*;
use crossbeam_channel::Receiver;
use eframe::{egui::*, *};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::SystemTime,
};

use crate::{
    backend::process,
    config::{launch::AppLaunchConfig, Config},
    data::{
        bepinex_log::{BepInExLogEntry, LogLevel},
        bepinex_mod::BepInExMod,
    },
    views::{self, disclaimer::Disclaimer},
};

use super::Tab;

struct LogSelection {
    pub button_just_got_down: bool,
    pub first_index_of_log_that_is_selected: u32,
    pub selected_index_in_mods_combo_box: usize,
    pub smallest_index_of_hovered_log: u32,
    pub biggest_index_of_hovered_log: u32,
}

impl LogSelection {
    fn is_log_selection_button_down(&mut self, ui: &mut Ui) -> bool {
        let is_log_selection_button_down = ui.ctx().input(|i| i.pointer.primary_down());
        let is_log_selection_button_up = !ui
            .ctx()
            .input(|i| i.pointer.button_down(PointerButton::Primary));

        if is_log_selection_button_up {
            self.button_just_got_down = false;
        }

        is_log_selection_button_down
    }
}

struct Filter {
    text: String,
}

struct Scroll {
    last_log_count: usize,
}

pub struct ConsoleTab {
    disclaimer: Disclaimer,
    log_selection: LogSelection,
    filter: Filter,
    scroll: Scroll,
    target_process_paused: bool,
    mod_receiver: Receiver<BepInExMod>,
    mods: Vec<BepInExMod>,
    log_receiver: Receiver<BepInExLogEntry>,
    logs: Vec<BepInExLogEntry>,
    should_exit_app: Arc<AtomicBool>,
}

impl ConsoleTab {
    pub fn new(
        mod_receiver: Receiver<BepInExMod>,
        log_receiver: Receiver<BepInExLogEntry>,
        should_exit_app: Arc<AtomicBool>,
    ) -> Self {
        Self {
            disclaimer: Disclaimer {
                first_time_showing_it: true,
                time_when_disclaimer_showed_up: None,
            },
            log_selection: LogSelection {
                button_just_got_down: false,
                first_index_of_log_that_is_selected: std::u32::MAX,
                selected_index_in_mods_combo_box: 0,
                smallest_index_of_hovered_log: std::u32::MAX,
                biggest_index_of_hovered_log: std::u32::MAX,
            },
            filter: Filter {
                text: Default::default(),
            },
            scroll: Scroll { last_log_count: 0 },
            target_process_paused: false,
            mod_receiver,
            mods: vec![BepInExMod::new("", "")],
            log_receiver,
            logs: vec![],
            should_exit_app,
        }
    }

    fn render(&mut self, data: &AppLaunchConfig, gui_config: &Config, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            if self.logs.is_empty() {
                ui.vertical_centered_justified(|ui| {
                    let loading_text = "Loading ⌛";
                    let text_size =
                        views::utils::egui::compute_text_size(ui, loading_text, true, false, None);
                    ui.add_space(ui.available_height() / 2. - text_size.y);
                    ui.heading(loading_text);
                });
            } else {
                ui.spacing_mut().scroll_bar_width = 16.;
                ScrollArea::vertical()
                    .drag_to_scroll(false)
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        self.render_logs(gui_config, ui);
                    });
            }

            if ui.ctx().input(|i| i.modifiers.command) && ui.ctx().input(|i| i.key_pressed(Key::F5))
            {
                self.kill_gui_and_target(data);
            }
        });
    }

    fn kill_gui_and_target(&mut self, data: &AppLaunchConfig) {
        process::kill(data.target_process_id(), || {
            tracing::info!("Exiting because Command + F5 was pressed.");
            self.should_exit_app.store(true, Ordering::Relaxed);
        });
    }

    fn render_logs(&mut self, gui_config: &Config, ui: &mut eframe::egui::Ui) {
        let clip_rect = ui.painter().clip_rect();

        let info_log_color = if gui_config.dark_mode {
            Color32::WHITE
        } else {
            Color32::BLACK
        };

        let is_log_selection_button_down = self.log_selection.is_log_selection_button_down(ui);

        let mut i = 0;
        for log in &mut self.logs {
            Self::render_log(
                &mut self.log_selection,
                &mut self.filter,
                log,
                gui_config,
                info_log_color,
                i,
                ui,
                clip_rect,
                is_log_selection_button_down,
            );

            i += 1;
        }

        let log_count = self.logs.len();
        if gui_config.log_auto_scroll_to_bottom
            && self.scroll.last_log_count != log_count
            && !self.log_selection.button_just_got_down
        {
            ui.scroll_with_delta(Vec2::new(0., f32::NEG_INFINITY));
            self.scroll.last_log_count = log_count;
        }

        if ui.ctx().input(|i| i.modifiers.command) && ui.ctx().input(|i| i.key_pressed(Key::C)) {
            match ClipboardProvider::new() {
                Ok(ctx_) => {
                    let mut ctx: ClipboardContext = ctx_;

                    let selected_logs: Vec<String> = self
                        .logs
                        .iter()
                        .filter(|x| x.is_selected)
                        .map(|x| x.data().to_string())
                        .collect();

                    let selected_logs_string = selected_logs.join("\n");

                    match ctx.set_contents(selected_logs_string) {
                        Ok(_) => {}
                        Err(err) => {
                            tracing::error!("Failed copying logs to clipboard: {}", err);
                        }
                    }
                }
                Err(_) => {}
            }
        }
    }

    fn render_log(
        log_selection: &mut LogSelection,
        filter: &mut Filter,
        log: &mut BepInExLogEntry,
        gui_config: &Config,
        info_log_color: Color32,
        i: u32,
        ui: &mut Ui,
        clip_rect: Rect,
        is_log_selection_button_down: bool,
    ) {
        if log.level() > gui_config.log_level_filter {
            return;
        }

        if !ConsoleTab::does_log_match_text_filter(&filter.text, log) {
            return;
        }

        let log_color = get_color_from_log_level(log, info_log_color);

        let is_selected = i >= log_selection.smallest_index_of_hovered_log
            && i <= log_selection.biggest_index_of_hovered_log;

        log.is_selected = is_selected;

        let ui_log_entry = ui.add(SelectableLabel::new(
            is_selected,
            RichText::new(log.data()).color(log_color),
        ));
        let mut log_rect = ui_log_entry.rect;
        log_rect.max.x = clip_rect.max.x;

        if is_log_selection_button_down {
            if ui.rect_contains_pointer(log_rect) {
                if !log_selection.button_just_got_down {
                    log_selection.button_just_got_down = true;
                    log_selection.first_index_of_log_that_is_selected = i;
                    log_selection.smallest_index_of_hovered_log = i;
                    log_selection.biggest_index_of_hovered_log = i;
                }

                if i <= log_selection.first_index_of_log_that_is_selected {
                    log_selection.smallest_index_of_hovered_log = i;
                }

                if i >= log_selection.first_index_of_log_that_is_selected {
                    log_selection.biggest_index_of_hovered_log = i;
                }
            }

            views::utils::egui::scroll_when_trying_to_select_stuff_above_or_under_rect(
                ui, clip_rect,
            );
        }
    }

    fn does_log_match_text_filter(text_filter: &String, log: &BepInExLogEntry) -> bool {
        if !text_filter.is_empty() {
            if !log
                .data()
                .to_lowercase()
                .contains(&text_filter.to_lowercase())
            {
                return false;
            }
        }

        return true;
    }

    fn render_footer(&mut self, data: &AppLaunchConfig, gui_config: &mut Config, ctx: &Context) {
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.add_space(2.0);

            ui.horizontal(|ui| {
                ui.label(RichText::new("Log Level Filtering: ").font(FontId::proportional(15.0)));

                let log_level_text = gui_config.log_level_filter.to_string();
                ui.add(
                    Slider::new(
                        &mut gui_config.log_level_filter,
                        LogLevel::Fatal..=LogLevel::All,
                    )
                    .show_value(false)
                    .text(log_level_text),
                );
            });

            views::BepInExGUI::render_useful_buttons_footer(
                ui,
                ctx,
                &data.game_folder_full_path(),
                &data.bepinex_log_output_file_full_path(),
                data.target_process_id(),
            );
        });
    }

    fn render_console_first_time_disclaimer(&mut self, ctx: &Context, gui_config: &mut Config) {
        CentralPanel::default().show(ctx, |_| {
                Window::new("Console Disclaimer")
                    .collapsible(false)
                    .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                    .show(ctx, |ui| {
                        ui.heading(
                            r#"The console is meant to be used by mod developers.
                                If any of your mods is malfunctioning and that you wish to receive help in the #tech-support channel of the discord:
                                Please use the buttons below and use the "Copy Log File" button, and drag and drop it in the #tech-support channel."#);

                        if self.disclaimer.first_time_showing_it {
                            self.disclaimer.time_when_disclaimer_showed_up =
                                Some(SystemTime::now());
                            self.disclaimer.first_time_showing_it = false;
                        }

                        if let Ok(_elapsed) =
                            self.disclaimer.time_when_disclaimer_showed_up.unwrap().elapsed()
                        {
                            let elapsed = _elapsed.as_secs() as i64;
                            if 9 - elapsed >= 0 {
                                ui.label(
                                    RichText::new((10 - elapsed).to_string())
                                        .font(FontId::proportional(20.0)),
                                );
                            } else {
                                if ui
                                    .button(
                                        RichText::new("Ok").font(FontId::proportional(20.0)),
                                    )
                                    .clicked()
                                {
                                    gui_config.first_time_console_disclaimer = false;
                                }
                            }
                        }
                    });
            });
    }

    fn render_kill_gui_and_game_butto(
        &mut self,
        ui: &mut Ui,
        pause_game_btn_size: Vec2,
        data: &AppLaunchConfig,
    ) {
        let kill_game_btn_text = "Kill GUI & Game";
        let kill_game_btn_size =
            views::utils::egui::compute_text_size(ui, kill_game_btn_text, true, false, None);

        ui.add_space(
            ui.available_width()
                - pause_game_btn_size.x
                - kill_game_btn_size.x
                - (ui.spacing().item_spacing.x * 4.),
        );

        if ui
            .add(Button::new(
                RichText::new(kill_game_btn_text).text_style(egui::TextStyle::Heading),
            ))
            .clicked()
        {
            self.kill_gui_and_target(data);
        }
    }

    fn render_pause_game_button(&mut self, ui: &mut Ui, data: &AppLaunchConfig) -> Vec2 {
        let pause_game_btn_text = if self.target_process_paused {
            "Resume Game"
        } else {
            "Pause Game"
        };
        let pause_game_btn_size =
            views::utils::egui::compute_text_size(ui, pause_game_btn_text, true, false, None);

        ui.add_space(
            ui.available_width() - pause_game_btn_size.x - (ui.spacing().item_spacing.x * 2.),
        );

        if ui
            .add(Button::new(
                RichText::new(pause_game_btn_text).text_style(egui::TextStyle::Heading),
            ))
            .clicked()
        {
            if self.target_process_paused {
                self.target_process_paused = !process::resume(data.target_process_id());
            } else {
                self.target_process_paused = process::suspend(data.target_process_id());
            }
        }
        pause_game_btn_size
    }

    fn render_log_text_filter_input(
        &mut self,
        ui: &mut Ui,
        mods_combo_box: Response,
        gui_config: &mut Config,
    ) {
        ui.add_sized(
            mods_combo_box.rect.size(),
            TextEdit::singleline(&mut self.filter.text)
                .text_color(if gui_config.dark_mode {
                    Color32::WHITE
                } else {
                    Color32::BLACK
                })
                .hint_text(WidgetText::from("Filter Text").color(ui.style().visuals.text_color())),
        );
    }

    fn render_log_mod_filter(&mut self, ui: &mut Ui) -> Response {
        let mods_combo_box = ComboBox::from_id_source("combo_box_mods_log_filter")
            .width(200.)
            .show_index(
                ui,
                &mut self.log_selection.selected_index_in_mods_combo_box,
                self.mods.len(),
                |i| self.mods[i].name(),
            );

        if mods_combo_box.changed() {
            if self.log_selection.selected_index_in_mods_combo_box == 0 {
                self.filter.text = "".to_string();
            } else {
                self.filter.text = self.mods[self.log_selection.selected_index_in_mods_combo_box]
                    .name()
                    .to_string();
            }
        }
        mods_combo_box
    }
}

fn get_color_from_log_level(log: &mut BepInExLogEntry, info_log_color: Color32) -> Color32 {
    match log.level() {
        LogLevel::None => Color32::RED,
        LogLevel::Fatal => Color32::RED,
        LogLevel::Error => Color32::RED,
        LogLevel::Warning => Color32::YELLOW,
        LogLevel::Message => info_log_color,
        LogLevel::Info => info_log_color,
        LogLevel::Debug => info_log_color,
        LogLevel::All => info_log_color,
    }
}

impl Tab for ConsoleTab {
    fn name(&self) -> &str {
        "Console"
    }

    fn update_top_panel(
        &mut self,
        data: &AppLaunchConfig,
        gui_config: &mut Config,
        ui: &mut eframe::egui::Ui,
    ) {
        egui::menu::bar(ui, move |ui| {
            // controls
            ui.with_layout(Layout::left_to_right(Align::default()), move |ui| {
                let cur_cursor_rect = ui.cursor();

                ui.label(RichText::new("Log Filtering: ").font(FontId::proportional(20.0)));
                let mods_combo_box = self.render_log_mod_filter(ui);
                self.render_log_text_filter_input(ui, mods_combo_box, gui_config);

                render_auto_scroll_to_bottom_checkbox(ui, gui_config);

                ui.set_cursor(cur_cursor_rect);

                let pause_game_btn_size = self.render_pause_game_button(ui, data);

                ui.set_cursor(cur_cursor_rect);

                self.render_kill_gui_and_game_butto(ui, pause_game_btn_size, data);
            });
        });
    }

    fn update(
        &mut self,
        data: &AppLaunchConfig,
        gui_config: &mut Config,
        ctx: &eframe::egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        self.update_mod_receiver();
        self.update_log_receiver();

        if gui_config.first_time_console_disclaimer {
            self.render_console_first_time_disclaimer(ctx, gui_config);
        } else {
            self.render_footer(data, gui_config, ctx);

            self.render(data, gui_config, ctx);
        }
    }
}

fn render_auto_scroll_to_bottom_checkbox(ui: &mut Ui, gui_config: &mut Config) {
    ui.checkbox(
        &mut gui_config.log_auto_scroll_to_bottom,
        "Auto Scroll to Bottom",
    );
}

impl ConsoleTab {
    fn update_mod_receiver(&mut self) {
        match self.mod_receiver.try_recv() {
            Ok(mod_) => {
                self.mods.push(mod_);
            }
            Err(_) => {}
        }
    }

    fn update_log_receiver(&mut self) {
        // loop until the channel is emptied
        // if we don't do that the maximum amount of log received is
        // tied to the framerate of the GUI
        loop {
            match self.log_receiver.try_recv() {
                Ok(log) => {
                    self.logs.push(log);
                }
                Err(err) => match err {
                    crossbeam_channel::TryRecvError::Empty => {
                        break;
                    }
                    crossbeam_channel::TryRecvError::Disconnected => {
                        break;
                    }
                },
            }
        }
    }
}
