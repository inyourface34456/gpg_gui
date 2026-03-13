pub struct CheckboxDropdown {
    pub label: String,
    pub options: Vec<(String, bool)>,
}

impl CheckboxDropdown {
    pub fn new(label: impl Into<String>, options: Vec<impl Into<String>>) -> Self {
        Self {
            label: label.into(),
            options: options.into_iter().map(|s| (s.into(), false)).collect(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        let selected_count = self.options.iter().filter(|(_, c)| *c).count();
        let button_label = if selected_count == 0 {
            format!("{} V", self.label)
        } else {
            format!("{} ({}) V", self.label, selected_count)
        };

        let button_response = ui.button(button_label);
        let popup_id = ui.make_persistent_id(&self.label);

        if button_response.clicked() {
            ui.memory_mut(|m| m.toggle_popup(popup_id));
        }

        egui::popup_below_widget(
            ui,
            popup_id,
            &button_response,
            egui::PopupCloseBehavior::CloseOnClickOutside,
            |ui| {
                ui.set_min_width(150.0);
                for (label, checked) in &mut self.options {
                    ui.checkbox(checked, label.as_str());
                }
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.small_button("All").clicked() {
                        self.options.iter_mut().for_each(|(_, c)| *c = true);
                    }
                    if ui.small_button("None").clicked() {
                        self.options.iter_mut().for_each(|(_, c)| *c = false);
                    }
                });
            },
        );
    }

    pub fn selected(&self) -> Vec<&str> {
        self.options.iter()
            .filter(|(_, c)| *c)
            .map(|(l, _)| l.as_str())
            .collect()
    }
    
    pub fn selected_by_pos(&self) -> Vec<bool> {
        self.options.iter()
            .map(|(_, l)| *l)
            .collect()
    }
}