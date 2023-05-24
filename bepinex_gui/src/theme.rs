use eframe;
use eframe::{egui::*, *};

fn parse_color(color: &str) -> egui::Color32 {
    #![allow(clippy::identity_op)]

    let color = color.strip_prefix('#').unwrap();
    if color.len() == 6 {
        // RGB
        let color = u32::from_str_radix(color, 16).unwrap();
        egui::Color32::from_rgb(
            ((color >> 16) & 0xff) as u8,
            ((color >> 8) & 0xff) as u8,
            ((color >> 0) & 0xff) as u8,
        )
    } else if color.len() == 8 {
        // RGBA
        let color = u32::from_str_radix(color, 16).unwrap();
        egui::Color32::from_rgba_unmultiplied(
            ((color >> 24) & 0xff) as u8,
            ((color >> 16) & 0xff) as u8,
            ((color >> 8) & 0xff) as u8,
            ((color >> 0) & 0xff) as u8,
        )
    } else {
        panic!()
    }
}

fn global_path_value<'json>(
    value: &'json serde_json::Value,
    global_path: &str,
) -> &'json serde_json::Value {
    follow_path_or_die(value, global_path).get("value").unwrap()
}

fn follow_path<'json>(
    mut value: &'json serde_json::Value,
    path: &str,
) -> Option<&'json serde_json::Value> {
    let path = path.strip_prefix('{')?;
    let path = path.strip_suffix('}')?;
    for component in path.split('.') {
        value = value.get(component)?;
    }
    Some(value)
}

fn follow_path_or_die<'json>(
    json: &'json serde_json::Value,
    json_path: &str,
) -> &'json serde_json::Value {
    follow_path(json, json_path).unwrap_or_else(|| panic!("Failed to find {json_path:?}"))
}

fn get_alias_str<'json>(json: &'json serde_json::Value, alias_path: &str) -> &'json str {
    let global_path = follow_path_or_die(json, alias_path).as_str().unwrap();
    global_path_value(json, global_path).as_str().unwrap()
}

fn get_aliased_color(json: &serde_json::Value, alias_path: &str) -> egui::Color32 {
    parse_color(get_alias_str(json, alias_path))
}

/// Margin on all sides of views.
pub fn view_padding() -> f32 {
    12.0
}

pub fn window_rounding() -> f32 {
    12.0
}

pub fn small_rounding() -> f32 {
    4.0
}

// taken from https://github.com/rerun-io/rerun/blob/main/crates/re_ui/src/design_tokens.rs#L25
pub fn get_dark_theme() -> egui::Style {
    let json: serde_json::Value =
        serde_json::from_str(include_str!("../assets/design_tokens.json"))
            .expect("Failed to parse data/design_tokens.json");

    let mut egui_style = egui::Style {
        visuals: egui::Visuals::dark(),
        ..Default::default()
    };

    // We want labels and buttons to have the same height.
    // Intuitively, we would just assign font_size to
    // the interact_size, but in practice text height does not match
    // font size (for unknown reason), so we fudge it for now:

    egui_style.spacing.interact_size.y = 15.0;
    // egui_style.spacing.interact_size.y = font_size;

    let panel_bg_color = get_aliased_color(&json, "{Alias.Color.Surface.Default.value}");
    let floating_color = get_aliased_color(&json, "{Alias.Color.Surface.Floating.value}");
    // let floating_color = Color32::from_gray(38); // TODO(emilk): change the content of the design_tokens.json origin instead

    // Used as the background of text edits, scroll bars and others things
    // that needs to look different from other interactive stuff.
    // We need this very dark, since the theme overall is very, very dark.
    egui_style.visuals.extreme_bg_color = egui::Color32::BLACK;

    egui_style.visuals.widgets.noninteractive.weak_bg_fill = panel_bg_color;
    egui_style.visuals.widgets.noninteractive.bg_fill = panel_bg_color;

    egui_style.visuals.button_frame = true;
    egui_style.visuals.widgets.inactive.weak_bg_fill = Default::default(); // Buttons have no background color when inactive
    egui_style.visuals.widgets.inactive.bg_fill = Color32::from_gray(40);
    // get_aliased_color(&json, "{Alias.Color.Action.Default.value}"); // too dark to see, especially for scroll bars

    {
        // Background colors for buttons (menu buttons, blueprint buttons, etc) when hovered or clicked:
        // let hovered_color = get_aliased_color(&json, "{Alias.Color.Action.Hovered.value}");
        let hovered_color = Color32::from_gray(64); // TODO(emilk): change the content of the design_tokens.json origin instead
        egui_style.visuals.widgets.hovered.weak_bg_fill = hovered_color;
        egui_style.visuals.widgets.hovered.bg_fill = hovered_color;
        egui_style.visuals.widgets.active.weak_bg_fill = hovered_color;
        egui_style.visuals.widgets.active.bg_fill = hovered_color;
        egui_style.visuals.widgets.open.weak_bg_fill = hovered_color;
        egui_style.visuals.widgets.open.bg_fill = hovered_color;
    }

    {
        // Turn off strokes around buttons:
        egui_style.visuals.widgets.inactive.bg_stroke = Default::default();
        egui_style.visuals.widgets.hovered.bg_stroke = Default::default();
        egui_style.visuals.widgets.active.bg_stroke = Default::default();
        egui_style.visuals.widgets.open.bg_stroke = Default::default();
    }

    {
        // Expand hovered and active button frames:
        egui_style.visuals.widgets.hovered.expansion = 2.0;
        egui_style.visuals.widgets.active.expansion = 2.0;
        egui_style.visuals.widgets.open.expansion = 2.0;
    }

    egui_style.visuals.selection.bg_fill =
        get_aliased_color(&json, "{Alias.Color.Highlight.Default.value}");

    egui_style.visuals.widgets.noninteractive.bg_stroke.color = Color32::from_gray(30); // from figma. separator lines, panel lines, etc

    let subudued = get_aliased_color(&json, "{Alias.Color.Text.Subdued.value}");
    let default = get_aliased_color(&json, "{Alias.Color.Text.Default.value}");
    let strong = get_aliased_color(&json, "{Alias.Color.Text.Strong.value}");

    egui_style.visuals.widgets.noninteractive.fg_stroke.color = subudued; // non-interactive text
    egui_style.visuals.widgets.inactive.fg_stroke.color = default; // button text
    egui_style.visuals.widgets.active.fg_stroke.color = strong; // strong text and active button text

    egui_style.visuals.popup_shadow = egui::epaint::Shadow::NONE;
    egui_style.visuals.window_shadow = egui::epaint::Shadow::NONE;

    egui_style.visuals.window_fill = floating_color; // tooltips and menus
    egui_style.visuals.window_stroke = egui::Stroke::NONE;
    egui_style.visuals.panel_fill = panel_bg_color;

    egui_style.visuals.window_rounding = window_rounding().into();
    egui_style.visuals.menu_rounding = window_rounding().into();
    let small_rounding = small_rounding().into();
    egui_style.visuals.widgets.noninteractive.rounding = small_rounding;
    egui_style.visuals.widgets.inactive.rounding = small_rounding;
    egui_style.visuals.widgets.hovered.rounding = small_rounding;
    egui_style.visuals.widgets.active.rounding = small_rounding;
    egui_style.visuals.widgets.open.rounding = small_rounding;

    egui_style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    egui_style.spacing.menu_margin = view_padding().into();

    // Add stripes to grids and tables?
    egui_style.visuals.striped = false;
    egui_style.visuals.indent_has_left_vline = false;
    egui_style.spacing.button_padding = egui::Vec2::new(1.0, 0.0); // Makes the icons in the blueprint panel align
    egui_style.spacing.indent = 14.0; // From figma

    egui_style.debug.show_blocking_widget = false; // turn this on to debug interaction problems

    egui_style.spacing.combo_width = 8.0; // minimum width of ComboBox - keep them small, with the down-arrow close.

    egui_style.spacing.scroll_bar_inner_margin = 2.0;
    egui_style.spacing.scroll_bar_width = 6.0;
    egui_style.spacing.scroll_bar_outer_margin = 2.0;

    egui_style
}

pub(crate) fn configure_fonts(ctx: &Context) {
    let mut font_def = FontDefinitions::default();
    font_def.font_data.insert(
        "MesloLGS".to_string(),
        FontData::from_static(include_bytes!("../assets/fonts/MesloLGS_NF_Regular.ttf")),
    );

    font_def
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "MesloLGS".to_string());

    ctx.set_fonts(font_def);
}
