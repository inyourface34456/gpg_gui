pub mod helpers;
pub mod new_cert_status;
pub mod page_code;
pub mod pages;
pub mod style_page_code;

use crate::platform;
use crate::platform::Storage;
use crate::try_or_return;
use eframe::egui;
use egui::Context;
use egui::Ui;
use new_cert_status::CertStatus;
use pages::Pages;
use sequoia_openpgp::Cert;
use sequoia_openpgp::Packet;
use sequoia_openpgp::cert::CertParser;
use sequoia_openpgp::packet::Signature;
use sequoia_openpgp::parse::Parse;
use sequoia_openpgp::serialize::SerializeInto;
use serde::{Deserialize, Serialize};
use web_time::{Duration, Instant};

#[derive(Serialize, Deserialize, Clone)]
pub struct MyApp {
    pub ui_scale: f32,
    #[serde(skip)]
    pub certs: Vec<Cert>,
    #[serde(skip)]
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
    #[serde(skip)]
    pub gpg_armoured_priv: String,
    pub storage: Storage,
    #[serde(skip, default = "web_time::Instant::now")]
    pub last_tick: Instant,
    pub interval: Duration,
    pub gpg_errored: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        let storage = match Storage::read() {
            Some(mut myapp) => {
                (myapp.certs, myapp.priv_certs) = match crate::platform::get_certs("", "") {
                    Ok(certs) => certs,
                    Err(err) => {
                        log::error!("{}@{}: {}", file!(), line!(), err);
                        (vec![], vec![])
                    }
                };
                return myapp;
            }
            // this must handle both native and wasm impls, and the native one has feilds, and the wasm one does not, requiring the default call
            #[allow(clippy::default_constructed_unit_structs)]
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
            interval: Duration::from_secs(1),
            last_tick: Instant::now(),
            gpg_errored: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.style().as_ref() != &self.style {
            ctx.set_style(self.style.clone());
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.input(|key| {
                if key.modifiers.ctrl && (key.key_pressed(egui::Key::Plus) || key.raw_scroll_delta[1] > 0.) {
                    self.ui_scale *= 1.1;
                }
                if key.modifiers.ctrl && (key.key_pressed(egui::Key::Minus) || key.raw_scroll_delta[1] < 0.) {
                    self.ui_scale *= 0.9;
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

            if self.last_tick.elapsed() >= self.interval {
                self.last_tick = Instant::now();
                Storage::write(self);
                ctx.request_repaint_after(self.interval);
            }


            egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.page, Pages::Certs, "Certs");
                    ui.selectable_value(&mut self.page, Pages::NewCert, "New Cert");
                    // ui.selectable_value(&mut self.page, Pages::Sign, "Sign/Verify");
                    ui.selectable_value(&mut self.page, Pages::Style, "Style");
                    ui.selectable_value(&mut self.page, Pages::Debug, "Debug");
                    ui.selectable_value(&mut self.page, Pages::About, "About");
                    // if ui.button("Save").clicked() {
                    //     let immutable_self: &MyApp = &self;
                    //     self.storage.write(immutable_self);
                    // }
                });
            });

            #[cfg(target_arch = "wasm32")]
            {
                let warning_window = egui::containers::Window::new("WARNING!!!");
                if self.show_warning {
                    warning_window.show(ctx, |ui| {
                        ui.label("This is the web version, and as such, is not 100% garenteed to be totally secure, due to the fact that the crypto libaries I am using do not suport wasm  fully, and as such, I recommend that you download or compile the native version. As of right now, the only feature I was forced to disable is the constant time crypto. If you're on firefox, opning devtools will cause a lot of lag for some reason, this is a bug with firefox.");
                        if ui.button("Dismiss").clicked() {
                            self.show_warning = false;
                        }
                    });
                }
            }

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
                        // self.sign(ui);
                        ui.label("WIP")
                    });
                }
                Pages::About => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        Self::about(ui);
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

    pub fn handle_certs(&mut self, cert: Cert, rev: Signature, ui: &mut Ui) {
        let cert = try_or_return!(self, ui, cert.insert_packets(vec![Packet::from(rev)])).0;

        let armored: Vec<u8> = try_or_return!(self, ui, cert.armored().to_vec());

        self.cert_status.cert_text = try_or_return!(self, ui, String::from_utf8(armored));

        match CertParser::from_reader(self.cert_status.cert_text.as_bytes())
            .map_err(|e| e.to_string())
        {
            Ok(cert) => {
                for cert in cert {
                    self.certs.push(match cert {
                        Ok(cert) => cert,
                        Err(err) => {
                            self.err = err.to_string();
                            log::error!("{err}");
                            break;
                        }
                    });
                }
            }
            Err(err) => {
                self.err.clone_from(&err);
                log::error!("{err}");
            }
        }

        self.cert_status.secret_text =
            String::from_utf8(try_or_return!(self, ui, cert.as_tsk().armored().to_vec()))
                .unwrap_or_default();

        match CertParser::from_reader(self.cert_status.secret_text.as_bytes())
            .map_err(|e| e.to_string())
        {
            Ok(cert) => {
                for cert in cert {
                    self.priv_certs.push(match cert {
                        Ok(cert) => cert,
                        Err(err) => {
                            self.err = err.to_string();
                            log::error!("{err}");
                            break;
                        }
                    });
                }
            }
            Err(err) => {
                self.err.clone_from(&err);
                log::error!("{err}");
            }
        }
    }

    pub fn display_certs(&mut self, ui: &mut Ui) {
        let cert_text = self.cert_status.cert_text.clone();
        let secret_text = self.cert_status.secret_text.clone();
        egui::containers::Window::new("Certs").vscroll(true).show(ui.ctx(), |ui| {
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.label("MAKE SURE TO WRITE THESE DOWN, THEY WILL NOT BE SHOWN AGAIN! Revocation certifacte is embedded in the private cert.\n");
                ui.label(egui::RichText::new(format!("Certificate: \n{cert_text}")).font(egui::FontId::new(12., egui::FontFamily::Monospace)));
                ui.label(egui::RichText::new(format!("Private Key: \n{secret_text}")).font(egui::FontId::new(12., egui::FontFamily::Monospace)));
                ui.horizontal(|ui| {
                    if ui.button("Dismiss").clicked() {
                        self.cert_status.show_window = false;
                    }
                    egui::containers::ComboBox::from_label("Download Format").selected_text(format!("{:?}", self.cert_status.bin_or_ask)).show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.cert_status.bin_or_ask, new_cert_status::BinOrAsc::Bin, "Binary");
                        ui.selectable_value(&mut self.cert_status.bin_or_ask, new_cert_status::BinOrAsc::Asc, "ASK");
                    });
                    if ui.button("Download").clicked() {
                        if self.cert_status.bin_or_ask == new_cert_status::BinOrAsc::Bin {
                            let cert_obj = try_or_return!(self, ui, Self::str_to_cert_obj(&self.cert_status.cert_text.clone()));
                            let bin_dat = try_or_return!(self, ui, self.cert_obj_to_bin(ui, &cert_obj));

                            try_or_return!(self, ui, platform::write_file("PublicKey", &bin_dat));

                            let cert_obj = try_or_return!(self, ui, Self::str_to_cert_obj(&self.cert_status.secret_text.clone()));
                            let bin_dat = try_or_return!(self, ui, self.cert_obj_to_bin(ui, &cert_obj));

                            try_or_return!(self, ui, platform::write_file("SecretKey", &bin_dat));
                        } else {
                            try_or_return!(self, ui, platform::write_file("PublicKey.asc", &self.cert_status.cert_text.as_bytes().to_vec()));
                            try_or_return!(self, ui, platform::write_file("SecretKey.asc", &self.cert_status.secret_text.as_bytes().to_vec()));
                        }
                    }
                });
            });
        });
    }
}
