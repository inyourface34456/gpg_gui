use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize, Default, Clone)]
pub enum Pages {
    #[default]
    Certs,
    NewCert,
    Style,
    Sign,
    Debug,
    About,
}
