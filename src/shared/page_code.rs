use egui::Ui;
use crate::MyApp;
#[cfg(target_arch = "wasm32")]
use crate::get_and_display_certs;

pub fn see_certs(self_: &mut MyApp, ui: &mut Ui) {
    #[cfg(target_arch = "wasm32")]
    get_and_display_certs(self_, ui);
    
    for i in &self_.certs {
        ui.label(format!("User Ids: {}", i.userids().map(|cert| String::from_utf8_lossy(cert.userid().value()).to_string()).collect::<Vec<String>>()[0]));
    }
    if self_.err != String::new() {
        ui.label(&self_.err);
    }
}

pub fn new_cert(self_: &mut MyApp, ui: &mut Ui) {
    ui.label("New Cert");
}