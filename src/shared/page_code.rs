use crate::MyApp;
use crate::custom_widgets::AddUserids;
use crate::custom_widgets::ExpireTimeSelector;
use crate::custom_widgets::MultiSelect;
use crate::custom_widgets::PasswordViewer;
use crate::selectable_values;
use crate::shared::new_cert_status;
use egui::Ui;
use new_cert_status::CipherSuite;

impl MyApp {
    pub fn debug(&mut self, ui: &mut Ui) {
        if ui.button("Trigger Error").clicked() {
            self.err = String::from("This is an error");
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

        assert!(
            !ui.button("Test Panic").clicked(),
            "test panic button clicked"
        );
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

    #[allow(clippy::too_many_lines)]
    pub fn new_cert(&mut self, ui: &mut Ui) {
        ui.heading("New Certificate");
        ui.separator();

        #[rustfmt::skip]
        ui.checkbox(&mut self.cert_status.diff_algos, "Diffrent algorithm for signing and encrypting");

        if self.cert_status.diff_algos {
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
        } else {
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

        if self.cert_status.indvidual_expire {
            for i in &mut self.cert_status.desired_subkeys {
                ui.add(ExpireTimeSelector::new(
                    &format!("Expire Time for {i} Subkey"),
                    i.as_mut(),
                ));
            }
            ui.add(ExpireTimeSelector::new(
                "Expire Time for Primary Cert",
                &mut self.cert_status.expire_date,
            ));
        } else {
            ui.add(ExpireTimeSelector::new(
                "Expire Time",
                &mut self.cert_status.expire_date,
            ));

            for i in &mut self.cert_status.desired_subkeys {
                i.set_expire(self.cert_status.expire_date);
            }
        }

        ui.add_space(5.);

        ui.add(PasswordViewer::new(
            "a",
            &mut self.cert_status.password,
            &mut self.cert_status.password2,
            &mut self.cert_status.password_vis,
            &self.cert_status.email,
            &self.cert_status.comment,
            &self.cert_status.display_name,
        ));

        if !self.cert_status.display_name.is_empty()
            && !self.cert_status.password.is_empty()
            && self.cert_status.password == self.cert_status.password2
            // important that this is the last line, otherwise the button will show up all of the time
            && ui.button("Generate Certificate").clicked()
        {
            let result = self.cert_status.generate_certs();

            if let Some(result) = result {
                match result {
                    Ok((cert, rev)) => {
                        self.handle_certs(cert, rev, ui);
                    }
                    Err(err) => {
                        self.err = err.to_string();
                        log::error!("{err}");
                        self.display_error(ui.ctx(), file!(), line!());
                    }
                }
            }
        }

        // Immediate mode: redraw the window every frame it should be visible,
        // gated only by `show_window` (not tied to the click event).
        if self.cert_status.show_window {
            self.display_certs(ui);
        }
    }

    // pub fn sign(&mut self, _ui: &mut Ui) {}

    pub fn about(ui: &mut Ui) {
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
