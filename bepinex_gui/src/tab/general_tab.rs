use std::{cell::RefCell, path::PathBuf, rc::Rc};

use eframe::{
    egui::{self, CentralPanel, Context, Label, Layout, RichText, ScrollArea, TopBottomPanel},
    epaint::FontId,
};
use sysinfo::Pid;

use crate::{
    bepinex_gui, bepinex_gui_config::BepInExGUIConfig, bepinex_log::BepInExLog,
    bepinex_mod::BepInExMod, egui_utils,
};

use super::Tab;

pub struct GeneralTab {
    mods: Rc<RefCell<Option<Vec<BepInExMod>>>>,
    logs: Rc<RefCell<Option<Vec<BepInExLog>>>>,
    target_name: String,
    game_folder_full_path: PathBuf,
    bepinex_log_output_file_full_path: PathBuf,
    target_process_id: Pid,
}

impl GeneralTab {
    pub fn new(
        mods: Rc<RefCell<Option<Vec<BepInExMod>>>>,
        logs: Rc<RefCell<Option<Vec<BepInExLog>>>>,
        target_process_id: Pid,
        target_name: String,
        game_folder_full_path: PathBuf,
        bepinex_log_output_file_full_path: PathBuf,
    ) -> Self {
        Self {
            mods,
            logs,
            target_process_id,
            target_name,
            game_folder_full_path,
            bepinex_log_output_file_full_path,
        }
    }

    fn render_footer(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            bepinex_gui::render_useful_buttons_footer(
                ui,
                ctx,
                &self.game_folder_full_path,
                &self.bepinex_log_output_file_full_path,
                self.target_process_id,
            );
        });
    }

    fn render(&mut self, gui_config: &BepInExGUIConfig, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            if self.logs.borrow_mut().as_mut().unwrap().is_empty() {
                ui.vertical_centered_justified(|ui| {
                    let loading_text = "Loading ⌛";
                    let text_size =
                        egui_utils::compute_text_size(ui, loading_text, true, false, None);
                    ui.add_space(ui.available_height() / 2. - text_size.y);
                    ui.heading(loading_text);
                });
            } else {
                ui.spacing_mut().scroll_bar_width = 16.;
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        self.render_mods(gui_config, ui);
                    });
            }
        });
    }

    fn render_mods(&self, _gui_config: &BepInExGUIConfig, ui: &mut egui::Ui) {
        let mods_borrow = self.mods.borrow();
        let mods = mods_borrow.as_ref().unwrap().iter();
        for mod_ in mods {
            ui.add(Label::new(RichText::new(mod_.to_string())));
        }
    }
}

impl Tab for GeneralTab {
    fn name(&self) -> &str {
        "General"
    }

    fn update_top_panel(&mut self, gui_config: &mut BepInExGUIConfig, ui: &mut eframe::egui::Ui) {
        let target_name = self.target_name.clone();
        egui::menu::bar(ui, move |ui| {
            // controls
            ui.with_layout(Layout::left_to_right(), |ui| {
                let target_is_loading_text = format!(
                    "Modded {} is loading, you can close this window at any time.",
                    target_name
                );
                ui.label(RichText::new(target_is_loading_text).font(FontId::proportional(20.0)));

                bepinex_gui::render_theme_button(gui_config, ui);
            });
        });

        let mods_borrow = self.mods.borrow();
        let mods = mods_borrow.as_ref().unwrap();
        let loaded_mods_text = format!("Loaded Mods: {}", (mods.len() - 1));
        ui.label(RichText::new(loaded_mods_text).font(FontId::proportional(20.0)));
    }

    fn update(
        &mut self,
        gui_config: &mut BepInExGUIConfig,
        ctx: &eframe::egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        self.render_footer(ctx);

        self.render(gui_config, ctx);
    }
}
