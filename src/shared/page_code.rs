use crate::MyApp;
use egui::Ui;
use sequoia_openpgp::Packet;
use sequoia_openpgp::armor::{Kind, Writer};
use sequoia_openpgp::cert::{CertBuilder, CipherSuite};
use sequoia_openpgp::serialize::Marshal;
use sequoia_openpgp::serialize::SerializeInto;
use sequoia_openpgp::types::KeyFlags;
use zeroize::Zeroize;

impl MyApp {
    pub fn debug(&mut self, ui: &mut Ui) {
        if ui.button("Trigger Error").clicked() {
            self.err = String::from("This is an error")
        }

        self.display_error(ui.ctx());

        if ui.button("Test Error").clicked() {
            log::error!("Test Error");
        }

        if ui.button("Test Warn").clicked() {
            log::warn!("Test Warn");
        }

        if ui.button("Test Info").clicked() {
            log::info!("Test Info");
        }

        if ui.button("Test Debug").clicked() {
            log::debug!("Test Debug");
        }

        if ui.button("Test Trace").clicked() {
            log::trace!("Test Trace");
        }

        if ui.button("Test Panic").clicked() {
            panic!("Test")
        }
    }

    pub fn see_certs(&mut self, ui: &mut Ui) {
        self.get_and_display_certs(ui);

        for i in &self.certs {
            ui.label(format!(
                "User Id: {}",
                match i
                    .userids()
                    .map(|cert| String::from_utf8_lossy(cert.userid().value()).to_string())
                    .next()
                {
                    Some(e) => e.to_string(),
                    None => String::from("No names in export"),
                }
            ));
        }
        if !self.err.is_empty() {
            ui.label(&self.err);
        }
    }

    pub fn new_cert(&mut self, ui: &mut Ui) {
        ui.heading("New Certifacate");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Primary Crypto Algorithm");
            egui::ComboBox::from_label(" ")
                .selected_text(format!("{:?}", self.cert_status.crypto_algo))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.cert_status.crypto_algo,
                        CipherSuite::Cv25519,
                        "Cv25519",
                    );
                    ui.selectable_value(
                        &mut self.cert_status.crypto_algo,
                        CipherSuite::Cv448,
                        "Cv448",
                    );
                    ui.selectable_value(
                        &mut self.cert_status.crypto_algo,
                        CipherSuite::P256,
                        "NistP256",
                    );
                    ui.selectable_value(
                        &mut self.cert_status.crypto_algo,
                        CipherSuite::P384,
                        "NistP384",
                    );
                    ui.selectable_value(
                        &mut self.cert_status.crypto_algo,
                        CipherSuite::P521,
                        "NistP521",
                    );
                    ui.selectable_value(
                        &mut self.cert_status.crypto_algo,
                        CipherSuite::RSA2k,
                        "RSA2k",
                    );
                    ui.selectable_value(
                        &mut self.cert_status.crypto_algo,
                        CipherSuite::RSA3k,
                        "RSA3k",
                    );
                    ui.selectable_value(
                        &mut self.cert_status.crypto_algo,
                        CipherSuite::RSA4k,
                        "RSA4k",
                    );
                });
        });

        ui.horizontal(|ui| {
            ui.label("Display Name*: ");
            ui.text_edit_singleline(&mut self.cert_status.display_name);
        });

        ui.horizontal(|ui| {
            ui.label("Comment (optional): ");
            ui.text_edit_singleline(&mut self.cert_status.comment);
        });

        ui.horizontal(|ui| {
            ui.label("Email (optional): ");
            ui.text_edit_singleline(&mut self.cert_status.email);
        });

        let user_id;
        if !self.cert_status.display_name.is_empty() {
            if self.cert_status.comment.is_empty() && !self.cert_status.email.is_empty() {
                user_id = format!(
                    "{} <{}>",
                    self.cert_status.display_name, self.cert_status.email
                );
            } else if !self.cert_status.comment.is_empty() && self.cert_status.email.is_empty() {
                user_id = format!(
                    "{} ({})",
                    self.cert_status.display_name, self.cert_status.comment
                );
            } else if !self.cert_status.comment.is_empty() && !self.cert_status.email.is_empty() {
                user_id = format!(
                    "{} ({}) <{}>",
                    self.cert_status.display_name, self.cert_status.comment, self.cert_status.email
                );
            } else {
                user_id = self.cert_status.display_name.clone();
            }
        } else {
            user_id = String::new()
        }

        ui.label(format!("User ID: {}", user_id));

        let mut temp = self.cert_status.expire_date.to_string();

        ui.add_space(1.);

        ui.horizontal(|ui| {
            ui.label("Expire date (in seconds): ");
            if !self.cert_status.never_expires {
                ui.text_edit_singleline(&mut temp);
            }
            ui.checkbox(&mut self.cert_status.never_expires, "Never Expire");
        });

        self.cert_status.expire_date = match temp.parse() {
            Ok(num) => num,
            Err(err) => {
                self.err = err.to_string();
                log::error!("{}", err);
                self.display_error(ui.ctx());
                self.cert_status.expire_date
            }
        };

        ui.horizontal(|ui| {
            ui.label("Passord: ");
            ui.add(
                egui::TextEdit::singleline(&mut self.cert_status.password)
                    .password(true)
                    .hint_text("Password"),
            );
        });

        if !self.cert_status.display_name.is_empty() && !self.cert_status.password.is_empty() {
            let mut result = None;
            if ui.button("Generate Certifcate").clicked() {
                if self.cert_status.never_expires {
                    result = Some(
                        CertBuilder::new()
                            .add_userid(self.cert_status.display_name.clone())
                            .set_cipher_suite(self.cert_status.crypto_algo)
                            .add_signing_subkey()
                            .add_subkey(KeyFlags::empty().set_transport_encryption(), None, None)
                            .set_password(Some(self.cert_status.password.clone().into())) // optional: encrypt the secret keys
                            .generate(),
                    );
                } else {
                    result = Some(
                        CertBuilder::new()
                            .add_userid(self.cert_status.display_name.clone())
                            .set_cipher_suite(self.cert_status.crypto_algo)
                            .set_validity_period(std::time::Duration::from_secs(
                                self.cert_status.expire_date,
                            ))
                            .add_signing_subkey()
                            .add_subkey(KeyFlags::empty().set_transport_encryption(), None, None)
                            .set_password(Some(self.cert_status.password.clone().into())) // optional: encrypt the secret keys
                            .generate(),
                    );
                }
                self.cert_status.password.zeroize();
                self.cert_status.show_window = true;
            }

            match result {
                Some(result) => {
                    match result {
                        Ok((cert, rev)) => {
                            let armored: Vec<u8> = match cert.armored().to_vec() {
                                Ok(cert) => cert,
                                Err(err) => {
                                    self.err = err.to_string();
                                    log::error!("{}", err);
                                    self.display_error(ui.ctx());
                                    vec![]
                                }
                            };
                            self.cert_status.cert_text = match String::from_utf8(armored) {
                                Ok(output) => output,
                                Err(err) => {
                                    self.err = err.to_string();
                                    log::error!("{}", err);
                                    self.display_error(ui.ctx());
                                    String::new()
                                }
                            };
                            self.cert_status.secret_text = match cert.as_tsk().armored().to_vec() {
                                Ok(bytes) => String::from_utf8(bytes).unwrap_or_default(),
                                Err(err) => {
                                    self.err = err.to_string();
                                    log::error!("{}", err);
                                    self.display_error(ui.ctx());
                                    String::new()
                                }
                            };
                            let mut buf = Vec::new();
                            let mut w = match Writer::with_headers(
                                &mut buf,
                                Kind::PublicKey, // GPG uses PUBLIC KEY BLOCK for rev certs
                                vec![("Comment", "Revocation certificate")],
                            ) {
                                Ok(output) => output,
                                Err(err) => {
                                    self.err = err.to_string();
                                    log::error!("{}", err);
                                    self.display_error(ui.ctx());
                                    return;
                                }
                            };
                            let _ = Packet::from(rev).serialize(&mut w); // Signature → Packet, then serialize
                            let _ = w.finalize();
                            self.cert_status.rev_text = String::from_utf8(buf).unwrap_or_default();
                        }
                        Err(err) => {
                            self.err = err.to_string();
                            log::error!("{}", err);
                            self.display_error(ui.ctx());
                        }
                    }
                }
                None => {}
            }
        }

        // Immediate mode: redraw the window every frame it should be visible,
        // gated only by `show_window` (not tied to the click event).
        if self.cert_status.show_window {
            let cert_text = self.cert_status.cert_text.clone();
            let rev_text = self.cert_status.rev_text.clone();
            let secret_text = self.cert_status.secret_text.clone();
            egui::containers::Window::new("Certs")
                .vscroll(true)
                .show(ui.ctx(), |ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        ui.label("MAKE SURE TO WRITE THESE DOWN, THEY WILL NOT BE SHOWN AGAIN!");
                        ui.code(format!("Certificate: \n{}", cert_text));
                        ui.code(format!("Revocation Certificate: \n{}", rev_text));
                        ui.code(format!("Private Key: \n{}", secret_text));
                        if ui.button("Dismiss").clicked() {
                            self.cert_status.show_window = false;
                        }
                    });
                });
        }
    }
}
