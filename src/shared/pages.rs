#[derive(PartialEq, Eq)]
pub enum Pages {
    Certs,
    NewCert,
    Style,
    Debug,
}

impl Default for Pages {
    fn default() -> Self {
        Self::Certs
    }
}
