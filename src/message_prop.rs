use eframe::egui::{self, ViewportClass};

pub struct MessageProp {
    pub title: String,
    pub main_text: String,
    pub button_text: String,
    pub size_width: f32,
    pub size_height: f32,
    pub show_prop: bool,
}

impl MessageProp {
    pub fn update(&mut self, ctx: &eframe::egui::Context) {
        if self.show_prop {
            let viewport_id = eframe::egui::ViewportId::from_hash_of("messagepropviewport");
            let viewport_builder = eframe::egui::ViewportBuilder::default()
                .with_inner_size((self.size_width, self.size_height))
                .with_title(&self.title);
            let viewport_ui = |ctx: &eframe::egui::Context, _: ViewportClass| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.add_space(10.0);
                    ui.label(&self.main_text);
                    ui.add_space(25.0);
                    if ui.button(&self.button_text).clicked() {
                        self.show_prop = false;
                    };
                });
            };
            ctx.show_viewport_immediate(viewport_id, viewport_builder, viewport_ui);
        }
    }
}
