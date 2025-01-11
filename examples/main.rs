use eframe::{Frame, NativeOptions};
use egui::{CentralPanel, Context, TextEdit, TopBottomPanel, Widget};
use egui_qr::QrCodeWidget;

fn main() {
    let _ = eframe::run_native(
        "egui_qr",
        NativeOptions::default(),
        Box::new(|_| Ok(Box::new(App::new()))),
    );
}

struct App {
    data: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            data: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("main").show(ctx, |ui| TextEdit::singleline(&mut self.data).ui(ui));
        CentralPanel::default().show(ctx, |ui| {
            if let Ok(q) = QrCodeWidget::from_data(self.data.as_bytes()) {
                q.ui(ui)
            } else {
                ui.label("Invalid data")
            }
        });
    }
}
