use chrono::{Duration, Local, NaiveDateTime};
use eframe::egui::{self, ViewportClass};
use std::path::PathBuf;

pub struct ScrollingLogProp {
    pub lines_file_path: PathBuf,
    pub lines: Vec<String>,
    pub last_timestamp: NaiveDateTime,
    pub size_width: f32,
    pub size_height: f32,
    pub show_prop: bool,
}

impl ScrollingLogProp {
    pub fn update(&mut self, ctx: &eframe::egui::Context) {
        if self.show_prop {
            let viewport_id = eframe::egui::ViewportId::from_hash_of("scrollinglogpropviewport");
            let viewport_builder = eframe::egui::ViewportBuilder::default()
                .with_inner_size((self.size_width, self.size_height))
                .with_title("Log");
            let viewport_ui = |ctx: &eframe::egui::Context, _: ViewportClass| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if ui.input(|i| i.viewport().close_requested()) {
                        self.show_prop = false;
                    }
                    let now = Local::now().naive_local();
                    if now > (self.last_timestamp + random_duration()) {
                        self.last_timestamp = now;
                        self.lines.push(format!(
                            "-{}-[{}]- here goes a text...",
                            random_level(),
                            &self.last_timestamp,
                        ));
                    }
                    for line in &self.lines {
                        ui.label(line);
                    }
                });
            };
            ctx.show_viewport_immediate(viewport_id, viewport_builder, viewport_ui);
        }
    }
}

fn random_level() -> String {
    String::from("Info")
}

fn random_duration() -> Duration {
    Duration::seconds(1)
}
