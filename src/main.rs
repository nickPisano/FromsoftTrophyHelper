#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod games;
mod guides;
mod steam;
mod theme;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1360.0, 860.0])
            .with_min_inner_size([980.0, 620.0])
            .with_title("FromSoft Trophy Helper")
            .with_icon(app_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "fromsoft-trophy-helper",
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}

fn app_icon() -> egui::IconData {
    let img = image::load_from_memory(include_bytes!("../assets/icon-256.png"))
        .expect("bundled icon decodes")
        .into_rgba8();
    let (width, height) = img.dimensions();
    egui::IconData {
        rgba: img.into_raw(),
        width,
        height,
    }
}
