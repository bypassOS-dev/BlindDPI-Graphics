use eframe::egui;

fn main() {
    let option = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "BlindDPI", 
        option, 
        Box::new(|_cc| Ok(Box::new(MyApp::default())))
    ).unwrap();
}

struct MyApp{
    is_running: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { is_running: false}
    }
}
impl eframe::App for MyApp{
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("BlindDPI"). size(30.0));

                ui.add_space(15.0);

                let (status_text, status_color) = if self.is_running {
                    ("BlindDPI is running", egui::Color32::GREEN)
                } else {
                    ("BlindDPI is stopped", egui::Color32::RED)
                };
                
                ui.label(
                    egui::RichText::new(status_text)
                    .size(20.0)
                    .color(status_color)
                );

                ui.add_space(5.0);

                let (btn_text, btn_bg_color) = if self.is_running {
                    ("Stop", egui::Color32::from_rgb(180, 50, 50))
                } else {
                    ("Start", egui::Color32::from_rgb(40, 140, 60)) 
                };
                
                let rich_btn_text = egui::RichText::new(btn_text)
                    .size(22.0)
                    .color(egui::Color32::WHITE);
                let button = egui::Button::new(rich_btn_text).fill(btn_bg_color);

                if ui.add_sized([200.0, 50.0], button).clicked() {
                    self.is_running = !self.is_running;
                }
            });
        });
    }
}