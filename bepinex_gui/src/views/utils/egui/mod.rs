use eframe::egui::*;

pub fn compute_text_size(
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
        rich_text = rich_text.font(FontId::proportional(font_size));
    }

    let label = Label::new(rich_text).wrap(is_wrap);

    let label_layout_in_ui = label.layout_in_ui(ui);

    label_layout_in_ui.1.size()
}

pub fn scroll_when_trying_to_select_stuff_above_or_under_rect(
    ui: &mut Ui,
    rect: Rect,
) -> Option<Vec2> {
    if !ui.rect_contains_pointer(rect) {
        if let Some(interact_pos) = ui.input(|i| i.pointer.interact_pos()) {
            let mut scroll = Vec2::new(0., 0.);
            let dist = rect.bottom() - interact_pos.y;

            const SCROLL_SPEED: f32 = 0.25;

            if dist < 0. {
                scroll.y = dist;
                scroll.y *= SCROLL_SPEED;
            } else if dist > 0. {
                scroll.y = rect.top() - interact_pos.y;
                scroll.y *= SCROLL_SPEED;
            }

            return Some(scroll);
        }
    }

    None
}
