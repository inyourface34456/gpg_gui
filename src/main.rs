mod custom_widgets;
mod shared;

#[cfg(target_arch = "wasm32")]
#[path = "wasm/mod.rs"]
mod platform;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native/mod.rs"]
mod platform;

use platform::init_logging;
use shared::MyApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    init_logging();
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

    init_logging();

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
            .start(
                canvas,
                web_options,
                Box::new(|_cc| Ok(Box::new(MyApp::default()))),
            )
            .await
            .expect("failed to start eframe");
    });
}
