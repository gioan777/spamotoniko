use eframe::App;
mod spamotoniko;
use spamotoniko::spamotoniko::update_spamotonics;
use spamotoniko::spamotoniko::SpamotonicSystem;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    //let mut line = String::new();
    //let _ = std::io::stdin().read_line(&mut line).unwrap();

    //println!("{}", katharevousopoisis(&line, &vowels_map));

    /*
      let native_options = eframe::NativeOptions{
          viewport: egui::ViewportBuilder::default()
             .with_inner_size([400.0, 300.0])
             .with_min_inner_size([300.0, 220.0])
             //.with_icon()
             ,
          ..Default::default()
      };

      eframe::run_native(
          "eframe template",
          native_options,
          Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc))),
      )
    */

    //let app = SpamotonicSystem {vowels_map};
    let native_options = eframe::NativeOptions::default();
    //Text::run(Settings::default())
    let _ = eframe::run_native(
        "Σπαμοτονικό Σύστημα",
        native_options,
        Box::new(|_| Ok(Box::new(SpamotonicSystem::new()))),
    );
}
#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Ok(Box::new(SpamotonicSystem::new(cc)))),
            )
            .await
            .expect("failed to start eframe");
    });
}

impl App for SpamotonicSystem {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        //let mut text=katharevousopoisis(input, &self.vowels_map);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Σπαμοτονικό Σύστημα");
            let response = ui.add(
                egui::TextEdit::multiline(&mut self.input)
                    .font(egui::TextStyle::Monospace)
                    .desired_rows(10)
                    .desired_width(f32::INFINITY),
            );
            ui.heading(&self.text);
            if response.changed() {
                //text = katharevousopoisis(input, &self.vowels_map).clone();
                update_spamotonics(&mut self.text, &mut self.input, &self.vowels_map);
                ctx.request_repaint();
            }
        });
    }
}
