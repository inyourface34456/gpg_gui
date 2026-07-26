use crate::custom_widgets::add_userids::AddUserids;
use crate::custom_widgets::expire_time_selector::ExpireTimeSelector;
use crate::custom_widgets::multi_select::MultiSelect;
use crate::shared::helpers;
use crate::shared::new_cert_status;
use crate::{MyApp, platform};
use crate::{selectable_values, try_or_return};
use egui::Ui;
use new_cert_status::{CipherSuite, Subkeys};
use sequoia_openpgp::Packet;
use sequoia_openpgp::cert::{CertBuilder, CertParser, CipherSuite as Cs};
use sequoia_openpgp::parse::Parse;
use sequoia_openpgp::serialize::SerializeInto;
use sequoia_openpgp::types::KeyFlags;
use zeroize::Zeroize;
use zxcvbn::zxcvbn;

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
                i.userids()
                    .map(|cert| String::from_utf8_lossy(cert.userid().value()).to_string())
                    .next()
                    .unwrap_or(String::from("No names in Export"))
            ));
        }

        ui.add_space(10.);
        ui.label("Private Keys");
        for i in &self.priv_certs {
            ui.label(format!(
                "User Id: {}",
                i.userids()
                    .map(|cert| String::from_utf8_lossy(cert.userid().value()).to_string())
                    .next()
                    .unwrap_or(String::from("No names in Export"))
            ));
        }
        if !self.err.is_empty() {
            ui.label(&self.err);
        }
    }

    pub fn new_cert(&mut self, ui: &mut Ui) {
        ui.heading("New Certificate");
        ui.separator();

        #[rustfmt::skip]
        ui.checkbox(&mut self.cert_status.diff_algos, "Diffrent algorithm for signing and encrypting");

        if !self.cert_status.diff_algos {
            ui.horizontal(|ui| {
                ui.label("Genreal Algorithm");
                egui::ComboBox::from_label(" ")
                    .selected_text(format!("{:?}", self.cert_status.encrypt_sign.0))
                    .show_ui(ui, |ui| {
                        selectable_values!(
                            ui,
                            &mut self.cert_status.encrypt_sign,
                            (CipherSuite::Cv25519, CipherSuite::Cv25519) => "Cv25519",
                            (CipherSuite::Cv448, CipherSuite::Cv448)     => "Cv448",
                            (CipherSuite::P256, CipherSuite::P256)       => "NistP256",
                            (CipherSuite::P384, CipherSuite::P384)       => "NistP384",
                            (CipherSuite::P521, CipherSuite::P521)       => "NistP521",
                            (CipherSuite::RSA2k, CipherSuite::RSA2k)     => "RSA2k",
                            (CipherSuite::RSA3k, CipherSuite::RSA3k)     => "RSA3k",
                            (CipherSuite::RSA4k, CipherSuite::RSA4k)     => "RSA4k",
                        );
                    });
            });
        } else {
            ui.horizontal(|ui| {
                ui.label("Ecryption Algorithm");
                egui::ComboBox::from_label(" ")
                    .selected_text(format!("{:?}", self.cert_status.encrypt_sign.0))
                    .show_ui(ui, |ui| {
                        selectable_values!(
                            ui,
                            &mut self.cert_status.encrypt_sign.0,
                            CipherSuite::Cv25519 => "Cv25519",
                            CipherSuite::Cv448   => "Cv448",
                            CipherSuite::P256    => "NistP256",
                            CipherSuite::P384    => "NistP384",
                            CipherSuite::P521    => "NistP521",
                            CipherSuite::RSA2k   => "RSA2k",
                            CipherSuite::RSA3k   => "RSA3k",
                            CipherSuite::RSA4k   => "RSA4k",
                        );
                    });
            });
            ui.horizontal(|ui| {
                ui.label("Signing Algorithm");
                egui::ComboBox::from_label("  ")
                    .selected_text(format!("{:?}", self.cert_status.encrypt_sign.1))
                    .show_ui(ui, |ui| {
                        selectable_values!(
                            ui,
                            &mut self.cert_status.encrypt_sign.1,
                            CipherSuite::Cv25519 => "Cv25519",
                            CipherSuite::Cv448   => "Cv448",
                            CipherSuite::P256    => "NistP256",
                            CipherSuite::P384    => "NistP384",
                            CipherSuite::P521    => "NistP521",
                            CipherSuite::RSA2k   => "RSA2k",
                            CipherSuite::RSA3k   => "RSA3k",
                            CipherSuite::RSA4k   => "RSA4k",
                        );
                    });
            });
        }

        ui.add_space(10.);

        ui.add(MultiSelect::new(
            "a",
            new_cert_status::Subkeys::ALL_SUBKEYS,
            &mut self.cert_status.desired_subkeys,
        ));

        ui.add_space(15.);

        ui.add(AddUserids::new(
            " ",
            &mut self.cert_status.editing_userid,
            &mut self.cert_status.display_name,
            &mut self.cert_status.comment,
            &mut self.cert_status.email,
            &mut self.cert_status.userid,
        ));

        ui.add_space(5.);

        ui.checkbox(
            &mut self.cert_status.indvidual_expire,
            "Set expiration for indvidual subkeys",
        );

        if !self.cert_status.indvidual_expire {
            ui.add(ExpireTimeSelector::new(
                "Expire Time",
                &mut self.cert_status.expire_date,
            ));

            for i in self.cert_status.desired_subkeys.iter_mut() {
                i.set_expire(self.cert_status.expire_date);
            }
        } else {
            for i in self.cert_status.desired_subkeys.iter_mut() {
                ui.add(ExpireTimeSelector::new(
                    &format!("Expire Time for {} Subkey", i),
                    i.get_mut_ref(),
                ));
            }
            ui.add(ExpireTimeSelector::new(
                "Expire Time for Primary Cert",
                &mut self.cert_status.expire_date,
            ));
        }

        ui.add_space(5.);

        ui.horizontal(|ui| {
            ui.label("Password*: ");
            ui.add(
                egui::TextEdit::singleline(&mut self.cert_status.password)
                    .password(!self.cert_status.password_vis.0)
                    .hint_text("Password"),
            );
            ui.checkbox(&mut self.cert_status.password_vis.0, "Show Password");
        });

        ui.horizontal(|ui| {
            ui.label("Confirm Password*: ");
            ui.add(
                egui::TextEdit::singleline(&mut self.cert_status.password2)
                    .password(!self.cert_status.password_vis.1)
                    .hint_text("Password"),
            );
            ui.checkbox(&mut self.cert_status.password_vis.1, "Show Password");
        });

        let score = match zxcvbn(
            &self.cert_status.password,
            &[
                &self.cert_status.comment,
                &self.cert_status.email,
                &self.cert_status.display_name,
            ],
        ) {
            Ok(score) => score,
            Err(_) => zxcvbn("a", &[]).expect("No idea how this can fail"),
        };
        let (label, color) = helpers::score_info(score.score());

        ui.horizontal(|ui| {
            ui.label("Password Strength");
            let bar = egui::ProgressBar::new(score.score() as u8 as f32 / 4.)
                .show_percentage()
                .fill(color)
                .desired_width(200.);
            ui.add(bar);
            ui.colored_label(color, label);
        });

        if let Some(feedback) = score.feedback() {
            if let Some(warning) = feedback.warning() {
                ui.label(format!("Warning: {}", warning));
            }
            for sugestion in feedback.suggestions() {
                ui.label(format!("Suggestion: {}", sugestion));
            }
        }

        if !self.cert_status.password2.is_empty()
            && self.cert_status.password != self.cert_status.password2
        {
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
                if self.cert_status.expire_date.is_none() {
                    cert_builder = CertBuilder::new()
                } else {
                    let expire_time = match self.cert_status.expire_date {
                        Some(time) => time.into(),
                        None => unreachable!(),
                    };
                    cert_builder = CertBuilder::new()
                        .set_validity_period(std::time::Duration::from_secs(expire_time))
                }

                cert_builder =
                    cert_builder.set_password(Some(self.cert_status.password.clone().into()));

                for i in self.cert_status.userid.iter() {
                    cert_builder = cert_builder.add_userid(i.clone())
                }

                let (sign, encrypt): (Cs, Cs) = (
                    self.cert_status.encrypt_sign.0.into(),
                    self.cert_status.encrypt_sign.1.into(),
                );
                cert_builder = cert_builder.set_cipher_suite(sign);

                for subkey_type in self.cert_status.desired_subkeys.iter() {
                    cert_builder = match subkey_type {
                        Subkeys::Authentcation(v) => cert_builder.add_subkey(
                            KeyFlags::empty().set_authentication(),
                            v.map(Into::into),
                            Some(sign),
                        ),
                        Subkeys::Signing(v) => cert_builder.add_subkey(
                            KeyFlags::empty().set_signing(),
                            v.map(Into::into),
                            Some(sign),
                        ),
                        Subkeys::StorageEncryption(v) => cert_builder.add_subkey(
                            KeyFlags::empty().set_storage_encryption(),
                            v.map(Into::into),
                            Some(encrypt),
                        ),
                        Subkeys::TransportEncryption(v) => cert_builder.add_subkey(
                            KeyFlags::empty().set_transport_encryption(),
                            v.map(Into::into),
                            Some(encrypt),
                        ),
                    };
                }

                result = Some(cert_builder.generate());
                self.cert_status.password.zeroize();
                self.cert_status.password2.zeroize();
                self.cert_status.show_window = true;
            }

            match result {
                Some(result) => match result {
                    Ok((cert, rev)) => {
                        let cert =
                            try_or_return!(self, ui, cert.insert_packets(vec![Packet::from(rev)]))
                                .0;

                        let armored: Vec<u8> = try_or_return!(self, ui, cert.armored().to_vec());

                        self.cert_status.cert_text =
                            try_or_return!(self, ui, String::from_utf8(armored));

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
                            }
                        }

                        self.cert_status.secret_text = String::from_utf8(try_or_return!(
                            self,
                            ui,
                            cert.as_tsk().armored().to_vec()
                        ))
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
                                            log::error!("{}", err);
                                            break;
                                        }
                                    });
                                }
                            }
                            Err(err) => {
                                self.err = err.to_string();
                                log::error!("{}", err);
                            }
                        }
                    }
                    Err(err) => {
                        self.err = err.to_string();
                        log::error!("{}", err);
                        self.display_error(ui.ctx(), file!(), line!());
                    }
                },
                None => {}
            }
        }

        // Immediate mode: redraw the window every frame it should be visible,
        // gated only by `show_window` (not tied to the click event).
        if self.cert_status.show_window {
            let cert_text = self.cert_status.cert_text.clone();
            let secret_text = self.cert_status.secret_text.clone();
            egui::containers::Window::new("Certs").vscroll(true).show(ui.ctx(), |ui| {
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    ui.label("MAKE SURE TO WRITE THESE DOWN, THEY WILL NOT BE SHOWN AGAIN! Revocation certifacte is embedded in the private cert.\n");
                    ui.label(egui::RichText::new(format!("Certificate: \n{}", cert_text)).font(egui::FontId::new(12., egui::FontFamily::Monospace)));
                    ui.label(egui::RichText::new(format!("Private Key: \n{}", secret_text)).font(egui::FontId::new(12., egui::FontFamily::Monospace)));
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
                                let cert_obj = try_or_return!(self, ui, self.str_to_cert_obj(&self.cert_status.cert_text.clone()));
                                let bin_dat = try_or_return!(self, ui, self.cert_obj_to_bin(ui, cert_obj));

                                try_or_return!(self, ui, platform::write_file("PublicKey", bin_dat));

                                let cert_obj = try_or_return!(self, ui, self.str_to_cert_obj(&self.cert_status.secret_text.clone()));
                                let bin_dat = try_or_return!(self, ui, self.cert_obj_to_bin(ui, cert_obj));

                                try_or_return!(self, ui, platform::write_file("SecretKey", bin_dat));
                            } else {
                                try_or_return!(self, ui, platform::write_file("PublicKey.asc", self.cert_status.cert_text.as_bytes().to_vec()));
                                try_or_return!(self, ui, platform::write_file("SecretKey.asc", self.cert_status.secret_text.as_bytes().to_vec()));
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
        ui.label(format!("Target Os: {}", std::env::consts::OS));
        if ui.link("Github Repo").clicked() {
            ui.ctx().open_url(egui::OpenUrl::new_tab(
                "https://github.com/inyourface34456/gpg_gui",
            ));
        }
    }
}
