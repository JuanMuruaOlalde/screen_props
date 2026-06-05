#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // eframe - hide console window on Windows in release
#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en");

use chrono::{Datelike, NaiveDateTime, Timelike};
use eframe::egui::{self};
use std::{fmt, path::PathBuf};

mod message_prop;
mod scrolling_log_prop;
mod utils;

use message_prop::MessageProp;

use crate::scrolling_log_prop::ScrollingLogProp;

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
    log: ScrollingLogProp,
}

impl ScreenPropsApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Self {
            message: MessageProp {
                title: String::from("This is a message Prop"),
                main_text: String::from("Hello, world.\n\nAnd some more text..."),
                button_text: String::from("Close"),
                size_width: 175.0,
                size_height: 120.0,
                show_prop: false,
            },
            log: ScrollingLogProp::new(
                PathBuf::from("./scrolling_log_text_lines_sample.txt"),
                chrono::Local::now().naive_local(),
                1,
                5,
                800.0,
                400.0,
            ),
        }
    }
}

impl eframe::App for ScreenPropsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Please, stay focused in this window in order to allow keypress events.");
            ui.label("- press F1 to show the MESSAGE prop ; and F2 to hide it.");
            let input = ctx.input(|input| input.clone());
            if input.key_pressed(egui::Key::F1) {
                self.message.show_prop = true;
            }
            if input.key_pressed(egui::Key::F2) {
                self.message.show_prop = false;
            }
            self.message.update(ctx);
            ui.label("- press F3 to show the SCROLLING LOG prop ; and F4 to hide it.");
            if input.key_pressed(egui::Key::F3) {
                self.log.reset_displayed_lines();
                self.log.show_prop = true;
            };
            if input.key_pressed(egui::Key::F4) {
                self.log.show_prop = false;
            };
            self.log.update(ctx);
            ui.add_space(15.0);
            ui.separator();
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
            ui.separator();
            ui.add_space(15.0);
            ui.label("Source file for log text lines: ");
            let mut text_filepath = self.log.lines_file_path.to_string_lossy().to_string();
            ui.text_edit_singleline(&mut text_filepath);
            self.log.lines_file_path = PathBuf::from(text_filepath);
            ui.label("Start date&time for timestamps:");
            let mut text_datetime: TextDateTime = TextDateTime::from(&self.log.last_timestamp);
            ui.horizontal(|ui| {
                ui.label("Year: ");
                ui.add(egui::DragValue::new(&mut text_datetime.year).speed(1));
                ui.label("  Month: ");
                ui.add(egui::DragValue::new(&mut text_datetime.month).range(1..=12));
                ui.label("  Day: ");
                ui.add(egui::DragValue::new(&mut text_datetime.day).range(1..=31));
                ui.label("  Hour: ");
                ui.add(egui::DragValue::new(&mut text_datetime.hour).range(1..=24));
                ui.label("  Minute: ");
                ui.add(egui::DragValue::new(&mut text_datetime.minute).range(1..=60));
                ui.label("  Second: ");
                ui.add(egui::DragValue::new(&mut text_datetime.second).range(1..=60));
            });
            match NaiveDateTime::parse_from_str(
                format!("{}", text_datetime).as_str(),
                "%Y/%m/%d %H:%M:%S",
            ) {
                Ok(datetime) => self.log.last_timestamp = datetime,
                Err(_) => (),
            }
            ui.horizontal(|ui| {
                ui.label("Random delay between lines:  from ");
                ui.add(egui::DragValue::new(&mut self.log.delay_seconds_from).range(1..=3600));
                ui.label(" to ");
                ui.add(egui::DragValue::new(&mut self.log.delay_seconds_to).range(1..=3600));
                ui.label(" seconds")
            });
            ui.label("Scrolling log window size: ");
            ui.add(egui::Slider::new(&mut self.log.size_width, 100.0..=1024.0).text("width"));
            ui.add(egui::Slider::new(&mut self.log.size_height, 100.0..=1024.0).text("height"));
            ui.add_space(15.0);
            ui.separator();
            ui.add_space(15.0);
        });
    }
}

pub struct TextDateTime {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

impl From<&NaiveDateTime> for TextDateTime {
    fn from(datetime: &NaiveDateTime) -> Self {
        Self {
            year: datetime.year(),
            month: datetime.month(),
            day: datetime.day(),
            hour: datetime.hour(),
            minute: datetime.minute(),
            second: datetime.second(),
        }
    }
}

impl fmt::Display for TextDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}/{}/{} {}:{}:{}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}
