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
            ui.heading(egui::RichText::new("BlindDPI"). size(30.0));

            ui.add_space(15.0);

            if self.is_running {
                ui.label(egui::RichText::new("working").size(20.0));
            } else {
                ui.label(egui::RichText::new("don't working").size(20.0));
            }

            ui.add_space(5.0);

            let btn_text = if self.is_running {"stop"} else {"start"};
            let rich_btn_text = egui::RichText::new(btn_text).size(22.0);

            let button = egui::Button::new(rich_btn_text);

            if ui.add_sized([240.0, 60.0], button).clicked() {
                self.is_running = !self.is_running;
            }
        });
    }
}