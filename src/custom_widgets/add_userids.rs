use crate::shared::helpers::user_id_to_componets;
use egui::{Response, Ui, Widget};

pub struct AddUserids<'a> {
    id_salt: &'a str,
    editing_userid: &'a mut usize,
    name: &'a mut String,
    comment: &'a mut String,
    email: &'a mut String,
    userid: &'a mut Vec<String>,
}

impl<'a> AddUserids<'a> {
    pub fn new(
        id_salt: &'a str,
        editing_userid: &'a mut usize,
        name: &'a mut String,
        comment: &'a mut String,
        email: &'a mut String,
        userid: &'a mut Vec<String>,
    ) -> Self {
        Self {
            id_salt,
            editing_userid,
            name,
            comment,
            email,
            userid,
        }
    }
}

impl<'a> Widget for AddUserids<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let id = ui.make_persistent_id(self.id_salt);
        ui.push_id(id, |ui| {
            let mut response: Option<Response> = None;
            let merge = |r: Response, response: &mut Option<Response>| {
                *response = Some(match response.take() {
                    Some(existing) => existing.union(r),
                    None => r,
                });
            };

            let t = egui::containers::ComboBox::from_label("")
                .selected_text(format!(
                    "UserID #{} ({})",
                    *self.editing_userid + 1,
                    self.userid[*self.editing_userid]
                ))
                .show_ui(ui, |ui| {
                    for (index, value) in self.userid.iter().enumerate() {
                        if ui
                            .selectable_value(
                                self.editing_userid,
                                index,
                                format!("UserID #{} ({})", index + 1, value),
                            )
                            .clicked()
                        {
                            let userid_parts =
                                user_id_to_componets(self.userid[*self.editing_userid].clone());
                            *self.name = userid_parts.0;
                            *self.comment = userid_parts.1;
                            *self.email = userid_parts.2;
                        }
                    }
                });
            merge(t.response, &mut response);

            let t = ui.horizontal(|ui| {
                ui.label("Display Name*: ");
                ui.text_edit_singleline(self.name);
            });
            merge(t.response, &mut response);

            let t = ui.horizontal(|ui| {
                ui.label("Comment (optional): ");
                ui.text_edit_singleline(self.comment);
            });
            merge(t.response, &mut response);

            let t = ui.horizontal(|ui| {
                ui.label("Email (optional): ");
                ui.text_edit_singleline(self.email);
            });
            merge(t.response, &mut response);

            let user_id;
            if !self.name.is_empty() {
                if self.comment.is_empty() && !self.email.is_empty() {
                    user_id = format!("{} <{}>", self.name, self.email);
                } else if !self.comment.is_empty() && self.email.is_empty() {
                    user_id = format!("{} ({})", self.name, self.comment);
                } else if !self.comment.is_empty() && !self.email.is_empty() {
                    user_id = format!("{} ({}) <{}>", self.name, self.comment, self.email);
                } else {
                    user_id = self.name.clone();
                }
            } else {
                user_id = String::new()
            }

            self.userid[*self.editing_userid] = user_id.clone();

            ui.horizontal(|ui| {
                let t = ui.button("Add UserID");
                if t.clicked() {
                    self.userid.push(String::new());
                    *self.name = String::new();
                    *self.email = String::new();
                    *self.comment = String::new();
                    *self.editing_userid = self.userid.len() - 1;
                }
                merge(t, &mut response);

                let t = ui.button("Remove Current Userid");

                if t.clicked() && self.userid.len() > 1 {
                    self.userid.remove(*self.editing_userid);
                    *self.editing_userid -= 1;
                    let userid_parts =
                        user_id_to_componets(self.userid[*self.editing_userid].clone());
                    *self.name = userid_parts.0;
                    *self.comment = userid_parts.1;
                    *self.email = userid_parts.2;
                }
                merge(t, &mut response);
            });

            response.expect("Bad news guys")
        })
        .inner
    }
}
