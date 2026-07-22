use sequoia_openpgp::cert::CipherSuite as Cs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Clone, Copy)]
pub enum CipherSuite {
    #[default]
    Cv25519,
    Cv448,
    RSA3k,
    P256,
    P384,
    P521,
    RSA2k,
    RSA4k,
}

impl Into<Cs> for CipherSuite {
    fn into(self) -> Cs {
        match self {
            Self::Cv25519 => Cs::Cv25519,
            Self::Cv448 => Cs::Cv448,
            Self::RSA3k => Cs::RSA3k,
            Self::P256 => Cs::P256,
            Self::P384 => Cs::P384,
            Self::P521 => Cs::P521,
            Self::RSA2k => Cs::RSA2k,
            Self::RSA4k => Cs::RSA4k,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum BinOrAsc {
    Bin,
    Asc,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Subkeys {
    Authentcation(Option<ExpireTime>),
    TransportEncryption(Option<ExpireTime>),
    Signing(Option<ExpireTime>),
    StorageEncryption(Option<ExpireTime>),
}

impl Subkeys {
    pub fn set_expire(&mut self, expire: Option<ExpireTime>) {
        *self = match self {
            Self::Authentcation(_) => Self::Authentcation(expire),
            Self::Signing(_) => Self::Signing(expire),
            Self::StorageEncryption(_) => Self::StorageEncryption(expire),
            Self::TransportEncryption(_) => Self::TransportEncryption(expire),
        };
    }
}

impl std::fmt::Display for Subkeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authentcation(_) => write!(f, "Authentcation"),
            Self::Signing(_) => write!(f, "Signing"),
            Self::StorageEncryption(_) => write!(f, "Storage Encryption"),
            Self::TransportEncryption(_) => write!(f, "Transport Encryption"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Copy, Clone)]
pub enum ExpireTime {
    OneHour,
    SixHour,
    OneDay,
    FiveDays,
    OneWeek,
    TwoWeeks,
    OneMonth,
    TwoMonths,
    SixMonths,
    OneYear,
    TwoYears,
    FiveYears,
    Custom(u64),
}

impl ExpireTime {
    const FIVE_DAYS: u64 = 60 * 60 * 24 * 5;
    const FIVE_YEARS: u64 = 60 * 60 * 24 * 365 * 5;
    const ONE_DAY: u64 = 60 * 60 * 24;
    const ONE_HOUR: u64 = 60 * 60;
    const ONE_MONTH: u64 = 60 * 60 * 24 * 27;
    const ONE_WEEK: u64 = 60 * 60 * 24 * 7;
    const ONE_YEAR: u64 = 60 * 60 * 24 * 365;
    const SIX_HOUR: u64 = 60 * 60 * 6;
    const SIX_MONTHS: u64 = 60 * 60 * 24 * 27 * 6;
    const TWO_MONTHS: u64 = 60 * 60 * 24 * 27 * 2;
    const TWO_WEEKS: u64 = 60 * 60 * 24 * 7 * 2;
    const TWO_YEARS: u64 = 60 * 60 * 24 * 365 * 2;

    pub fn to_string(&self) -> String {
        format!("{}", Into::<u64>::into(*self))
    }
}

impl From<u64> for ExpireTime {
    fn from(value: u64) -> Self {
        match value {
            Self::FIVE_DAYS => Self::FiveDays,
            Self::FIVE_YEARS => Self::FiveYears,
            Self::ONE_DAY => Self::OneDay,
            Self::ONE_HOUR => Self::OneHour,
            Self::ONE_MONTH => Self::OneMonth,
            Self::ONE_WEEK => Self::OneWeek,
            Self::ONE_YEAR => Self::OneYear,
            Self::SIX_HOUR => Self::SixHour,
            Self::SIX_MONTHS => Self::SixMonths,
            Self::TWO_MONTHS => Self::TwoMonths,
            Self::TWO_WEEKS => Self::TwoWeeks,
            Self::TWO_YEARS => Self::TwoYears,
            _ => Self::Custom(value),
        }
    }
}

impl Into<u64> for ExpireTime {
    fn into(self) -> u64 {
        match self {
            Self::Custom(t) => t,
            Self::FiveDays => 60 * 60 * 24 * 5,
            Self::FiveYears => 60 * 60 * 24 * 365 * 5,
            Self::OneDay => 60 * 60 * 24,
            Self::OneHour => 60 * 60,
            Self::OneMonth => 60 * 60 * 24 * 27,
            Self::OneWeek => 60 * 60 * 24 * 7,
            Self::OneYear => 60 * 60 * 24 * 365,
            Self::SixHour => 60 * 60 * 6,
            Self::SixMonths => 60 * 60 * 24 * 27 * 6,
            Self::TwoMonths => 60 * 60 * 24 * 27 * 2,
            Self::TwoWeeks => 60 * 60 * 24 * 7 * 2,
            Self::TwoYears => 60 * 60 * 24 * 365 * 2,
        }
    }
}

impl Subkeys {
    pub const ALL_SUBKEYS: &[Subkeys] = &[
        Subkeys::Authentcation(Some(ExpireTime::OneDay)),
        Subkeys::Signing(Some(ExpireTime::OneDay)),
        Subkeys::StorageEncryption(Some(ExpireTime::OneDay)),
        Subkeys::TransportEncryption(Some(ExpireTime::OneDay)),
    ];
}

impl Into<std::time::Duration> for ExpireTime {
    fn into(self) -> std::time::Duration {
        std::time::Duration::from_secs(self.into())
    }
}

#[derive(Serialize, Deserialize)]
pub struct CertStatus {
    pub crypto_algo: CipherSuite,
    /// 0 is encrypt 1 is sign
    pub encrypt_sign: (CipherSuite, CipherSuite),
    pub display_name: String,
    pub comment: String,
    pub email: String,
    pub expire_date: Option<ExpireTime>,
    #[serde(skip_serializing, skip_deserializing)]
    pub password: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub password2: String,
    pub show_window: bool,
    pub cert_text: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub secret_text: String,
    pub bin_or_ask: BinOrAsc,
    pub userid: Vec<String>,
    pub editing_userid: usize,
    pub password_vis: (bool, bool),
    pub desired_subkeys: Vec<Subkeys>,
    pub diff_algos: bool,
}

impl Default for CertStatus {
    fn default() -> Self {
        Self {
            crypto_algo: CipherSuite::Cv25519,
            display_name: String::new(),
            comment: String::new(),
            email: String::new(),
            expire_date: Some(ExpireTime::OneDay),
            password: String::new(),
            password2: String::new(),
            show_window: false,
            cert_text: String::new(),
            secret_text: String::new(),
            bin_or_ask: BinOrAsc::Asc,
            userid: vec![String::new()],
            editing_userid: 0,
            password_vis: (false, false),
            desired_subkeys: vec![
                Subkeys::Authentcation(Some(ExpireTime::OneDay)),
                Subkeys::Signing(Some(ExpireTime::OneDay)),
                Subkeys::TransportEncryption(Some(ExpireTime::OneDay)),
            ],
            diff_algos: false,
            encrypt_sign: (CipherSuite::Cv25519, CipherSuite::Cv25519),
        }
    }
}
