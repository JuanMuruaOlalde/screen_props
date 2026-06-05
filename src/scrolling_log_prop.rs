use chrono::{Duration, Local, NaiveDateTime};
use eframe::egui::{self, Align, Color32, RichText, ViewportClass};
use rand::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub struct ScrollingLogProp {
    pub lines_file_path: PathBuf,
    lines_from_file: Vec<String>,
    last_line_index: usize,
    displayed_lines: Vec<String>,
    pub last_timestamp: NaiveDateTime,
    elapsed_time: NaiveDateTime,
    random_generator: rand::rngs::ThreadRng,
    elapsed_duration: Duration,
    pub delay_seconds_from: i64,
    pub delay_seconds_to: i64,
    pub size_width: f32,
    pub size_height: f32,
    pub show_prop: bool,
}

impl ScrollingLogProp {
    pub fn new(
        lines_file_path: PathBuf,
        start_datetime_for_timestamps: NaiveDateTime,
        delay_seconds_from: i64,
        delay_seconds_to: i64,
        size_width: f32,
        size_height: f32,
    ) -> Self {
        let lines_from_file = load_lines_from_file(&lines_file_path);
        Self {
            lines_file_path,
            lines_from_file,
            last_line_index: 0,
            displayed_lines: Vec::new(),
            last_timestamp: start_datetime_for_timestamps,
            elapsed_time: Local::now().naive_local(),
            random_generator: rand::rng(),
            elapsed_duration: Duration::seconds(1),
            delay_seconds_from,
            delay_seconds_to,
            size_width,
            size_height,
            show_prop: false,
        }
    }

    pub fn reset_displayed_lines(&mut self) {
        self.displayed_lines.clear();
        self.last_line_index = 0;
    }

    fn random_duration(&mut self) -> Duration {
        Duration::seconds(
            self.random_generator
                .random_range(self.delay_seconds_from..=self.delay_seconds_to),
        )
    }

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
                    if now > (self.elapsed_time + self.elapsed_duration) {
                        self.last_timestamp += self.elapsed_duration;
                        self.elapsed_time += self.elapsed_duration;
                        self.elapsed_duration = self.random_duration();
                        if let Some(line) = self.lines_from_file.get(self.last_line_index) {
                            self.last_line_index += 1;
                            if self.last_line_index >= self.lines_from_file.len() {
                                self.last_line_index = 0;
                            }
                            self.displayed_lines
                                .push(format!("[{}]-- {}", &self.last_timestamp, line,));
                        }
                    }
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for line in &self.displayed_lines {
                            if line.to_uppercase().contains("ERROR") {
                                ui.label(RichText::new(line).color(Color32::RED));
                            } else if line.to_uppercase().contains("WARNING") {
                                ui.label(RichText::new(line).color(Color32::DARK_RED));
                            } else {
                                ui.label(line);
                            }
                        }
                        ui.scroll_to_cursor(Some(Align::BOTTOM));
                    })
                });
            };
            ctx.show_viewport_immediate(viewport_id, viewport_builder, viewport_ui);
        }
    }
}

fn load_lines_from_file(lines_file_path: &PathBuf) -> Vec<String> {
    let mut lines_from_file = Vec::new();
    let file = File::open(lines_file_path);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(content) => lines_from_file.push(content),
                    Err(_) => (),
                }
            }
        }
        Err(e) => lines_from_file.push(format!(
            "-ERROR- Cannot load lines from file {} . error: {}",
            lines_file_path.to_string_lossy(),
            e
        )),
    }
    lines_from_file
}
