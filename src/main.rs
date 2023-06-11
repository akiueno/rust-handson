fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.default_theme = eframe::Theme::Light;
    native_options.initial_window_size = Some(egui::Vec2 { x: 400.0, y: 200.0 });
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

struct MyEguiApp {
    pub message: String,
    pub content: String,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            message: String::from("Hello"),
            content: String::from("This is content."),
        }
    }
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            ui.spacing();

            let msg = format!("Title:\"{}\"\nContent:[{}]", self.message, self.content);
            let label_text = egui::RichText::new(msg)
                .size(28.0)
                .color(egui::Color32::from_rgba_premultiplied(255, 0, 0, 100))
                .italics();
            let label = egui::Label::new(label_text);
            ui.add(label);

            ui.separator();

            let te_s1 = egui::TextEdit::singleline(&mut self.message)
                .font(egui::FontId::proportional(20.0));
            ui.add(te_s1);

            let te_m1 =
                egui::TextEdit::multiline(&mut self.content).font(egui::FontId::proportional(20.0));
            ui.add(te_m1);
        });
    }
}
