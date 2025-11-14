use egui::{Color32, Rounding, Stroke, Style, Visuals};

pub fn setup_custom_style(ctx: &egui::Context) {
    let mut style = Style {
        visuals: Visuals::dark(),
        ..Default::default()
    };

    // Epic Games-inspired dark theme
    style.visuals.window_fill = Color32::from_rgb(18, 18, 18);
    style.visuals.panel_fill = Color32::from_rgb(25, 25, 28);
    style.visuals.faint_bg_color = Color32::from_rgb(32, 32, 36);
    style.visuals.extreme_bg_color = Color32::from_rgb(10, 10, 12);

    // Text colors
    style.visuals.override_text_color = Some(Color32::from_rgb(230, 230, 230));

    // Button styling
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(45, 45, 48);
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(180, 180, 180));
    style.visuals.widgets.inactive.rounding = Rounding::same(4.0);

    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(60, 60, 65);
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::from_rgb(230, 230, 230));

    style.visuals.widgets.active.bg_fill = Color32::from_rgb(0, 121, 214);
    style.visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);

    // Selection color (Epic Games blue)
    style.visuals.selection.bg_fill = Color32::from_rgb(0, 121, 214);
    style.visuals.selection.stroke = Stroke::new(1.0, Color32::from_rgb(0, 121, 214));

    ctx.set_style(style);
}

pub const CARD_BG: Color32 = Color32::from_rgb(32, 32, 36);
