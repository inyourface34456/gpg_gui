use sequoia_openpgp::cert::CipherSuite;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum BinOrAsc {
    Bin,
    Asc,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Subkeys {
    Authentcation,
    TransportEncryption,
    Certification,
    Signing,
    StorageEncryption,
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
        Subkeys::Authentcation,
        Subkeys::Certification,
        Subkeys::Signing,
        Subkeys::StorageEncryption,
        Subkeys::TransportEncryption,
    ];
}

#[derive(Serialize, Deserialize)]
pub struct CertStatus {
    #[serde(skip_serializing, skip_deserializing)]
    pub crypto_algo: CipherSuite,
    pub display_name: String,
    pub comment: String,
    pub email: String,
    pub expire_date: Option<ExpireTime>,
    pub password: String,
    pub password2: String,
    pub show_window: bool,
    pub cert_text: String,
    pub secret_text: String,
    pub bin_or_ask: BinOrAsc,
    pub userid: Vec<String>,
    pub editing_userid: usize,
    pub password_vis: (bool, bool),
    pub desired_subkeys: Vec<Subkeys>,
}

impl Default for CertStatus {
    fn default() -> Self {
        Self {
            // key_flags,
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
                Subkeys::Authentcation,
                Subkeys::Signing,
                Subkeys::TransportEncryption,
            ],
        }
    }
}

impl CertStatus {
    pub fn to_string(&self) -> String {
        match self.expire_date {
            Some(t) => <ExpireTime as Into<u64>>::into(t).to_string(),
            None => String::from("0"),
        }
    }
}
