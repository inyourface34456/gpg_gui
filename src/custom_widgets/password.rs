use crate::shared::helpers;
use egui::{Response, Ui, Widget};
use zxcvbn::zxcvbn;

pub struct PasswordViewer<'a> {
    id_salt: &'a str, // needed if you show more than one on the same screen
    password: &'a mut String,
    password2: &'a mut String,
    password_vis: &'a mut (bool, bool),
    email: &'a String,
    comment: &'a String,
    display_name: &'a String,
}

impl<'a> PasswordViewer<'a> {
    pub fn new(
        id_salt: &'a str,
        password: &'a mut String,
        password2: &'a mut String,
        password_vis: &'a mut (bool, bool),
        email: &'a String,
        comment: &'a String,
        display_name: &'a String,
    ) -> Self {
        Self {
            id_salt,
            password,
            password2,
            password_vis,
            email,
            comment,
            display_name,
        }
    }
}

impl Widget for PasswordViewer<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let id = ui.make_persistent_id(self.id_salt);

        // ui.push_id lets us draw multiple rows/children and still
        // hand back a single combined Response for the whole widget.
        ui.push_id(id, |ui| {
            let mut response: Option<Response> = None;
            let merge = |r: Response, response: &mut Option<Response>| {
                *response = Some(match response.take() {
                    Some(existing) => existing.union(r),
                    None => r,
                });
            };

            let t = ui.horizontal(|ui| {
                ui.label("Password*: ");
                ui.add(
                    egui::TextEdit::singleline(self.password)
                        .password(!self.password_vis.0)
                        .hint_text("Password"),
                );
                ui.checkbox(&mut self.password_vis.0, "Show Password");
            });

            merge(t.response, &mut response);

            let t = ui.horizontal(|ui| {
                ui.label("Confirm Password*: ");
                ui.add(
                    egui::TextEdit::singleline(self.password2)
                        .password(!self.password_vis.1)
                        .hint_text("Password"),
                );
                ui.checkbox(&mut self.password_vis.1, "Show Password");
            });

            merge(t.response, &mut response);

            let score = match zxcvbn(
                self.password,
                &[self.comment, self.email, self.display_name],
            ) {
                Ok(score) => score,
                Err(_) => zxcvbn("a", &[]).expect("No idea how this can fail"),
            };
            let (label, color) = helpers::score_info(score.score());

            let t = ui.horizontal(|ui| {
                ui.label("Password Strength");
                let bar = egui::ProgressBar::new(f32::from(score.score()) / 4.)
                    .show_percentage()
                    .fill(color)
                    .desired_width(200.);
                ui.add(bar);
                ui.colored_label(color, label);
            });

            merge(t.response, &mut response);

            if let Some(feedback) = score.feedback() {
                if let Some(warning) = feedback.warning() {
                    let t = ui.label(format!("Warning: {warning}"));
                    merge(t, &mut response);
                }
                for sugestion in feedback.suggestions() {
                    let t = ui.label(format!("Suggestion: {sugestion}"));
                    merge(t, &mut response);
                }
            }

            if !self.password2.is_empty() && self.password != self.password2 {
                let t = ui.label(
                    egui::RichText::new("Password does not match!")
                        .color(egui::Color32::from_rgb(255, 0, 0)),
                );
                merge(t, &mut response);
            }
            // merge(t, &mut response);
            response.expect("MultiSelect must draw at least one option")
        })
        .inner
    }
}
