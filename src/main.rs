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
struct MyEguiApp {
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
            plot(ui);
        });
    }
}

fn plot(ui: &mut egui::Ui) {
    ui.painter().rect_filled(
        egui::Rect::from_min_max(
            egui::Pos2::new(50.0, 50.0),
            egui::Pos2::new(150.0, 150.0)
        ),
        egui::Rounding::same(20.0),
        egui::Color32::RED,
    );

    ui.painter().rect_stroke(
        egui::Rect::from_min_max(
            egui::Pos2::new(100.0, 100.0),
            egui::Pos2::new(200.0, 200.0),
        ),
        egui::Rounding::none(),
        egui::Stroke::new(10.0, egui::Color32::GREEN),
    );
}
