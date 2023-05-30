pub(crate) use std::path::PathBuf;

use eframe::{
    self,
    egui::{Button, Context, RichText, TopBottomPanel, Ui, Visuals},
    emath::Vec2,
    epaint::FontId,
};
use sysinfo::Pid;

use crate::{
    app::BepInExGUI,
    backend::{file_explorer_utils, thunderstore},
    data::bepinex_log,
};

pub(crate) mod components;
pub(crate) mod disclaimer;
pub(crate) mod tabs;
pub(crate) mod utils;

impl BepInExGUI {
    pub(crate) fn view_update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        if self.config.theme_just_changed {
            if self.config.dark_mode {
                ctx.set_style(self.dark_theme.clone());
            } else {
                ctx.set_visuals(Visuals::light());
            }

            self.config.theme_just_changed = false;
        }

        if self.config.first_time {
            self.show_first_time_disclaimer(ctx);
        } else {
            self.render_header(ctx, frame);

            let tab = &mut self.tabs[self.config.selected_tab_index];

            tab.update(&self.app_launch_config, &mut self.config, ctx, frame);
        }
    }

    fn show_first_time_disclaimer(&mut self, ctx: &Context) {
        disclaimer::show(&mut self.config, &mut self.disclaimer, ctx);
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
                    &self.app_launch_config,
                    &mut self.config,
                    ui,
                );
                ui.add_space(10.);
            }
        });
    }

    pub fn render_useful_buttons_footer(
        ui: &mut Ui,
        _ctx: &Context,
        game_folder_full_path: &PathBuf,
        bepinex_log_output_file_full_path: &PathBuf,
        target_process_id: Pid,
    ) {
        ui.add_space(3.0);

        ui.horizontal(|ui| {
            const FONT_SIZE: f32 = 18.;
            // let mut FONT_SIZE = 20. * (ui.available_width() / 900.);

            let mut button_size = ui.available_size() / 5.;
            let spacing = ui.available_width() / 8.;
            button_size.y += 25.;

            let placement_cursor = ui.cursor();
            ui.add_space(spacing * 0.5);

            render_open_game_folder_button(ui, button_size, game_folder_full_path, FONT_SIZE);

            ui.set_cursor(placement_cursor);
            ui.add_space(spacing * 3.25);

            render_copy_log_file_button(
                ui,
                button_size,
                bepinex_log_output_file_full_path,
                FONT_SIZE,
            );

            ui.set_cursor(placement_cursor);
            ui.add_space(spacing * 5.85);

            render_open_modding_discord_button(ui, button_size, target_process_id, FONT_SIZE);
        });
        ui.add_space(25.);
    }
}

fn render_open_game_folder_button(
    ui: &mut Ui,
    button_size: Vec2,
    game_folder_full_path: &PathBuf,
    font_size: f32,
) {
    if components::button("Open Game Folder", ui, button_size, font_size) {
        file_explorer_utils::open_path_in_explorer(game_folder_full_path);
    }
}

fn render_copy_log_file_button(
    ui: &mut Ui,
    button_size: Vec2,
    bepinex_log_output_file_full_path: &PathBuf,
    font_size: f32,
) {
    if components::button("Copy Log File", ui, button_size, font_size) {
        bepinex_log::file::open_file_explorer_to_file_and_zip_it_if_needed(
            bepinex_log_output_file_full_path,
            "zipped_log.zip",
        );
    }
}

fn render_open_modding_discord_button(
    ui: &mut Ui,
    button_size: Vec2,
    target_process_id: Pid,
    font_size: f32,
) {
    if components::button("Modding Discord", ui, button_size, font_size) {
        thunderstore::api::open_modding_discord(target_process_id);
    }
}
