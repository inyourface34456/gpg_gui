use sequoia_openpgp::cert::CipherSuite;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum BinOrAsc {
    Bin,
    Asc,
}

#[derive(Serialize, Deserialize)]
pub struct CertStatus {
    #[serde(skip_serializing, skip_deserializing)]
    pub crypto_algo: CipherSuite,
    pub display_name: String,
    pub comment: String,
    pub email: String,
    pub expire_date: u64,
    pub never_expires: bool,
    pub password: String,
    pub show_window: bool,
    pub cert_text: String,
    pub secret_text: String,
    pub bin_or_ask: BinOrAsc,
}

impl Default for CertStatus {
    fn default() -> Self {
        Self {
            // key_flags,
            crypto_algo: CipherSuite::Cv25519,
            display_name: String::new(),
            comment: String::new(),
            email: String::new(),
            expire_date: 0,
            never_expires: false,
            password: String::new(),
            show_window: false,
            cert_text: String::new(),
            secret_text: String::new(),
            bin_or_ask: BinOrAsc::Asc,
        }
    }
}
