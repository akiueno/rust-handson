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

#[derive(PartialEq, Debug)]
enum RadioValue {
    First,
    Second,
    Third,
}

struct MyEguiApp {
    pub value: RadioValue,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            value: RadioValue::First,
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

            let msg = format!("Checked = {:?}.", self.value);
            let label_text = egui::RichText::new(msg)
                .size(32.0)
                .color(egui::Color32::from_rgba_premultiplied(255, 0, 0, 100))
                .italics();
            let label = egui::Label::new(label_text);
            ui.add(label);

            ui.separator();

            let label_1 = egui::RichText::new("First").size(24.0);
            let label_2 = egui::RichText::new("Second").size(24.0);
            let label_3 = egui::RichText::new("Third").size(24.0);

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.value, RadioValue::First, label_1);
                ui.radio_value(&mut self.value, RadioValue::Second, label_2);
                ui.radio_value(&mut self.value, RadioValue::Third, label_3);
            });
        });
    }
}
