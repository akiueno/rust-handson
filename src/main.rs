fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.default_theme = eframe::Theme::Light;
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            let label_text = egui::RichText::new("This is sample message.")
                .size(32.0)
                .color(egui::Color32::from_rgba_premultiplied(255, 0, 0, 100))
                .italics();
            let label = egui::Label::new(label_text);
            ui.add(label);
        });
    }
}
