#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // eframe - hide console window on Windows in release
#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en");

use eframe::egui::{self, ViewportClass};

mod message_prop;
mod utils;

use message_prop::MessageProp;

fn main() -> eframe::Result<()> {
    let options_for_eframe = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(eframe::emath::Vec2::new(1024.0, 800.0))
            .with_icon(egui::IconData::default()),
        ..eframe::NativeOptions::default()
    };
    let title = format!(
        "{} {}",
        String::from("screen props"),
        utils::get_version_text()
    );
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
    message: MessageProp,
}

impl ScreenPropsApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Self {
            message: MessageProp {
                title: String::from("Message Prop"),
                main_text: String::from("Hello, world."),
                button_text: String::from("Close"),
                size_width: 200.0,
                size_height: 400.0,
                show_prop: false,
            },
        }
    }
}

impl eframe::App for ScreenPropsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            let input = ctx.input(|input| input.clone());
            if input.key_pressed(egui::Key::F1) {
                self.message.show_prop = true;
            };
            self.message.update(ctx);
            if ui.button("Quit").clicked() {
                std::process::exit(0);
            };
            ui.add_space(15.0);
            ui.label("Message title: ");
            ui.text_edit_singleline(&mut self.message.title);
            ui.label("Message text: ");
            ui.text_edit_multiline(&mut self.message.main_text);
            ui.label("Message button text: ");
            ui.text_edit_singleline(&mut self.message.button_text);
            ui.label("Message window size: ");
            ui.add(egui::Slider::new(&mut self.message.size_width, 100.0..=600.0).text("width"));
            ui.add(egui::Slider::new(&mut self.message.size_height, 100.0..=600.0).text("height"));
            ui.add_space(15.0);
        });
    }
}
