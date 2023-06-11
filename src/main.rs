fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.default_theme = eframe::Theme::Light;
    native_options.initial_window_size = Some(egui::Vec2 { x: 400.0, y: 300.0 });
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
            plot(ui);
        });
    }
}

fn plot(ui: &mut egui::Ui) {
    ui.painter().text(
        egui::Pos2 { x: 50.0, y: 50.0 },
        egui::Align2::LEFT_CENTER,
        "Hello!",
        egui::FontId::proportional(24.0),
        egui::Color32::RED,
    );
    ui.painter().text(
        egui::Pos2 { x: 50.0, y: 100.0 },
        egui::Align2::LEFT_CENTER,
        "Sample Message.",
        egui::FontId::proportional(36.0),
        egui::Color32::BLUE,
    );
}
