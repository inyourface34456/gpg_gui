use crate::MyApp;
use crate::shared::new_cert_status::Subkeys;
use egui::Color32;
use egui::Ui;
use sequoia_openpgp::Cert;
use sequoia_openpgp::Result as pgpResult;
use sequoia_openpgp::cert::CertBuilder;
use sequoia_openpgp::cert::prelude::*;
use sequoia_openpgp::packet::Signature;
use sequoia_openpgp::parse::Parse;
use sequoia_openpgp::policy::StandardPolicy;
use sequoia_openpgp::serialize::Marshal;
use sequoia_openpgp::types::KeyFlags;

impl MyApp {
    // pub fn cert_obj_to_str(&mut self, ui: &Ui, cert: Cert) -> Result<String, String> {
    //     let armored: Vec<u8> = match cert.armored().to_vec() {
    //         Ok(cert) => cert,
    //         Err(err) => {
    //             self.err = err.to_string();
    //             log::error!("{}", err);
    //             self.display_error(ui.ctx(), file!(), line!());
    //             return Err(err.to_string());
    //         }
    //     };
    //     match String::from_utf8(armored) {
    //         Ok(output) => Ok(output),
    //         Err(err) => {
    //             self.err = err.to_string();
    //             log::error!("{}", err);
    //             self.display_error(ui.ctx(), file!(), line!());
    //             Err(err.to_string())
    //         }
    //     }
    // }

    pub fn cert_obj_to_bin(&mut self, ui: &Ui, cert: Cert) -> Result<Vec<u8>, String> {
        let mut buf = Vec::new();
        match cert.serialize(&mut buf) {
            Ok(cert) => cert,
            Err(err) => {
                self.err = err.to_string();
                log::error!("{}", err);
                self.display_error(ui.ctx(), file!(), line!());
                return Err(err.to_string());
            }
        };
        Ok(buf)
    }

    pub fn str_to_cert_obj(&mut self, input: &str) -> Result<Cert, String> {
        let cert = Cert::from_bytes(input.as_bytes()).map_err(|err| err.to_string())?;
        Ok(cert)
    }

    pub fn set_signing_and_encryption_type(
        &self,
        cert_builder: CertBuilder,
    ) -> pgpResult<(Cert, Signature)> {
        let (cert, revocation) = cert_builder
            .set_cipher_suite(self.cert_status.encrypt_sign.0)
            .generate()?;

        let decrypted_cert = cert
            .primary_key()
            .key()
            .clone()
            .parts_into_secret()?
            .decrypt_secret(&self.cert_status.password.clone().into())?;

        let mut working_cert = cert.insert_packets(decrypted_cert)?.0;

        let policy = StandardPolicy::new();

        // 2. Attach one subkey per requested capability, each with the
        //    appropriate algorithm.
        for subkey in self.cert_status.desired_subkeys.iter() {
            let (flags, suite) = match subkey {
                Subkeys::Authentcation => (
                    KeyFlags::empty().set_authentication(),
                    self.cert_status.encrypt_sign.1,
                ),
                Subkeys::Signing => (
                    KeyFlags::empty().set_signing(),
                    self.cert_status.encrypt_sign.1,
                ),
                Subkeys::StorageEncryption => (
                    KeyFlags::empty().set_storage_encryption(),
                    self.cert_status.encrypt_sign.0,
                ),
                Subkeys::TransportEncryption => (
                    KeyFlags::empty().set_transport_encryption(),
                    self.cert_status.encrypt_sign.0,
                ),
            };

            let new_cert = {
                let vc = working_cert.with_policy(&policy, None)?;
                KeyBuilder::new(flags)
                    .set_cipher_suite(suite)
                    .set_password(Some(self.cert_status.password.clone().into()))
                    .subkey(vc)?
                    .attach_cert()?
            };

            working_cert = new_cert;
        }

        let re_encrypted_primary = working_cert
            .primary_key()
            .key()
            .clone()
            .parts_into_secret()?
            .encrypt_secret(&self.cert_status.password.clone().into())?;

        // I hate this, it feels like im leaking the decrypted cert,
        // but sequioa says that this should not happen
        let cert = working_cert.insert_packets(re_encrypted_primary)?.0;

        Ok((cert, revocation))
    }
}

pub fn user_id_to_componets(user_id: String) -> (String, String, String) {
    let userid_com = user_id.split(' ').collect::<Vec<&str>>();
    match userid_com.len() {
        1 => (userid_com[0].to_string(), String::new(), String::new()),
        2 => (
            userid_com[0].to_string(),
            userid_com[1].replace('(', "").replace(')', ""),
            String::new(),
        ),
        3 => (
            userid_com[0].to_string(),
            userid_com[1].replace('(', "").replace(')', ""),
            userid_com[2].replace('<', "").replace('>', ""),
        ),
        _ => (String::new(), String::new(), String::new()),
    }
}

pub fn score_info(score: zxcvbn::Score) -> (&'static str, egui::Color32) {
    match score {
        zxcvbn::Score::Zero => ("Very Weak", Color32::from_rgb(220, 50, 50)),
        zxcvbn::Score::One => ("Weak", Color32::from_rgb(230, 126, 34)),
        zxcvbn::Score::Two => ("Fair", Color32::from_rgb(241, 196, 15)),
        zxcvbn::Score::Three => ("Good", Color32::from_rgb(46, 204, 113)),
        zxcvbn::Score::Four => ("Strong", Color32::from_rgb(39, 174, 96)),
        _ => unreachable!(),
    }
}
