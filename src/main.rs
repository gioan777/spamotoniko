#[cfg(target_arch = "wasm32")]
use eframe::web_sys;
use eframe::App;
mod spamotoniko;
use spamotoniko::spamotoniko::update_spamotonics;
use spamotoniko::spamotoniko::SpamotonicSystem;

fn main() {
    use_eframe();
}
#[cfg(not(target_arch = "wasm32"))]
fn use_eframe() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Σπαμοτονικό Σύστημα",
        native_options,
        Box::new(|_| Ok(Box::new(SpamotonicSystem::new()))),
    );
}
#[cfg(target_arch = "wasm32")]
fn use_eframe() {
    use eframe::wasm_bindgen::JsCast as _;
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
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
        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_| Ok(Box::new(SpamotonicSystem::new()))),
            )
            .await
            .expect("failed to start eframe");
    });
}

impl App for SpamotonicSystem {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        //let mut text=katharevousopoisis(input, &self.vowels_map);
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Σπαμοτονικό Σύστημα");
            let old_string = self.input.clone();
            let response = ui.add(
                egui::TextEdit::multiline(&mut self.input)
                    .font(egui::TextStyle::Monospace)
                    .desired_rows(10)
                    .desired_width(f32::INFINITY),
            );
            ui.heading(&self.text);
            if response.changed() && old_string != *self.input {
                //text = katharevousopoisis(input, &self.vowels_map).clone();
                update_spamotonics(&mut self.text, &self.input, &self.vowels_map);
                ui.request_repaint();
            }
        });
    }
}
