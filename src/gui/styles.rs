use egui::{Color32, Rounding, Stroke, Style, Visuals};

pub fn setup_custom_style(ctx: &egui::Context) {
    let mut style = Style {
        visuals: Visuals::dark(),
        ..Default::default()
    };

    // Epic Games-inspired dark theme with richer colors
    style.visuals.window_fill = Color32::from_rgb(16, 18, 22);
    style.visuals.panel_fill = Color32::from_rgb(22, 24, 28);
    style.visuals.faint_bg_color = Color32::from_rgb(28, 30, 34);
    style.visuals.extreme_bg_color = Color32::from_rgb(12, 14, 18);

    // Text colors - brighter for better contrast
    style.visuals.override_text_color = Some(Color32::from_rgb(245, 245, 245));

    // Button styling - Enhanced Epic Games style
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(50, 52, 58);
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(200, 200, 200));
    style.visuals.widgets.inactive.rounding = Rounding::same(5.0);

    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(65, 68, 75);
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::from_rgb(240, 240, 240));
    style.visuals.widgets.hovered.rounding = Rounding::same(5.0);

    style.visuals.widgets.active.bg_fill = Color32::from_rgb(0, 121, 214);
    style.visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    style.visuals.widgets.active.rounding = Rounding::same(5.0);

    // Selection color (Epic Games blue)
    style.visuals.selection.bg_fill = Color32::from_rgb(0, 121, 214);
    style.visuals.selection.stroke = Stroke::new(1.5, Color32::from_rgb(0, 121, 214));

    // Enhance spacing
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);

    ctx.set_style(style);
}

// Epic Games-inspired color palette
pub const CARD_BG: Color32 = Color32::from_rgb(28, 28, 32);
pub const CARD_HOVER_BG: Color32 = Color32::from_rgb(35, 35, 40);
pub const EPIC_BLUE: Color32 = Color32::from_rgb(0, 121, 214);
pub const EPIC_BLUE_HOVER: Color32 = Color32::from_rgb(0, 145, 255);
pub const SUCCESS_GREEN: Color32 = Color32::from_rgb(76, 175, 80);
pub const ERROR_RED: Color32 = Color32::from_rgb(244, 67, 54);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(160, 160, 170);
