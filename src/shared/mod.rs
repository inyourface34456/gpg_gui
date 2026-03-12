use eframe::egui;
use sequoia_openpgp::Cert;

#[cfg(not(target_arch = "wasm32"))]
use crate::get_certs;
#[cfg(target_arch = "wasm32")]
use crate::get_and_display_certs;

pub struct MyApp {
    pub ui_scale: f32,
    pub certs: Vec<Cert>,
    pub err: String,
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
            #[cfg(target_arch = "wasm32")]
            gpg_armoured: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let pixels_per_point = ctx.pixels_per_point();
        ctx.set_pixels_per_point(pixels_per_point);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.input(|key| {
                if (key.key_pressed(egui::Key::Plus) && key.modifiers.ctrl) || (key.modifiers.ctrl && key.raw_scroll_delta[1] < 0.) {
                    self.ui_scale *= 1.1
                }
                if (key.key_pressed(egui::Key::Minus) && key.modifiers.ctrl) || (key.modifiers.ctrl && key.raw_scroll_delta[1] > 0.) {
                    self.ui_scale *= 0.9
                } 
            });
            
            #[cfg(target_arch = "wasm32")]
            get_and_display_certs(self, ui);
            
            for i in &self.certs {
                ui.label(format!("User Ids: {}", i.userids().map(|cert| String::from_utf8_lossy(cert.userid().value()).to_string()).collect::<Vec<String>>()[0]));
            }
            if self.err != String::new() {
                ui.label(&self.err);
            }
        });
    }
}