use crate::MyApp;
use egui::Color32;
use egui::Ui;
use sequoia_openpgp::Cert;
use sequoia_openpgp::parse::Parse;
use sequoia_openpgp::serialize::Marshal;

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

pub fn score_info(score: u8) -> (&'static str, egui::Color32) {
    match score {
        0 => ("Very Weak", Color32::from_rgb(220, 50, 50)),
        1 => ("Weak", Color32::from_rgb(230, 126, 34)),
        2 => ("Fair", Color32::from_rgb(241, 196, 15)),
        3 => ("Good", Color32::from_rgb(46, 204, 113)),
        4 => ("Strong", Color32::from_rgb(39, 174, 96)),
        _ => unreachable!(),
    }
}
