use crate::custom_widgets::multi_select::MultiSelect;
use crate::shared::helpers::user_id_to_componets;
use crate::shared::new_cert_status;
use crate::{MyApp, platform};
use egui::Ui;
use sequoia_openpgp::Packet;
use sequoia_openpgp::cert::{CertBuilder, CertParser, CipherSuite};
use sequoia_openpgp::parse::Parse;
use sequoia_openpgp::serialize::SerializeInto;
use zeroize::Zeroize;

impl MyApp {
    pub fn debug(&mut self, ui: &mut Ui) {
        if ui.button("Trigger Error").clicked() {
            self.err = String::from("This is an error")
        }

        self.display_error(ui.ctx(), file!(), line!());

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

        ui.label("Public Keys:");
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

        ui.add_space(10.);
        ui.label("Private Keys");
        for i in &self.priv_certs {
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
        ui.heading("New Certificate");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Encryption Algorithm");
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

        ui.add_space(10.);

        ui.add(MultiSelect::new(
            "a",
            new_cert_status::Subkeys::ALL_SUBKEYS,
            &mut self.cert_status.desired_subkeys,
        ));

        ui.add_space(15.);

        egui::containers::ComboBox::from_label("")
            .selected_text(format!(
                "UserID #{} ({})",
                self.cert_status.editing_userid + 1,
                self.cert_status.userid[self.cert_status.editing_userid]
            ))
            .show_ui(ui, |ui| {
                for (index, value) in self.cert_status.userid.iter().enumerate() {
                    if ui
                        .selectable_value(
                            &mut self.cert_status.editing_userid,
                            index,
                            format!("UserID #{} ({})", index + 1, value),
                        )
                        .clicked()
                    {
                        let userid_parts = user_id_to_componets(
                            self.cert_status.userid[self.cert_status.editing_userid].clone(),
                        );
                        self.cert_status.display_name = userid_parts.0;
                        self.cert_status.comment = userid_parts.1;
                        self.cert_status.email = userid_parts.2;
                    }
                }
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

        self.cert_status.userid[self.cert_status.editing_userid] = user_id.clone();

        ui.horizontal(|ui| {
            if ui.button("Add UserID").clicked() {
                self.cert_status.userid.push(String::new());
                self.cert_status.display_name.zeroize();
                self.cert_status.email.zeroize();
                self.cert_status.comment.zeroize();
                self.cert_status.editing_userid = self.cert_status.userid.len() - 1;
            }
            if ui.button("Remove Current Userid").clicked() && self.cert_status.userid.len() > 1 {
                self.cert_status
                    .userid
                    .remove(self.cert_status.editing_userid);
                self.cert_status.editing_userid -= 1;
                let userid_parts = user_id_to_componets(
                    self.cert_status.userid[self.cert_status.editing_userid].clone(),
                );
                self.cert_status.display_name = userid_parts.0;
                self.cert_status.comment = userid_parts.1;
                self.cert_status.email = userid_parts.2;
            }
        });

        ui.add_space(10.);

        let mut temp = self.cert_status.expire_date.to_string();

        ui.add_space(1.);

        ui.horizontal(|ui| {
            ui.label("Expire date (in seconds): ");
            if !self.cert_status.never_expires {
                ui.text_edit_singleline(&mut temp);
            }
            ui.checkbox(&mut self.cert_status.never_expires, "Never Expire");
        });

        ui.add_space(5.);

        self.cert_status.expire_date = match temp.parse() {
            Ok(num) => num,
            Err(err) => {
                self.err = err.to_string();
                log::error!("{}", err);
                self.display_error(ui.ctx(), file!(), line!());
                self.cert_status.expire_date
            }
        };

        ui.add_space(5.);

        ui.horizontal(|ui| {
            ui.label("Password*: ");
            ui.add(
                egui::TextEdit::singleline(&mut self.cert_status.password)
                    .password(true)
                    .hint_text("Password"),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Confirm Password*: ");
            ui.add(
                egui::TextEdit::singleline(&mut self.cert_status.password2)
                    .password(true)
                    .hint_text("Confirm Password"),
            );
        });

        if self.cert_status.password != self.cert_status.password2 {
            ui.label(
                egui::RichText::new("Password does not match!")
                    .color(egui::Color32::from_rgb(255, 0, 0)),
            );
        }

        if !self.cert_status.display_name.is_empty()
            && !self.cert_status.password.is_empty()
            && self.cert_status.password == self.cert_status.password2
        {
            let mut result = None;
            if ui.button("Generate Certificate").clicked() {
                let mut cert_builder;
                if self.cert_status.never_expires {
                    cert_builder = CertBuilder::new()
                        .set_cipher_suite(self.cert_status.crypto_algo)
                        .set_password(Some(self.cert_status.password.clone().into())); // optional: encrypt the secret keys
                } else {
                    cert_builder = CertBuilder::new()
                        .set_cipher_suite(self.cert_status.crypto_algo)
                        .set_validity_period(std::time::Duration::from_secs(
                            self.cert_status.expire_date,
                        ))
                        .set_password(Some(self.cert_status.password.clone().into())); // optional: encrypt the secret keys
                }
                self.cert_status.password.zeroize();
                self.cert_status.password2.zeroize();
                self.cert_status.show_window = true;

                for i in self.cert_status.desired_subkeys.iter() {
                    cert_builder = match i {
                        new_cert_status::Subkeys::Authentcation => {
                            cert_builder.add_authentication_subkey()
                        }
                        new_cert_status::Subkeys::Certification => {
                            cert_builder.add_certification_subkey()
                        }
                        new_cert_status::Subkeys::Signing => cert_builder.add_signing_subkey(),
                        new_cert_status::Subkeys::StorageEncryption => {
                            cert_builder.add_storage_encryption_subkey()
                        }
                        new_cert_status::Subkeys::TransportEncryption => {
                            cert_builder.add_transport_encryption_subkey()
                        }
                    };
                }

                for i in self.cert_status.userid.iter() {
                    cert_builder = cert_builder.add_userid(i.clone())
                }

                result = Some(cert_builder.generate())
            }

            match result {
                Some(result) => {
                    match result {
                        Ok((cert, rev)) => {
                            let cert = match cert.insert_packets(vec![Packet::from(rev.clone())]) {
                                Ok(output) => output.0,
                                Err(err) => {
                                    self.err = err.to_string();
                                    log::error!("{}", err);
                                    self.display_error(ui.ctx(), file!(), line!());
                                    return;
                                }
                            };

                            let armored: Vec<u8> = match cert.armored().to_vec() {
                                Ok(cert) => cert,
                                Err(err) => {
                                    self.err = err.to_string();
                                    log::error!("{}", err);
                                    self.display_error(ui.ctx(), file!(), line!());
                                    vec![]
                                }
                            };
                            self.cert_status.cert_text = match String::from_utf8(armored) {
                                Ok(output) => output,
                                Err(err) => {
                                    self.err = err.to_string();
                                    log::error!("{}", err);
                                    self.display_error(ui.ctx(), file!(), line!());
                                    String::new()
                                }
                            };

                            match CertParser::from_reader(self.cert_status.cert_text.as_bytes())
                                .map_err(|e| e.to_string())
                            {
                                Ok(cert) => {
                                    for cert in cert {
                                        self.certs.push(match cert {
                                            Ok(cert) => cert,
                                            Err(err) => {
                                                self.err = err.to_string();
                                                log::error!("{}", err);
                                                break;
                                            }
                                        });
                                    }
                                }
                                Err(err) => {
                                    self.err = err.to_string();
                                    log::error!("{}", err);
                                    // self.display_error(ui.ctx(), file!(), line!());
                                }
                            }

                            self.cert_status.secret_text = match cert.as_tsk().armored().to_vec() {
                                Ok(bytes) => String::from_utf8(bytes).unwrap_or_default(),
                                Err(err) => {
                                    self.err = err.to_string();
                                    log::error!("{}", err);
                                    self.display_error(ui.ctx(), file!(), line!());
                                    String::new()
                                }
                            };

                            match CertParser::from_reader(self.cert_status.secret_text.as_bytes())
                                .map_err(|e| e.to_string())
                            {
                                Ok(cert) => {
                                    for cert in cert {
                                        self.priv_certs.push(match cert {
                                            Ok(cert) => cert,
                                            Err(err) => {
                                                self.err = err.to_string();
                                                log::error!("{}", err);
                                                break;
                                            }
                                        });
                                    }
                                }
                                Err(err) => {
                                    self.err = err.to_string();
                                    log::error!("{}", err);
                                    // self.display_error(ui.ctx(), file!(), line!());
                                }
                            }
                        }
                        Err(err) => {
                            self.err = err.to_string();
                            log::error!("{}", err);
                            self.display_error(ui.ctx(), file!(), line!());
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
            let secret_text = self.cert_status.secret_text.clone();
            egui::containers::Window::new("Certs")
                .vscroll(true)
                .show(ui.ctx(), |ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        ui.label("MAKE SURE TO WRITE THESE DOWN, THEY WILL NOT BE SHOWN AGAIN! Revocation certifacte is embedded in the secret key.\n");
                        ui.label(
                            egui::RichText::new(format!("Certificate: \n{}", cert_text))
                                .font(egui::FontId::new(12., egui::FontFamily::Monospace)),
                        );
                        ui.label(
                            egui::RichText::new(format!("Private Key: \n{}", secret_text))
                                .font(egui::FontId::new(12., egui::FontFamily::Monospace)),
                        );
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
                                    let cert_obj = match self.str_to_cert_obj(&self.cert_status.cert_text.clone()) {
                                        Ok(cert) => cert,
                                        Err(err) => {
                                            self.err = err.to_string();
                                            log::error!("{}", err);
                                            self.display_error(ui.ctx(), file!(), line!());
                                            return;
                                        }
                                    };
                                    let bin_dat = match self.cert_obj_to_bin(ui, cert_obj) {
                                        Ok(cert) => cert,
                                        Err(err) => {
                                            self.err = err.to_string();
                                            log::error!("{}", err);
                                            self.display_error(ui.ctx(), file!(), line!());
                                            return;
                                        }
                                    };
                                    match platform::write_file("PublicKey", bin_dat) {
                                        Ok(_) => {},
                                        Err(err) => {
                                            self.err = err.to_string();
                                            log::error!("{}", err);
                                            self.display_error(ui.ctx(), file!(), line!());
                                        }
                                    }

                                    let cert_obj = match self.str_to_cert_obj(&self.cert_status.secret_text.clone()) {
                                        Ok(cert) => cert,
                                        Err(err) => {
                                            self.err = err.to_string();
                                            log::error!("{}", err);
                                            self.display_error(ui.ctx(), file!(), line!());
                                            return;
                                        }
                                    };
                                    let bin_dat = match self.cert_obj_to_bin(ui, cert_obj) {
                                        Ok(cert) => cert,
                                        Err(err) => {
                                            self.err = err.to_string();
                                            log::error!("{}", err);
                                            self.display_error(ui.ctx(), file!(), line!());
                                            return;
                                        }
                                    };
                                    match platform::write_file("SecretKey", bin_dat) {
                                        Ok(_) => {},
                                        Err(err) => {
                                            self.err = err.to_string();
                                            log::error!("{}", err);
                                            self.display_error(ui.ctx(), file!(), line!());
                                        }
                                    }
                                } else {
                                    match platform::write_file("PublicKey.asc", self.cert_status.cert_text.as_bytes().to_vec()) {
                                        Ok(_) => {},
                                        Err(err) => {
                                            self.err = err.to_string();
                                            log::error!("{}", err);
                                            self.display_error(ui.ctx(), file!(), line!());
                                        }
                                    }
                                    match platform::write_file("SecretKey.asc", self.cert_status.secret_text.as_bytes().to_vec()) {
                                        Ok(_) => {},
                                        Err(err) => {
                                            self.err = err.to_string();
                                            log::error!("{}", err);
                                            self.display_error(ui.ctx(), file!(), line!());
                                        }
                                    }
                                }
                            }
                        });
                    });
                });
        }
    }

    pub fn sign(&mut self, _ui: &mut Ui) {
        return;
    }

    pub fn about(&mut self, ui: &mut Ui) {
        ui.label(format!(
            "Version: {} ({})",
            include_str!("../../VERSION").replace('\n', ""),
            env!("GIT_HASH")
        ));
        ui.label(format!("Target Arch: {}", std::env::consts::ARCH));
        if ui.link("Github Repo").clicked() {
            ui.ctx().open_url(egui::OpenUrl::new_tab(
                "https://github.com/inyourface34456/gpg_gui",
            ));
        }
    }
}
