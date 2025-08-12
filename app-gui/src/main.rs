#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod system_font;

use eframe::egui;

use crate::{app::MyApp, system_font::set_system_fonts};

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Todo App",
        options,
        Box::new(|cc| {
            set_system_fonts(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}
