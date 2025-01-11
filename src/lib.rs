use egui::{vec2, Color32, Rect, Response, Rounding, Sense, Stroke, Ui, Widget};
use qrcode::types::QrError;
use qrcode::{Color, QrCode};

enum CodeSrc<'a> {
    Ref(&'a QrCode),
    Owned(QrCode),
}

pub struct QrCodeWidget<'a> {
    code: CodeSrc<'a>,
}

impl<'a> QrCodeWidget<'a> {
    pub fn new(code: &'a QrCode) -> Self {
        QrCodeWidget {
            code: CodeSrc::Ref(code),
        }
    }

    pub fn from_data(data: &[u8]) -> Result<Self, QrError> {
        let code = QrCode::new(data)?;
        Ok(QrCodeWidget {
            code: CodeSrc::Owned(code),
        })
    }
}

impl Widget for QrCodeWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let size = ui.available_size();

        let code_ref = match &self.code {
            CodeSrc::Ref(code) => code,
            CodeSrc::Owned(q) => q,
        };
        let w = code_ref.width();
        let scale = (size.x.min(size.y) / w as f32).floor();
        let start = ui.cursor().min;
        let (response, painter) =
            ui.allocate_painter(vec2(w as f32 * scale, w as f32 * scale), Sense::click());

        painter.rect(response.rect, Rounding::ZERO, Color32::WHITE, Stroke::NONE);
        let mut ctr = 0;
        for c in code_ref.to_colors() {
            let row = ctr / w;
            let col = ctr % w;
            let c_start = start + vec2(col as f32 * scale, row as f32 * scale);
            let c_end = c_start + vec2(scale, scale);
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
