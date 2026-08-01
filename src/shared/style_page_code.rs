use crate::MyApp;
use crate::custom_widgets::StyleEditor;
use egui::Ui;

impl MyApp {
    pub fn style(&mut self, ui: &mut Ui) {
        ui.add(StyleEditor::new(&mut self.style));
    }
}
