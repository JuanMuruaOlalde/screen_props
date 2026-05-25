#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // eframe - hide console window on Windows in release
#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en");

use eframe::egui;

mod utils;

fn main() -> eframe::Result<()> {
    let options_for_eframe = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(eframe::emath::Vec2::new(1024.0, 800.0))
            .with_icon(egui::IconData::default()),
        ..eframe::NativeOptions::default()
    };
    let title = format!("{} {}", String::from("screen props"), utils::get_version_text());
    eframe::run_native(
        &title,
        options_for_eframe,
        Box::new(|ctx| {
            egui_extras::install_image_loaders(&ctx.egui_ctx);
            Ok(Box::new(ScreenPropsApp::new(ctx)))
        }),
    )
}

struct ScreenPropsApp {

}

impl Default for ScreenPropsApp {
    fn default() -> Self {
        Self {

        }
    }
}

impl ScreenPropsApp {
        pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }

    
}

impl eframe::App for ScreenPropsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, world.");
        });
    }

}