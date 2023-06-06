use crossbeam_channel::Receiver;

use eframe::{
    egui::{self, CentralPanel, Context, Label, Layout, RichText, ScrollArea, TopBottomPanel},
    emath::Align,
    epaint::FontId,
};

use crate::{
    app,
    config::{launch::AppLaunchConfig, Config},
    data::bepinex_mod::BepInExMod,
    views,
};

use super::Tab;

pub struct GeneralTab {
    mod_receiver: Receiver<BepInExMod>,
    mods: Vec<BepInExMod>,
}

impl GeneralTab {
    pub fn new(mods_receiver: Receiver<BepInExMod>) -> Self {
        Self {
            mod_receiver: mods_receiver,
            mods: Vec::new(),
        }
    }

    fn render_footer(&mut self, data: &AppLaunchConfig, ctx: &Context) {
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.add_space(25.0);

            app::BepInExGUI::render_useful_buttons_footer(
                ui,
                ctx,
                data.game_folder_full_path(),
                data.bepinex_log_output_file_full_path(),
                data.target_process_id(),
            );
        });
    }

    fn render(&mut self, gui_config: &Config, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            if self.mods.is_empty() {
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
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        self.render_mods(gui_config, ui);
                    });
            }
        });
    }

    fn render_mods(&self, _gui_config: &Config, ui: &mut egui::Ui) {
        for mod_ in self.mods.as_slice() {
            ui.add(Label::new(RichText::new(mod_.to_string())));
        }
    }

    fn update_mod_receiver(&mut self) {
        if let Ok(mod_) = self.mod_receiver.try_recv() {
            self.mods.push(mod_);
        }
    }
}

impl Tab for GeneralTab {
    fn name(&self) -> &str {
        "General"
    }

    fn update_top_panel(
        &mut self,
        data: &AppLaunchConfig,
        _gui_config: &mut Config,
        ui: &mut eframe::egui::Ui,
    ) {
        egui::menu::bar(ui, move |ui| {
            // controls
            ui.with_layout(Layout::left_to_right(Align::default()), |ui| {
                let target_is_loading_text = format!(
                    "Modded {} is loading, you can close this window at any time.",
                    data.target_name()
                );
                ui.label(RichText::new(target_is_loading_text).font(FontId::proportional(20.0)));
            });
        });

        let loaded_mod_count = self.mods.len();
        let loaded_mods_text = format!("Loaded Mods: {loaded_mod_count}");
        ui.label(RichText::new(loaded_mods_text).font(FontId::proportional(20.0)));
    }

    fn update(
        &mut self,
        data: &AppLaunchConfig,
        gui_config: &mut Config,
        ctx: &eframe::egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        self.update_mod_receiver();

        self.render_footer(data, ctx);

        self.render(gui_config, ctx);
    }
}
