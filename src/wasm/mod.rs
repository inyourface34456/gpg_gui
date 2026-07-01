use crate::MyApp;
use eframe::egui::Ui;
use sequoia_openpgp::Cert;
use sequoia_openpgp::cert::CertParser;
use sequoia_openpgp::parse::Parse;

pub fn get_certs(armoured: &str) -> Result<Vec<Cert>, String> {
    let mut certs = vec![];
    for cert in CertParser::from_reader(armoured.as_bytes()).map_err(|e| e.to_string())? {
        match cert {
            Ok(cert) => certs.push(cert),
            Err(e) => log::error!("Skipping malformed cert: {}", e),
        }
    }
    Ok(certs)
}

impl MyApp {
    #[cfg(target_arch = "wasm32")]
    pub fn get_and_display_certs(&mut self, ui: &mut Ui) {
        ui.label("GPG armoured output: ");
        egui::ScrollArea::vertical()
            .max_height(200.0)
            .min_scrolled_height(200.0)
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.gpg_armoured)
                        .desired_width(500.)
                        .desired_rows(1), // start small, grows up to max_height
                );
            });

        if ui.button("Enter").clicked() {
            if let Ok(certs) = get_certs(&self.gpg_armoured) {
                self.certs = certs;
                self.err = String::new();
            } else {
                self.err = "corrupted gpg --export -a output".to_owned();
            }
        }
    }
}

pub fn init_logging() {
    eframe::WebLogger::init(log::LevelFilter::Trace).ok();
    console_error_panic_hook::set_once();
}
