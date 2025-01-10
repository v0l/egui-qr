use eframe::{Frame, NativeOptions};
use egui::{CentralPanel, Context, Widget};
use egui_qr::QrCodeWidget;
use qrcode::QrCode;

fn main() {
    let _ = eframe::run_native(
        "egui_qr",
        NativeOptions::default(),
        Box::new(|_| Box::new(App)),
    );
}

struct App;

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        let code = QrCode::new(b"hello").unwrap();
        CentralPanel::default().show(ctx, |ui| {
            QrCodeWidget::new(&code).ui(ui);
        });
    }
}
