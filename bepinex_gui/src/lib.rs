mod bepinex_gui;

pub use bepinex_gui::{BepInExGUI, Msg, PADDING};
use eframe::{
    egui::{
        CentralPanel, Context, Hyperlink, Label, RichText, ScrollArea, Separator, TextStyle,
        TopBottomPanel, Ui, Visuals,
    },
    App,
};

const APP_NAME: &str = "BepInEx GUI";

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

        self.render_top_panel(ctx, frame);
        render_footer(ctx);

        self.update_receive_logs_from_channel();

        CentralPanel::default().show(ctx, |ui| {
            if self.logs.is_empty() {
                ui.vertical_centered_justified(|ui| {
                    ui.heading("Loading ⌛");
                });
            } else {
                render_header(ui);
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        self.render_logs(ui);
                    });
            }
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, &APP_NAME, &self.config);
    }
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading(APP_NAME);
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}

fn render_footer(ctx: &Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.add(Label::new(
                RichText::new("API source: newsapi.org")
                    .small()
                    .text_style(TextStyle::Monospace),
            ));
            ui.add(Hyperlink::from_label_and_url(
                "creativcoder/headlines",
                "https://github.com/creativcoder/headlines",
            ));
            ui.add_space(10.);
        })
    });
}
