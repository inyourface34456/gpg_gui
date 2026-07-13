use crate::MyApp;
use egui::Ui;
use sequoia_openpgp::Cert;
use sequoia_openpgp::parse::Parse;
use sequoia_openpgp::serialize::Marshal;
use sequoia_openpgp::serialize::SerializeInto;

impl MyApp {
    pub fn cert_obj_to_str(&mut self, ui: &Ui, cert: Cert) -> Result<String, String> {
        let armored: Vec<u8> = match cert.armored().to_vec() {
            Ok(cert) => cert,
            Err(err) => {
                self.err = err.to_string();
                log::error!("{}", err);
                self.display_error(ui.ctx(), file!(), line!());
                return Err(err.to_string());
            }
        };
        match String::from_utf8(armored) {
            Ok(output) => Ok(output),
            Err(err) => {
                self.err = err.to_string();
                log::error!("{}", err);
                self.display_error(ui.ctx(), file!(), line!());
                Err(err.to_string())
            }
        }
    }

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
}
