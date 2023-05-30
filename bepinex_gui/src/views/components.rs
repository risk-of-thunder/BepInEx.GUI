use eframe::egui::*;

pub fn button(text: &str, ui: &mut Ui, button_size: Vec2, font_size: f32) -> bool {
    let btn = Button::new(RichText::new(text).font(FontId::proportional(font_size)));
    ui.add_sized(button_size, btn).clicked()
}

pub fn colored_button(
    text: &str,
    ui: &mut Ui,
    button_size: Vec2,
    font_size: f32,
    fill_color: Option<Color32>,
) -> bool {
    let mut btn = Button::new(RichText::new(text).font(FontId::proportional(font_size)));
    if let Some(color) = fill_color {
        btn = btn.fill(color);
        btn = btn.stroke(Stroke::default());
    }

    ui.add_sized(button_size, btn).clicked()
}

pub fn checkbox(
    bool_ref: &mut bool,
    text: &str,
    ui: &mut Ui,
    button_size: Vec2,
    font_size: f32,
) -> bool {
    ui.add_sized(
        button_size,
        Checkbox::new(
            bool_ref,
            RichText::new(text).font(FontId::proportional(font_size)),
        ),
    )
    .clicked()
}
