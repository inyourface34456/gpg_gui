use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Pages {
    #[default]
    Certs,
    NewCert,
    Style,
    Sign,
    Debug,
    About,
}
