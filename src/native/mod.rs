use crate::MyApp;
use egui::Ui;
use postcard::{from_bytes, to_allocvec as to_vec};
use sequoia_openpgp::Cert;
use sequoia_openpgp::cert::CertParser;
use sequoia_openpgp::parse::Parse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
#[cfg(target_os = "linux")]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;
use std::{
    fs,
    io::{Read, Write},
    process::Command,
};

// Interface must be identical between wasm and native.

pub fn get_certs(_: &str, _: &str) -> Result<(Vec<Cert>, Vec<Cert>), String> {
    log::info!("getting certs");
    let mut command = Command::new("gpg");
    command.arg("--export").arg("-a");
    let output = command.output().map_err(|e| e.to_string())?;
    let armored_output = String::from_utf8_lossy(&output.stdout);
    let possible_error = String::from_utf8_lossy(&output.stderr);
    match command.status() {
        Ok(exit_code) => {
            if !exit_code.success() {
                log::error!("Gpg exited unsucsessfully: {possible_error}");
            }
        }
        Err(err) => log::error!("Error checking status: {err}")
    }
    let mut certs = vec![];
    for cert in CertParser::from_reader(armored_output.as_bytes()).map_err(|e| e.to_string())? {
        match cert {
            Ok(cert) => certs.push(cert),
            Err(e) => log::error!("Skipping malformed cert: {e}"),
        }
    }

    let mut command = Command::new("gpg");
    command.arg("--export-secret-keys").arg("-a");
    let output = command.output().map_err(|e| e.to_string())?;
    let armored_output = String::from_utf8_lossy(&output.stdout);
    let possible_error = String::from_utf8_lossy(&output.stderr);
    match command.status() {
        Ok(exit_code) => {
            if !exit_code.success() {
                log::error!("Gpg exited unsucsessfully: {possible_error}");
            }
        }
        Err(err) => log::error!("Error checking status: {err}")
    }
    let mut priv_certs = vec![];
    for cert in CertParser::from_reader(armored_output.as_bytes()).map_err(|e| e.to_string())? {
        match cert {
            Ok(cert) => priv_certs.push(cert),
            Err(e) => log::error!("Skipping malformed cert: {e}"),
        }
    }

    Ok((certs, priv_certs))
}

impl MyApp {
    pub fn get_and_display_certs(&mut self, ui: &mut Ui) {
        if !self.certs.is_empty() || !self.priv_certs.is_empty() {
            return;
        }

        let certs = match get_certs("", "") {
            Ok(certs) => certs,
            Err(err) => {
                self.err.clone_from(&err);
                log::error!("{}@{}: {err}", file!(), line!());
                self.display_error(ui.ctx(), file!(), line!());
                self
                return;
            }
        };
        self.certs = certs.0;
        self.priv_certs = certs.1;
    }
}

pub fn init_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Storage {
    storage: HashMap<String, String>,
}

impl Storage {
    const FILENAME: &'static str = "data";

    pub fn write(data: &MyApp) {
        let data: Vec<u8> = match to_vec(&data) {
            Ok(data) => data,
            Err(err) => {
                log::error!("{}@{}: {err}", file!(), line!());
                return;
            }
        };

        let mut file = match File::create(Self::FILENAME) {
            Ok(file) => file,
            Err(err) => {
                log::error!("{}@{}: {err}", file!(), line!());
                return;
            }
        };
        match file.write_all(&data) {
            Ok(()) => {}
            Err(err) => {
                log::error!("{}@{}: {err}", file!(), line!());
            }
        }
    }

    pub fn read() -> Option<MyApp> {
        let mut file_handle = match File::open(Self::FILENAME) {
            Ok(file) => file,
            Err(err) => {
                log::error!("{}@{}: {err}", file!(), line!());
                return None;
            }
        };

        let file_metadata = match fs::metadata(Self::FILENAME) {
            Ok(data) => data,
            Err(err) => {
                log::error!("{}@{}: {err}", file!(), line!());
                return None;
            }
        };

        #[cfg(target_os = "windows")]
        #[allow(clippy::cast_possible_truncation)]
        let mut data: Vec<u8> = Vec::with_capacity(file_metadata.file_size() as usize);
        #[cfg(target_os = "linux")]
        // i very much doubt that this file will get bigger then 4gb on 32 bit targets
        #[allow(clippy::cast_possible_truncation)]
        let mut data: Vec<u8> = Vec::with_capacity(file_metadata.size() as usize);
        match file_handle.read_to_end(&mut data) {
            Ok(bytes) => {
                log::info!("read in {bytes} bytes");
            }
            Err(err) => {
                log::error!("{}@{}: {err}", file!(), line!());
                return None;
            }
        }

        let myapp: MyApp = match from_bytes(&data) {
            Ok(map) => map,
            Err(err) => {
                log::error!("{}@{}: {err}", file!(), line!());
                return None;
            }
        };

        log::info!("style: {:?}", myapp.style);

        Some(myapp)
    }
}

pub fn write_file(filename: &str, data: &Vec<u8>) -> Result<(), String> {
    let mut file = fs::File::create(filename).map_err(|e| e.to_string())?;
    file.write(data.as_slice()).map_err(|err| err.to_string())?;
    Ok(())
}
