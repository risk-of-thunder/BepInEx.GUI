use eframe::egui::*;

pub(crate) fn compute_text_size(ui: &mut Ui, text: &str, is_heading: bool, is_wrap: bool) -> Vec2 {
    let label = if is_heading {
        Label::new(RichText::new(text).heading()).wrap(is_wrap)
    } else {
        Label::new(RichText::new(text)).wrap(is_wrap)
    };

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
