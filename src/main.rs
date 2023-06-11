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
    let pos_1 = egui::Pos2::new(50.0, 50.0);
    let pos_2 = egui::Pos2::new(200.0, 200.0);

    let stroke_1 = egui::Stroke::new(5.0, egui::Color32::RED);
    let stroke_2 = egui::Stroke::new(5.0, egui::Color32::GREEN);

    ui.painter().vline(50.0, std::ops::RangeInclusive::new(50.0, 200.0), stroke_1);
    ui.painter().hline(std::ops::RangeInclusive::new(50.0, 200.0), 50.0, stroke_1);
    ui.painter().line_segment([pos_1, pos_2], stroke_2)
}
