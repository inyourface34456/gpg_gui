use crate::shared::new_cert_status::ExpireTime;
use egui::{Response, Ui, Widget};

pub struct ExpireTimeSelector<'a> {
    // options: &'a [T],
    expire: &'a mut Option<ExpireTime>,
    id_salt: &'a str, // needed if you show more than one on the same screen
}

impl<'a> ExpireTimeSelector<'a> {
    pub fn new(id_salt: &'a str, expire: &'a mut Option<ExpireTime>) -> Self {
        Self { id_salt, expire }
    }
}

impl<'a> Widget for ExpireTimeSelector<'a> {
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

            let mut temp = match self.expire {
                Some(ex) => ex.to_string(),
                None => String::new()
            };

            let t = ui.horizontal(|ui| {
                        ui.label(self.id_salt);
                        ui.add_enabled_ui(
                            !self.expire.is_none()
                                || *self.expire == Some(ExpireTime::Custom(1)),
                            |ui| {
                                egui::ComboBox::from_label("")
                                    .selected_text(format!("{}", temp))
                                    .show_ui(ui, |ui| {
                                        #[rustfmt::skip]
                                        ui.selectable_value(self.expire, Some(ExpireTime::FiveDays), "Five Days");
                                        #[rustfmt::skip]
                                        ui.selectable_value(self.expire, Some(ExpireTime::FiveYears), "Five Years");
                                        #[rustfmt::skip]
                                        ui.selectable_value(self.expire, Some(ExpireTime::OneDay), "One Day");
                                        #[rustfmt::skip]
                                        ui.selectable_value(self.expire, Some(ExpireTime::OneHour), "One Hour");
                                        #[rustfmt::skip]
                                        ui.selectable_value(self.expire, Some(ExpireTime::OneMonth), "One Month");
                                        #[rustfmt::skip]
                                        ui.selectable_value(self.expire, Some(ExpireTime::OneWeek), "One Week");
                                        #[rustfmt::skip]
                                        ui.selectable_value(self.expire, Some(ExpireTime::OneYear), "One Year");
                                        #[rustfmt::skip]
                                        ui.selectable_value(self.expire, Some(ExpireTime::SixHour), "Six Hours");
                                        #[rustfmt::skip]
                                        ui.selectable_value(self.expire, Some(ExpireTime::SixMonths), "Six Months");
                                        #[rustfmt::skip]
                                        ui.selectable_value(self.expire, Some(ExpireTime::TwoMonths), "Two Months");
                                        #[rustfmt::skip]
                                        ui.selectable_value(self.expire, Some(ExpireTime::TwoWeeks), "Two Weeks");
                                        #[rustfmt::skip]
                                        ui.selectable_value(self.expire, Some(ExpireTime::TwoYears), "Two Years");
                                        #[rustfmt::skip]
                                        ui.selectable_value(self.expire, Some(ExpireTime::Custom(1)), "Custom");
                                        temp = match self.expire {
                                            Some(ex) => ex.to_string(),
                                            None => String::new()
                                        };
                                    });
                            },
                        );

                        if ui.button("Never Expire").clicked() {
                            if self.expire.is_none() {
                                *self.expire = Some(ExpireTime::OneDay);
                                temp = self.expire.unwrap().to_string();
                            } else {
                                *self.expire = None;
                                temp = String::new();
                            }
                        }
                    });

                    merge(t.response, &mut response);

                    if matches!(self.expire, Some(ExpireTime::Custom(_))) {
                        let t = ui.text_edit_singleline(&mut temp);
                        merge(t, &mut response);
                    }

                    let temp_2: u64 = match temp.parse() {
                        Ok(num) => num,
                        Err(err) => {
                            log::error!("{}", err);
                            match self.expire {
                                Some(t) => (*t).into(),
                                None => 0,
                            }
                        }
                    };

                    *self.expire = if temp_2 == 0 {
                        None
                    } else {
                        Some(temp_2.into())
                    };

            response.expect("MultiSelect must draw at least one option")
        })
        .inner
    }
}
