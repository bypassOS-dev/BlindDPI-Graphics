fn main() {
    let option = eframe::NativeOptions::default();

    eframe::run_native(
        "BlindDPI", 
        option, 
        Box::new(|_cc| Ok(Box::new(MyApp::default())))
    );
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp{
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        
    }
}
