use std::vec;

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
    let data = vec![
        egui::Pos2::new(50.0, 100.0),
        egui::Pos2::new(250.0, 100.0),
        egui::Pos2::new(75.0, 225.0),
        egui::Pos2::new(150.0, 50.0),
        egui::Pos2::new(225.0, 225.0),
    ];
    let stroke_1 = egui::Stroke::new(5.0, egui::Color32::from_rgb(255, 0, 0));
    let mut shape_1 = eframe::epaint::PathShape::line(data, stroke_1);
    shape_1.closed = true;
    ui.painter().add(shape_1);
}
