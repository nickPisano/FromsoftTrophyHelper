//! FromSoft-flavored visual theme: warm near-black surfaces, parchment text,
//! tarnished-gold accents, and a serif display face for headings.

use eframe::egui;

// Palette --------------------------------------------------------------

pub const GOLD: egui::Color32 = egui::Color32::from_rgb(214, 181, 106);
pub const GREEN: egui::Color32 = egui::Color32::from_rgb(133, 196, 137);
pub const RED: egui::Color32 = egui::Color32::from_rgb(224, 108, 92);
pub const ORANGE: egui::Color32 = egui::Color32::from_rgb(235, 165, 80);
pub const BLUE: egui::Color32 = egui::Color32::from_rgb(126, 170, 224);
pub const PURPLE: egui::Color32 = egui::Color32::from_rgb(185, 143, 235);
pub const DIM: egui::Color32 = egui::Color32::from_rgb(146, 138, 122);

pub const PARCHMENT: egui::Color32 = egui::Color32::from_rgb(230, 221, 199);
const BG_WINDOW: egui::Color32 = egui::Color32::from_rgb(19, 16, 13);
const BG_PANEL: egui::Color32 = egui::Color32::from_rgb(26, 22, 17);
const BG_FAINT: egui::Color32 = egui::Color32::from_rgb(36, 31, 23);
const BG_EXTREME: egui::Color32 = egui::Color32::from_rgb(12, 10, 8);
const BG_WIDGET: egui::Color32 = egui::Color32::from_rgb(43, 37, 28);
const BG_HOVER: egui::Color32 = egui::Color32::from_rgb(56, 48, 35);
const BG_ACTIVE: egui::Color32 = egui::Color32::from_rgb(70, 59, 41);
const STROKE_FAINT: egui::Color32 = egui::Color32::from_rgb(64, 55, 41);

pub const SELECTION: egui::Color32 = egui::Color32::from_rgb(94, 74, 34);
pub const TOAST_BG: egui::Color32 = egui::Color32::from_rgb(43, 37, 28);
pub const ROW_HOVER: egui::Color32 = egui::Color32::from_rgb(38, 33, 25);

const DISPLAY_FAMILY: &str = "display";

/// Install fonts and the full style. Call once at startup.
pub fn apply(ctx: &egui::Context) {
    install_fonts(ctx);

    let mut visuals = egui::Visuals::dark();
    visuals.override_text_color = Some(PARCHMENT);
    visuals.window_fill = BG_PANEL;
    visuals.panel_fill = BG_WINDOW;
    visuals.faint_bg_color = BG_FAINT;
    visuals.extreme_bg_color = BG_EXTREME;
    visuals.hyperlink_color = GOLD;
    visuals.selection.bg_fill = SELECTION;
    visuals.selection.stroke = egui::Stroke::new(1.0, GOLD);
    visuals.window_stroke = egui::Stroke::new(1.0, STROKE_FAINT);

    visuals.widgets.noninteractive.bg_fill = BG_PANEL;
    visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, STROKE_FAINT);
    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, PARCHMENT);
    visuals.widgets.inactive.bg_fill = BG_WIDGET;
    visuals.widgets.inactive.weak_bg_fill = BG_WIDGET;
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, PARCHMENT);
    visuals.widgets.hovered.bg_fill = BG_HOVER;
    visuals.widgets.hovered.weak_bg_fill = BG_HOVER;
    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, GOLD.gamma_multiply(0.6));
    visuals.widgets.active.bg_fill = BG_ACTIVE;
    visuals.widgets.active.weak_bg_fill = BG_ACTIVE;
    visuals.widgets.open.bg_fill = BG_WIDGET;
    visuals.widgets.open.weak_bg_fill = BG_WIDGET;

    ctx.set_visuals(visuals);

    ctx.all_styles_mut(|style| {
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.button_padding = egui::vec2(10.0, 5.0);
        style.spacing.menu_margin = egui::Margin::same(8);
    });
}

fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "cinzel".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/fonts/Cinzel-Bold.ttf"
        ))),
    );
    // Display family: Cinzel first, then every default proportional font as
    // fallback (keeps emoji and non-Latin glyphs working in headings).
    let mut display = fonts
        .families
        .get(&egui::FontFamily::Proportional)
        .cloned()
        .unwrap_or_default();
    display.insert(0, "cinzel".to_owned());
    fonts
        .families
        .insert(egui::FontFamily::Name(DISPLAY_FAMILY.into()), display);
    ctx.set_fonts(fonts);
}

/// Heading text in the serif display face.
pub fn display(text: impl Into<String>, size: f32) -> egui::RichText {
    egui::RichText::new(text.into())
        .family(egui::FontFamily::Name(DISPLAY_FAMILY.into()))
        .size(size)
        .color(GOLD)
}
