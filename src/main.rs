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
enum MyItem {
    First,
    Second,
    Third,
}

struct MyEguiApp {
    pub value: MyItem,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self { value: MyItem::First }
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
                .size(28.0)
                .color(egui::Color32::from_rgba_premultiplied(255, 0, 0, 100))
                .italics();
            let label = egui::Label::new(label_text);
            ui.add(label);

            ui.separator();

            ui.horizontal(|ui| {
                let label_1 = egui::RichText::new("First").size(24.0);
                if ui.add(egui::SelectableLabel::new(self.value == MyItem::First, label_1))
                    .clicked() {
                        self.value = MyItem::First;
                }

                let label_2 = egui::RichText::new("Second").size(24.0);
                if ui.add(egui::SelectableLabel::new(self.value == MyItem::Second, label_2))
                    .clicked() {
                        self.value = MyItem::Second;
                }

                let label_3 = egui::RichText::new("Third").size(24.0);
                if ui.add(egui::SelectableLabel::new(self.value == MyItem::Third, label_3))
                    .clicked() {
                        self.value = MyItem::Third;
                }
            })
        });
    }
}
