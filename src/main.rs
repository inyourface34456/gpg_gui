use eframe::egui;
// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen::prelude::wasm_bindgen;

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

#[derive(Default)]
struct MyApp {
    name: String,
    count: u32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let pixels_per_point = ctx.pixels_per_point();
        ctx.set_pixels_per_point(pixels_per_point);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, egui!");

            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            if ui.button("Click me!").clicked() {
                self.count += 1;
            }

            if !self.name.is_empty() {
                ui.label(format!("Hello, {}! 👋", self.name));
            }

            if self.count > 0 {
                ui.label(format!("Button clicked {} time(s)", self.count));
            }
        });
    }
}