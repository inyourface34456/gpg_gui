use crate::shared::CheckboxDropdown;
use sequoia_openpgp::cert::CipherSuite;

pub struct CertStatus {
    pub key_flags: CheckboxDropdown,
    pub crypto_algo: CipherSuite,
    pub display_name: String,
    pub comment: String,
    pub email: String,
}

impl Default for CertStatus {
    fn default() -> Self {
        let key_flags = CheckboxDropdown::new("Key Flags", vec!["Authentication", "Certification", "Sigining", "Transport Encryption", "Storage Encryption"]);
        
        Self {
            key_flags,
            crypto_algo: CipherSuite::Cv25519,
            display_name: String::new(),
            comment: String::new(),
            email: String::new(),
        }
    }
}