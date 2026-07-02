use crate::MyApp;
use egui::Ui;
use postcard::{from_bytes, to_allocvec as to_vec};
use sequoia_openpgp::Cert;
use sequoia_openpgp::cert::CertParser;
use sequoia_openpgp::parse::Parse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::{
    fs,
    io::{Read, Write},
    os::unix::fs::MetadataExt,
    process::Command,
};
use zeroize::Zeroize;

/// Interface must be identical between wasm and native.

pub fn get_certs(_: &str) -> Result<Vec<Cert>, String> {
    let mut command = Command::new("gpg");
    command.arg("--export").arg("-a");
    let output = command.output().map_err(|e| e.to_string())?;
    let armored_output = String::from_utf8_lossy(&output.stdout);
    let mut certs = vec![];
    for cert in CertParser::from_reader(armored_output.as_bytes()).map_err(|e| e.to_string())? {
        match cert {
            Ok(cert) => certs.push(cert),
            Err(e) => log::error!("Skipping malformed cert: {}", e),
        }
    }
    Ok(certs)
}

impl MyApp {
    pub fn get_and_display_certs(&mut self, ui: &mut Ui) {
        let certs = match get_certs("") {
            Ok(certs) => certs,
            Err(err) => {
                self.err = err.to_string();
                log::error!("{}@{}: {}", file!(), line!(), err.to_string());
                self.display_error(ui.ctx());
                return;
            }
        };
        self.certs = certs;
    }
}

pub fn init_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
}

#[derive(Serialize, Deserialize)]
pub struct Storage {
    storage: HashMap<String, String>,
}

impl Storage {
    const FILENAME: &'static str = "data";

    // yes none of these cannot technicly fail, but i want parody between native and wasm code
    pub fn set_item(&mut self, key: &str, value: &str) -> Result<(), String> {
        match self.storage.insert(key.into(), value.into()) {
            Some(_) => Ok(()),
            None => Ok(()),
        }
    }

    pub fn get_item(&mut self, key: &str) -> Result<Option<String>, String> {
        match self.storage.get(&key.to_string()) {
            Some(item) => Ok(Some(item.clone())),
            None => Ok(None),
        }
    }

    pub fn remove_item(&mut self, key: &str) -> Result<(), String> {
        match self.storage.remove(&key.to_string()) {
            Some(_) => Ok(()),
            None => Err(String::from("item does not exist")),
        }
    }

    // layout: MyApp (no need to re init storage struct)
    pub fn write(&self, data: &MyApp) {
        let data: Vec<u8> = match to_vec(&data) {
            Ok(data) => data,
            Err(err) => {
                log::error!("{}@{}: {}", file!(), line!(), err.to_string());
                return;
            }
        };

        let mut file = match File::create(Self::FILENAME) {
            Ok(file) => file,
            Err(err) => {
                log::error!("{}@{}: {}", file!(), line!(), err.to_string());
                return;
            }
        };
        match file.write(&data) {
            Ok(_) => {}
            Err(err) => {
                log::error!("{}@{}: {}", file!(), line!(), err.to_string());
                return;
            }
        };
    }

    pub fn read() -> Option<MyApp> {
        let mut file_handle = match File::open(Self::FILENAME) {
            Ok(file) => file,
            Err(err) => {
                log::error!("{}@{}: {}", file!(), line!(), err.to_string());
                return None;
            }
        };

        let file_metadata = match fs::metadata(Self::FILENAME) {
            Ok(data) => data,
            Err(err) => {
                log::error!("{}@{}: {}", file!(), line!(), err.to_string());
                return None;
            }
        };

        let mut data: Vec<u8> = Vec::with_capacity(file_metadata.size() as usize);
        match file_handle.read_to_end(&mut data) {
            Ok(bytes) => {
                log::error!("read in {} bytes", bytes);
            }
            Err(err) => {
                log::error!("{}@{}: {}", file!(), line!(), err.to_string());
                return None;
            }
        };

        let myapp: MyApp = match from_bytes(&data) {
            Ok(map) => map,
            Err(err) => {
                log::error!("{}@{}: {}", file!(), line!(), err.to_string());
                return None;
            }
        };

        log::info!("style: {:?}", myapp.style);

        Some(myapp)
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
}
