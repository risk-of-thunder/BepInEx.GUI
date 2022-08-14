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
