mod pages;
mod page_code;
pub mod checkbox;
mod new_cert_status;

use eframe::egui;
use egui::Color32;
use sequoia_openpgp::Cert;
use pages::Pages;
use page_code::*;
use checkbox::CheckboxDropdown;
use new_cert_status::CertStatus;

#[cfg(not(target_arch = "wasm32"))]
use crate::get_certs;
#[cfg(target_arch = "wasm32")]
use crate::get_and_display_certs;

pub struct MyApp {
    pub ui_scale: f32,
    pub certs: Vec<Cert>,
    pub err: String,
    pub page: Pages,
    pub cert_status: CertStatus,
    pub bg_color: [u8; 4],
    #[cfg(target_arch = "wasm32")]
    pub gpg_armoured: String,
}

impl Default for MyApp {
    fn default() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let mut certs: Vec<Cert> = vec![];
        #[allow(unused_mut)] // fixes warning when building for wasm32
        let mut err = "".to_owned();
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(cert) = get_certs() {
                certs = cert;
            } else {
                eprintln!("gpg --export -a output corrupted or failed");
                err = "gpg --export -a output corrupted or failed".to_owned();
            }
        }
        
        #[cfg(target_arch = "wasm32")]
        let certs = vec![];
        
        Self {
            ui_scale: 1.,
            err,
            certs,
            cert_status: CertStatus::default(),
            page: Pages::default(),
            bg_color: [35, 35, 35, 255],
            #[cfg(target_arch = "wasm32")]
            gpg_armoured: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = Color32::from_rgba_unmultiplied(self.bg_color[0], self.bg_color[1], self.bg_color[2], self.bg_color[3]);
        ctx.set_visuals(visuals);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.input(|key| {
                if (key.key_pressed(egui::Key::Plus) && key.modifiers.ctrl) || (key.modifiers.ctrl && key.raw_scroll_delta[1] < 0.) {
                    self.ui_scale *= 1.1
                }
                if (key.key_pressed(egui::Key::Minus) && key.modifiers.ctrl) || (key.modifiers.ctrl && key.raw_scroll_delta[1] > 0.) {
                    self.ui_scale *= 0.9
                } 
            });
            
            egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.page, Pages::Certs, "Certs");
                    ui.selectable_value(&mut self.page, Pages::NewCert, "New Cert");
                    ui.selectable_value(&mut self.page, Pages::Style, "Style");
                    // ui.selectable_value(&mut self.page, Pages::About, "About");
                });
            });
            
            ui.add_space(20.);
            match self.page {
                Pages::Certs => see_certs(self, ui),
                Pages::NewCert => new_cert(self, ui),
                Pages::Style => style(self, ui),
            }
        });
    }
}