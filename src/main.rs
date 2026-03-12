use eframe::egui;
use sequoia_openpgp::Cert;
use sequoia_openpgp::cert::CertParser;
use sequoia_openpgp::parse::Parse;
#[cfg(not(target_arch = "wasm32"))]
use std::process::Command;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hello egui",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find canvas #the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        eframe::WebRunner::new()
            .start(canvas, web_options, Box::new(|_cc| Ok(Box::new(MyApp::default()))))
            .await
            .expect("failed to start eframe");
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn get_certs() -> Vec<Cert> {
    let mut command = Command::new("gpg");
    command.arg("--export").arg("-a");
    let output = command.output().expect("failed to excaute command");
    let armored_output = String::from_utf8_lossy(&output.stdout);
    let mut certs = vec![];
    for cert in CertParser::from_reader(armored_output.as_bytes()).expect("could not parse output") {
        match cert {
            Ok(cert) => certs.push(cert),
            Err(e) => eprintln!("Skipping malformed cert: {}", e),
        }
    }
    certs
}

#[cfg(target_arch = "wasm32")]
fn get_certs(armoured: &str) -> Vec<Cert> {
    let mut certs = vec![];
    for cert in CertParser::from_reader(armoured.as_bytes()).expect("could not parse output") {
        match cert {
            Ok(cert) => certs.push(cert),
            Err(e) => eprintln!("Skipping malformed cert: {}", e),
        }
    }
    certs
}

struct MyApp {
    ui_scale: f32,
    certs: Vec<Cert>,
    #[cfg(target_arch = "wasm32")]
    gpg_armoured: String,
}

impl Default for MyApp {
    fn default() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let certs = get_certs();
        
        #[cfg(target_arch = "wasm32")]
        let certs = vec![];
        
        Self {
            ui_scale: 1.,
            certs,
            #[cfg(target_arch = "wasm32")]
            gpg_armoured: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let pixels_per_point = ctx.pixels_per_point();
        ctx.set_pixels_per_point(pixels_per_point);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.input(|key| {
               if key.key_pressed(egui::Key::Plus) && key.modifiers.ctrl {
                   self.ui_scale *= 1.1
               }
               if key.key_pressed(egui::Key::Minus) && key.modifiers.ctrl {
                   self.ui_scale *= 0.9
               } 
            });
            
            #[cfg(target_arch = "wasm32")]
            {
                ui.label("GPG armoured output: ");
                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .min_scrolled_height(200.0)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.gpg_armoured)
                                .desired_width(500.)
                                .desired_rows(1) // start small, grows up to max_height
                        );
                });
                if ui.button("Enter").clicked() {
                    self.certs = get_certs(&self.gpg_armoured);
                }
            }
            
            for i in &self.certs {
                ui.label(format!("Fingerprint: {:?}", i.fingerprint().as_bytes()));
            }
        });
    }
}