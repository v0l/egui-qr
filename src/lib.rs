use egui::{vec2, Color32, Rect, Response, Rounding, Sense, Stroke, Ui, Widget};
use qrcode::{Color, QrCode};

pub struct QrCodeWidget<'a> {
    code: &'a QrCode,
}

impl<'a> QrCodeWidget<'a> {
    pub fn new(code: &'a QrCode) -> Self {
        QrCodeWidget { code }
    }
}

impl Widget for QrCodeWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let size = ui.available_size();

        let (response, painter) = ui.allocate_painter(size, Sense::click());
        let w = self.code.width();
        let start = response.rect.min;
        let mut ctr = 0;
        let scale = (size.x / w as f32).floor();
        for c in self.code.to_colors() {
            let row = ctr / w;
            let col = ctr % w;
            let c_start = start + vec2(col as f32 * scale, row as f32 * scale);
            let c_end = c_start + vec2(scale, scale);
            painter.rect(
                Rect::from_min_max(c_start, c_end),
                Rounding::none(),
                match c {
                    Color::Light => Color32::WHITE,
                    Color::Dark => Color32::BLACK,
                },
                Stroke::NONE,
            );
            ctr += 1;
        }
        response
    }
}
