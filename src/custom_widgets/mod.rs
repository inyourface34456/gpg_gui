mod add_userids;
mod expire_time_selector;
mod multi_select;
mod password;
mod style;

pub use add_userids::AddUserids;
pub use expire_time_selector::ExpireTimeSelector;
pub use multi_select::MultiSelect;
pub use password::PasswordViewer;
pub use style::StyleEditor;

// pub struct ExpireTimeSelector<'a> {
//     id_salt: &'a str, // needed if you show more than one on the same screen
// }

// impl<'a> ExpireTimeSelector<'a> {
//     pub fn new(id_salt: &'a str) -> Self {
//         Self { id_salt }
//     }
// }

// impl<'a> Widget for ExpireTimeSelector<'a> {
//     fn ui(self, ui: &mut Ui) -> Response {
//         let id = ui.make_persistent_id(self.id_salt);

//         // ui.push_id lets us draw multiple rows/children and still
//         // hand back a single combined Response for the whole widget.
//         ui.push_id(id, |ui| {
//             let mut response: Option<Response> = None;
//             let merge = |r: Response, response: &mut Option<Response>| {
//                 *response = Some(match response.take() {
//                     Some(existing) => existing.union(r),
//                     None => r,
//                 });
//             };

//             let t = ui.label("test");
//             merge(t, &mut response);
//             response.expect("MultiSelect must draw at least one option")
//         })
//         .inner
//     }
// }
