use sequoia_openpgp::cert::CertParser;
use sequoia_openpgp::parse::Parse;
use crate::MyApp;
use eframe::egui::Ui;
use sequoia_openpgp::Cert;

fn get_certs(armoured: &str) -> Result<Vec<Cert>, String> {
    let mut certs = vec![];
    for cert in CertParser::from_reader(armoured.as_bytes()).map_err(|e| e.to_string())? {
        match cert {
            Ok(cert) => certs.push(cert),
            Err(e) => eprintln!("Skipping malformed cert: {}", e),
        }
    }
    Ok(certs)
}

pub fn get_and_display_certs(self_: &mut MyApp, ui: &mut Ui) {
    ui.label("GPG armoured output: ");
    egui::ScrollArea::vertical()
        .max_height(200.0)
        .min_scrolled_height(200.0)
        .show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut self_.gpg_armoured)
                    .desired_width(500.)
                    .desired_rows(1) // start small, grows up to max_height
            );
    });
    if ui.button("Enter").clicked() {
        if let Ok(certs) = get_certs(&self_.gpg_armoured) {
            self_.certs = certs;
            self_.err = String::new();
        } else {
            self_.err = "corrupted gpg --export -a output".to_owned();
        }
    }
}