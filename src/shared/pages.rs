#[derive(PartialEq, Eq)]
pub enum Pages {
    Certs,
    NewCert
}

impl Default for Pages {
    fn default() -> Self {
        Self::NewCert
    }
}