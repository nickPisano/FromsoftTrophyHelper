#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod games;
mod guides;
mod steam;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1360.0, 860.0])
            .with_min_inner_size([980.0, 620.0])
            .with_title("FromSoft Trophy Helper"),
        ..Default::default()
    };
    eframe::run_native(
        "fromsoft-trophy-helper",
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
