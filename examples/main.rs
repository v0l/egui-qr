use eframe::{Frame, NativeOptions};
use egui::{CentralPanel, Panel, TextEdit, Ui, Widget};
use egui_qr::QrCodeWidget;

fn main() {
    let _ = eframe::run_native(
        "egui-qr",
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
    fn ui(&mut self, root: &mut Ui, _frame: &mut Frame) {
        Panel::top("main").show(root, |ui| TextEdit::singleline(&mut self.data).ui(ui));
        CentralPanel::default().show(root, |ui| {
            if let Ok(q) = QrCodeWidget::from_data(self.data.as_bytes()) {
                q.ui(ui)
            } else {
                ui.label("Invalid data")
            }
        });
    }
}
