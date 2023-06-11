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

struct MyEguiApp {
    click_pos: Vec<egui::Pos2>,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            click_pos: vec![],
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
            let resp = ui.allocate_response(egui::vec2(400.0, 300.0), egui::Sense::click());
            if resp.clicked() {
                let p = resp.interact_pointer_pos().unwrap();
                self.click_pos.push(p);
            }

            plot(ui, &self.click_pos)
        });
    }
}

fn plot(ui: &mut egui::Ui, pos: &Vec<egui::Pos2>) {
    for p in pos {
        ui.painter().circle_filled(*p, 25.0,
        egui::Color32::from_rgba_premultiplied(255, 0, 0, 100))
    }
}
