use std::time::SystemTime;

use eframe::{
    egui::{emath::Vec2, Context, Label, RichText, Window},
    emath::Align2,
    epaint::FontId,
};

use crate::config::Config;

pub struct Disclaimer {
    pub first_time_showing_it: bool,
    pub time_when_disclaimer_showed_up: Option<SystemTime>,
}

pub fn show(config: &mut Config, disclaimer: &mut Disclaimer, ctx: &Context) {
    Window::new("Disclaimer")
        .collapsible(false).min_width(ctx.available_rect().size().x).anchor(Align2::CENTER_CENTER, Vec2::ZERO).show(ctx, |ui| {
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

                if disclaimer.first_time_showing_it {
                    disclaimer.time_when_disclaimer_showed_up = Some(SystemTime::now());
                    disclaimer.first_time_showing_it = false;
                }

                if let Ok(_elapsed) = disclaimer.time_when_disclaimer_showed_up.unwrap().elapsed() {
                    let elapsed = _elapsed.as_secs() as i64;
                    const NEEDED_TIME_BEFORE_CLOSABLE:i64 = 9;
                    let can_close = elapsed > NEEDED_TIME_BEFORE_CLOSABLE;
                    if can_close {
                        if ui.button(RichText::new("Ok").font(FontId::proportional(20.0))).clicked() {
                            config.first_time = false;
                        }
                    }
                    else {
                        ui.label(RichText::new(((NEEDED_TIME_BEFORE_CLOSABLE + 1) - elapsed).to_string()).font(FontId::proportional(20.0)));
                    }
                }
            });
        });
}
