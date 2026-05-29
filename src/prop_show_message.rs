pub struct PropShowMessage {
    pub message_text: String,
    pub button_text: String,
}

impl PropShowMessage {
    pub fn show(self, ui: &mut eframe::egui::Ui) {
        ui.label(self.message_text);
    }
}
