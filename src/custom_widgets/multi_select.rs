use egui::WidgetText;
use egui::{Response, Ui, Widget};

pub struct MultiSelect<'a, T> {
    options: &'a [T],
    selected: &'a mut Vec<T>,
    id_salt: &'a str, // needed if you show more than one on the same screen
}

impl<'a, T> MultiSelect<'a, T> {
    pub fn new(id_salt: &'a str, options: &'a [T], selected: &'a mut Vec<T>) -> Self {
        Self {
            options,
            selected,
            id_salt,
        }
    }

    // /// Builder-style config, matching egui's own conventions.
    // pub fn max_selected(mut self, max: usize) -> Self {
    //     self.max_selected = max;
    //     self
    // }
}

impl<'a, T> Widget for MultiSelect<'a, T>
where
    T: std::cmp::PartialEq,
    T: Clone,
    T: std::fmt::Debug,
{
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

            if !self.selected.is_empty() {
                ui.horizontal_wrapped(|ui| {
                    let mut to_remove = None;
                    for item in self.selected.iter() {
                        let r = ui
                            .group(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{:?}", item));
                                    if ui.small_button("x").clicked() {
                                        to_remove = Some(item.clone());
                                    }
                                })
                                .response
                            })
                            .inner;
                        merge(r, &mut response);
                    }
                    if let Some(item) = to_remove {
                        for (index, item_) in self.selected.iter().enumerate() {
                            if item_ == &item {
                                self.selected.remove(index);
                                break;
                            }
                        }
                    }
                });
            }

            ui.horizontal_wrapped(|ui| {
                for item in self.options {
                    let is_selected = self.selected.contains(item);
                    let r = ui.selectable_label(is_selected, format!("{:?}", item));
                    if r.clicked() {
                        if is_selected {
                            for (index, item_) in self.selected.iter().enumerate() {
                                if item_ == item {
                                    self.selected.remove(index);
                                    break;
                                }
                            }
                        } else {
                            self.selected.push(item.clone());
                        }
                    }
                    merge(r, &mut response);
                }
            });

            response.expect("MultiSelect must draw at least one option")
        })
        .inner
    }
}
