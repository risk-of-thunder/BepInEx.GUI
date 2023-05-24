use eframe::egui::*;

pub(crate) fn compute_text_size(
    ui: &mut Ui,
    text: &str,
    is_heading: bool,
    is_wrap: bool,
    font_size_: Option<f32>,
) -> Vec2 {
    let mut rich_text = RichText::new(text);

    if is_heading {
        rich_text = rich_text.heading();
    } else if let Some(font_size) = font_size_ {
        rich_text = rich_text.font(FontId::proportional(font_size))
    }

    let label = Label::new(rich_text).wrap(is_wrap);

    let label_layout_in_ui = label.layout_in_ui(ui);
    let text_size = label_layout_in_ui.1.size();

    text_size
}

pub(crate) fn scroll_when_trying_to_select_stuff_above_or_under_rect(ui: &mut Ui, clip_rect: Rect) {
    // if self.button_currently_down && !ui.rect_contains_pointer(clip_rect) {
    if !ui.rect_contains_pointer(clip_rect) {
        let mut scroll = Vec2::new(0., 0.);
        let dist = clip_rect.bottom() - ui.input(|i| i.pointer.interact_pos().unwrap().y);

        if dist < 0. {
            scroll.y = dist;
            scroll.y *= 0.005;
        } else if dist > 0. {
            scroll.y = clip_rect.top() - ui.input(|i| i.pointer.interact_pos().unwrap().y);
            scroll.y *= 0.005;
        }

        ui.scroll_with_delta(scroll);
    }
}

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
