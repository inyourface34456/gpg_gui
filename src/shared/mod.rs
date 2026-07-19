mod helpers;
mod new_cert_status;
mod page_code;
mod pages;
mod style_page_code;

use crate::platform::Storage;
use eframe::egui;
use egui::Context;
use new_cert_status::CertStatus;
use pages::Pages;
use sequoia_openpgp::Cert;
use sequoia_openpgp::cert::CipherSuite;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

#[derive(Serialize, Deserialize)]
pub struct MyApp {
    pub ui_scale: f32,
    #[serde(skip_serializing, skip_deserializing)]
    pub certs: Vec<Cert>,
    #[serde(skip_serializing, skip_deserializing)]
    pub priv_certs: Vec<Cert>,
    pub err: String,
    pub page: Pages,
    pub cert_status: CertStatus,
    #[cfg(target_arch = "wasm32")]
    pub show_warning: bool,
    pub style: eframe::egui::style::Style,
    #[cfg(target_arch = "wasm32")]
    pub gpg_armoured: String,
    #[cfg(target_arch = "wasm32")]
    pub gpg_armoured_priv: String,
    storage: Storage,
}

impl Default for MyApp {
    fn default() -> Self {
        let storage = match Storage::read() {
            Some(mut myapp) => {
                (myapp.certs, myapp.priv_certs) = match crate::platform::get_certs("", "") {
                    Ok(certs) => certs,
                    Err(err) => {
                        log::error!("{}@{}: {}", file!(), line!(), err.to_string());
                        (vec![], vec![])
                    }
                };
                myapp.cert_status.crypto_algo = CipherSuite::Cv25519;
                return myapp;
            }
            None => Storage::default(),
        };

        Self {
            ui_scale: 1.,
            err: String::new(),
            certs: vec![],
            priv_certs: vec![],
            cert_status: CertStatus::default(),
            page: Pages::default(),
            style: eframe::egui::style::Style::default(),
            #[cfg(target_arch = "wasm32")]
            show_warning: true,
            #[cfg(target_arch = "wasm32")]
            gpg_armoured: String::new(),
            #[cfg(target_arch = "wasm32")]
            gpg_armoured_priv: String::new(),
            storage,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_style(self.style.clone());
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.input(|key| {
                if (key.key_pressed(egui::Key::Plus) && key.modifiers.ctrl) || (key.modifiers.ctrl && key.raw_scroll_delta[1] > 0.) {
                    self.ui_scale *= 1.1
                }
                if (key.key_pressed(egui::Key::Minus) && key.modifiers.ctrl) || (key.modifiers.ctrl && key.raw_scroll_delta[1] < 0.) {
                    self.ui_scale *= 0.9
                }
                if let Some(multi_touch) = key.multi_touch() {
                    let raw_delta = multi_touch.zoom_delta;
                    if (raw_delta - 1.0).abs() > 0.008 {
                        let damped = 1.0 + (raw_delta - 1.0) * 0.5;
                        self.ui_scale *= damped;
                    }
                }
            });

            ctx.set_zoom_factor(self.ui_scale);

            egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.page, Pages::Certs, "Certs");
                    ui.selectable_value(&mut self.page, Pages::NewCert, "New Cert");
                    // ui.selectable_value(&mut self.page, Pages::Sign, "Sign/Verify");
                    ui.selectable_value(&mut self.page, Pages::Style, "Style");
                    ui.selectable_value(&mut self.page, Pages::Debug, "Debug");
                    ui.selectable_value(&mut self.page, Pages::About, "About");
                    if ui.button("Save").clicked() {
                        self.cert_status.password.zeroize();
                        self.cert_status.secret_text.zeroize();
                        self.priv_certs = vec![];
                        #[cfg(target_arch = "wasm32")]
                        self.gpg_armoured_priv.zeroize();
                        let immutable_self: &MyApp = &self;
                        self.storage.write(immutable_self);
                    }
                });
            });

            #[cfg(target_arch = "wasm32")]
            {
                let warning_window = egui::containers::Window::new("WARNING!!!");
                if self.show_warning {
                    warning_window.show(ctx, |ui| {
                        ui.label("This is the web version, and as such, is not 100% garenteed to be totally secure, due to the fact that the crypto libaries I am using do not suport wasm  fully, and as such, I recommend that you download or compile the native version. As of right now, the only feature I was forced to disable is the constant time crypto.");
                        if ui.button("Dismiss").clicked() {
                            self.show_warning = false;
                        }
                    });
                }
            }

            // self.display_error(ctx, file!(), line!());

            ui.add_space(20.);
            match self.page {
                Pages::Certs => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        self.see_certs(ui);
                    });
                }
                Pages::NewCert => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        self.new_cert(ui);
                    });
                }
                Pages::Style => {
                    egui::ScrollArea::vertical().auto_shrink(egui::Vec2b::new(false, false)).show(ui, |ui| {
                        self.style(ui);
                    });
                }
                Pages::Debug => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        self.debug(ui);
                    });
                }
                Pages::Sign => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        self.sign(ui);
                    });
                }
                Pages::About => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        self.about(ui);
                    });
                }
            }
        });
    }
}

impl MyApp {
    pub fn display_error(&mut self, ctx: &Context, file: &str, line: u32) {
        let err_window = egui::containers::Window::new("Error");
        if !self.err.is_empty() {
            err_window.show(ctx, |ui| {
                ui.label(format!(
                    "Error: {}\nFile: {} @ line {}",
                    self.err, file, line
                ));
                if ui.button("Dismiss").clicked() {
                    self.err = String::new();
                }
            });
        }
    }
}
