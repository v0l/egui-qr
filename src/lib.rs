use egui::{vec2, Color32, Rect, Response, Rounding, Sense, Stroke, Ui, Widget};
use qrcode::types::QrError;
use qrcode::{Color, QrCode};

enum CodeSrc<'a> {
    Ref(&'a QrCode),
    Owned(QrCode),
}

pub struct QrCodeWidget<'a> {
    code: CodeSrc<'a>,
    quiet_zone: f32,
}

impl<'a> QrCodeWidget<'a> {
    pub fn new(code: &'a QrCode) -> Self {
        QrCodeWidget {
            code: CodeSrc::Ref(code),
            quiet_zone: 1.,
        }
    }

    pub fn from_data(data: &[u8]) -> Result<Self, QrError> {
        let code = QrCode::new(data)?;
        Ok(QrCodeWidget {
            code: CodeSrc::Owned(code),
            quiet_zone: 1.,
        })
    }

    pub fn quiet_zone(mut self, quiet_zone: f32) -> Self {
        self.quiet_zone = quiet_zone;
        self
    }
}

impl Widget for QrCodeWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let outer_size = ui.available_size();

        let code_ref = match &self.code {
            CodeSrc::Ref(code) => code,
            CodeSrc::Owned(q) => q,
        };
        let w = code_ref.width();
        let min_size = outer_size.x.min(outer_size.y);
        let scale = min_size / (w as f32 + (self.quiet_zone * 2.));
        let start = ui.cursor().min + vec2(self.quiet_zone * scale, self.quiet_zone * scale);
        let (response, painter) = ui.allocate_painter(vec2(min_size, min_size), Sense::click());

        painter.rect(response.rect, Rounding::ZERO, Color32::WHITE, Stroke::NONE);
        let mut ctr = 0;
        for c in code_ref.to_colors() {
            let row = ctr / w;
            let col = ctr % w;
            let c_start = start.floor() + vec2(col as f32 * scale, row as f32 * scale).floor();
            let c_end = c_start.ceil() + vec2(scale, scale).ceil();
            if matches!(c, Color::Dark) {
                painter.rect(
                    Rect::from_min_max(c_start, c_end),
                    Rounding::ZERO,
                    match c {
                        Color::Light => Color32::WHITE,
                        Color::Dark => Color32::BLACK,
                    },
                    Stroke::NONE,
                );
            }
            ctr += 1;
        }
        response
    }
}
