#![warn(clippy::all, rust_2018_idioms)]
use shadertoy_rs::App;
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    use eframe::egui;
    use std::sync::Arc;

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "shadertoy_rs",
        native_options,
        Box::new(|cc| {
            // Add simplified Chinese font support
            #[cfg(target_os = "windows")]
            {
                // Try to load system font
                if let Ok(font_data) = std::fs::read("C:\\Windows\\Fonts\\msyh.ttc") {
                    let mut fonts = egui::FontDefinitions::default();
                    fonts.font_data.insert(
                        "msyh".to_owned(),
                        Arc::new(egui::FontData::from_owned(font_data)),
                    );

                    // Add font to all font families
                    for family in &[egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
                        if let Some(family_fonts) = fonts.families.get_mut(family) {
                            family_fonts.push("msyh".to_owned());
                        }
                    }

                    cc.egui_ctx.set_fonts(fonts);
                }
            }

            // Return application instance
            Ok(Box::new(App::new(cc)))
        }),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast;
    use std::panic;

    // Set panic hook for better error messages in the browser console
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(App::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
