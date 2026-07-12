use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum Pages {
    Certs,
    NewCert,
    Style,
    Sign,
    Debug,
}

impl Default for Pages {
    fn default() -> Self {
        Self::Certs
    }
}
