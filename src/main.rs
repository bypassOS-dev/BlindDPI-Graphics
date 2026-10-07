use eframe::egui;

fn main() {
    let option = eframe::NativeOptions::default();

    eframe::run_native(
        "BlindDPI", 
        option, 
        Box::new(|_cc| Ok(Box::new(MyApp::default())))
    ).unwrap();
}


struct MyApp{
    is_running: bool,
    status_text: String
}

impl Default for MyApp {
    fn default() -> Self {
        Self { is_running: false, status_text: String::from("BlindDPI isn't working") }
    }
}
impl eframe::App for MyApp{
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("BlindDPI");
            if self.is_running {
                ui.heading("working");
            } else {
                ui.heading("don't working");
            }

            if ui.button(if self.is_running { "Stop" } else { "Run" }).clicked() {
                self.is_running = !self.is_running;
            }
        });
    }
}