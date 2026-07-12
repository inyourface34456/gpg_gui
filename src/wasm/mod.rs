use crate::MyApp;
use base64::Engine;
use eframe::egui::Ui;
use postcard::{from_bytes, to_allocvec as to_vec};
use sequoia_openpgp::Cert;
use sequoia_openpgp::cert::CertParser;
use sequoia_openpgp::parse::Parse;
use serde::{Deserialize, Serialize};
use web_sys::window;

/// Interface must be identical between wasm and native.

pub fn get_certs(armoured: &str, priv_key: &str) -> Result<(Vec<Cert>, Vec<Cert>), String> {
    let mut certs = vec![];
    for cert in CertParser::from_reader(armoured.as_bytes()).map_err(|e| e.to_string())? {
        match cert {
            Ok(cert) => certs.push(cert),
            Err(e) => log::error!("Skipping malformed cert: {}", e),
        }
    }

    let mut priv_keys = vec![];
    for cert in CertParser::from_reader(priv_key.as_bytes()).map_err(|e| e.to_string())? {
        match cert {
            Ok(cert) => priv_keys.push(cert),
            Err(e) => log::error!("Skipping malformed cert: {}", e),
        }
    }

    Ok((certs, priv_keys))
}

impl MyApp {
    pub fn get_and_display_certs(&mut self, ui: &mut Ui) {
        ui.label("GPG armoured output: ");
        egui::ScrollArea::vertical()
            .max_height(200.0)
            .min_scrolled_height(200.0)
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.gpg_armoured)
                        .desired_width(500.)
                        .desired_rows(1), // start small, grows up to max_height
                );
            });

        ui.label("GPG armoured output: ");
        egui::ScrollArea::vertical()
            .max_height(200.0)
            .min_scrolled_height(200.0)
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.gpg_armoured_priv)
                        .desired_width(500.)
                        .desired_rows(1), // start small, grows up to max_height
                );
            });

        if !self.gpg_armoured.is_empty() {
            match get_certs(&self.gpg_armoured, &self.gpg_armoured_priv) {
                Ok((public, private)) => {
                    self.certs = public;
                    self.priv_certs = private;
                    self.err = String::new();
                }
                Err(err) => {
                    self.err = err;
                    log::error!("{}", self.err);
                }
            }
        }
    }
}

pub fn init_logging() {
    eframe::WebLogger::init(log::LevelFilter::Trace).ok();
    console_error_panic_hook::set_once();
}

#[derive(Serialize, Deserialize)]
pub struct Storage;

impl Storage {
    // pub fn set_item(&mut self, key: &str, value: &str) -> Result<(), String> {
    //     let window = window().ok_or("no global `window` exists")?;
    //     let storage = window
    //         .local_storage()
    //         .map_err(|err| {
    //             if err.is_string() {
    //                 format!("{}", err.as_string().unwrap())
    //             } else {
    //                 format!("{:?}", err)
    //             }
    //         })?
    //         .ok_or("no localStorage available")?;
    //     storage.set_item(key, value).map_err(|err| {
    //         if err.is_string() {
    //             format!("{}", err.as_string().unwrap())
    //         } else {
    //             format!("{:?}", err)
    //         }
    //     })?;
    //     Ok(())
    // }

    pub fn get_item(&mut self, key: &str) -> Result<Option<String>, String> {
        let window = window().ok_or("no global `window` exists")?;
        let storage = window
            .local_storage()
            .map_err(|err| {
                if err.is_string() {
                    format!("{}", err.as_string().unwrap())
                } else {
                    format!("{:?}", err)
                }
            })?
            .ok_or("no localStorage available")?;
        storage.get_item(key).map_err(|err| {
            if err.is_string() {
                format!("{}", err.as_string().unwrap())
            } else {
                format!("{:?}", err)
            }
        })
    }

    // pub fn remove_item(&mut self, key: &str) -> Result<(), String> {
    //     let window = window().ok_or("no global `window` exists")?;
    //     let storage = window
    //         .local_storage()
    //         .map_err(|err| {
    //             if err.is_string() {
    //                 format!("{}", err.as_string().unwrap())
    //             } else {
    //                 format!("{:?}", err)
    //             }
    //         })?
    //         .ok_or("no localStorage available")?;
    //     storage.remove_item(key).map_err(|err| {
    //         if err.is_string() {
    //             format!("{}", err.as_string().unwrap())
    //         } else {
    //             format!("{:?}", err)
    //         }
    //     })?;
    //     Ok(())
    // }

    pub fn write(&self, data: &MyApp) {
        let myapp = match to_vec(&data) {
            Ok(data) => data,
            Err(err) => {
                log::error!("{}@{}: {}", file!(), line!(), err.to_string());
                return;
            }
        };
        let engine = base64::engine::GeneralPurpose::new(
            &base64::alphabet::URL_SAFE,
            base64::engine::GeneralPurposeConfig::new(),
        );
        let data = engine.encode(myapp);

        fn set_item(key: &str, value: &str) -> Result<(), String> {
            let window = window().ok_or("no global `window` exists")?;
            let storage = window
                .local_storage()
                .map_err(|err| {
                    if err.is_string() {
                        format!("{}", err.as_string().unwrap())
                    } else {
                        format!("{:?}", err)
                    }
                })?
                .ok_or("no localStorage available")?;
            storage.set_item(key, value).map_err(|err| {
                if err.is_string() {
                    format!("{}", err.as_string().unwrap())
                } else {
                    format!("{:?}", err)
                }
            })?;
            Ok(())
        }

        match set_item("MyApp", &data) {
            Ok(_) => {}
            Err(err) => {
                log::error!("{}@{}: {}", file!(), line!(), err.to_string());
                return;
            }
        };
    }

    pub fn read() -> Option<MyApp> {
        let mut storage = Storage::default();
        let data = match storage.get_item("MyApp") {
            Ok(data) => data.unwrap_or_default(),
            Err(err) => {
                log::error!("{}@{}: {}", file!(), line!(), err.to_string());
                return None;
            }
        };

        let engine = base64::engine::GeneralPurpose::new(
            &base64::alphabet::URL_SAFE,
            base64::engine::GeneralPurposeConfig::new(),
        );
        let data = match engine.decode(data) {
            Ok(data) => data,
            Err(err) => {
                log::error!("{}@{}: {}", file!(), line!(), err.to_string());
                return None;
            }
        };

        let myapp: MyApp = match from_bytes(&data) {
            Ok(data) => data,
            Err(err) => {
                log::error!("{}@{}: {}", file!(), line!(), err.to_string());
                return None;
            }
        };

        Some(myapp)
    }
}

impl Default for Storage {
    fn default() -> Self {
        Storage
    }
}
