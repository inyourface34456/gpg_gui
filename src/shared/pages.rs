#[derive(PartialEq, Eq)]
pub enum Pages {
    Certs,
    NewCert,
    Style
}

impl Default for Pages {
    fn default() -> Self {
        Self::Certs
    }
}