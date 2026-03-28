use egui::Color32;
use egui::Ui;
use sequoia_openpgp::types::KeyFlags;
use crate::MyApp;
use crate::shared::checkbox::CheckboxDropdown;
use sequoia_openpgp::cert::prelude::*;
use sequoia_openpgp::cert::CipherSuite;
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
    ui.heading("New Certifacate");
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Key Flags");
        self_.cert_status.key_flags.show(ui);
    });
    ui.add_space(7.);
    
    let key_flags = KeyFlags::new([0]);
    // ["Authentication", "Certification", "Sigining", "Transport Encryption", "Storage Encryption"]
    let enabled_flags = self_.cert_status.key_flags.selected_by_pos();
    key_flags.set_authentication_to(enabled_flags[0])
             .set_certification_to(enabled_flags[1])
             .set_signing_to(enabled_flags[2])
             .set_transport_encryption_to(enabled_flags[3])
             .set_storage_encryption_to(enabled_flags[4]);
    
    ui.horizontal(|ui| {
        ui.label("Primary Crypto Algorithm");
        egui::ComboBox::from_label(" ")
            .selected_text(format!("{:?}", self_.cert_status.crypto_algo))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self_.cert_status.crypto_algo, CipherSuite::Cv25519, "Cv25519");
                ui.selectable_value(&mut self_.cert_status.crypto_algo, CipherSuite::Cv448, "Cv448");
                ui.selectable_value(&mut self_.cert_status.crypto_algo, CipherSuite::P256, "NistP256");
                ui.selectable_value(&mut self_.cert_status.crypto_algo, CipherSuite::P384, "NistP384");
                ui.selectable_value(&mut self_.cert_status.crypto_algo, CipherSuite::P521, "NistP521");
                ui.selectable_value(&mut self_.cert_status.crypto_algo, CipherSuite::RSA2k, "RSA2k");
                ui.selectable_value(&mut self_.cert_status.crypto_algo, CipherSuite::RSA3k, "RSA3k");
                ui.selectable_value(&mut self_.cert_status.crypto_algo, CipherSuite::RSA4k, "RSA4k");
            }
        );
    });
    
    ui.horizontal(|ui| {
        ui.label("Display Name: ");
        ui.text_edit_singleline(&mut self_.cert_status.display_name);
    });
    
    ui.horizontal(|ui| {
        ui.label("Comment (optional): ");
        ui.text_edit_singleline(&mut self_.cert_status.comment);
    });
    
    ui.horizontal(|ui| {
        ui.label("Email (optional): ");
        ui.text_edit_singleline(&mut self_.cert_status.email);
    });
    
    let user_id;
    if self_.cert_status.comment.is_empty() && !self_.cert_status.email.is_empty() {
        user_id = format!("{} <{}>", self_.cert_status.display_name, self_.cert_status.email);
    } else if !self_.cert_status.comment.is_empty() && self_.cert_status.email.is_empty() {
        user_id = format!("{} ({})", self_.cert_status.display_name, self_.cert_status.comment);
    } else if !self_.cert_status.comment.is_empty() && !self_.cert_status.email.is_empty() {
        user_id = format!("{} ({}) <{}>", self_.cert_status.display_name, self_.cert_status.comment, self_.cert_status.email);
    } else {
        user_id = self_.cert_status.display_name.clone();
    }
    
    ui.label(user_id);
}

pub fn style(self_: &mut MyApp, ui: &mut Ui) {
    // let style = ui.style_mut();
    ui.horizontal(|ui| {
        ui.label("Background color: ");
        let mut temp = self_.visuals.panel_fill.to_srgba_unmultiplied();
        ui.color_edit_button_srgba_unmultiplied(&mut temp);
        self_.visuals.panel_fill = Color32::from_rgba_unmultiplied(temp[0], temp[1], temp[2], temp[3])
    });
}