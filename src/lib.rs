use egui::{Color32, CornerRadius, Rect, Response, Sense, Stroke, StrokeKind, Ui, Widget, vec2};
use qrcode::types::QrError;
use qrcode::{Color, QrCode};

enum CodeSrc<'a> {
    Ref(&'a QrCode),
    Owned(QrCode),
}

pub struct QrCodeWidget<'a> {
    code: CodeSrc<'a>,
    quiet_zone: f32,
    size: Option<f32>,
}

impl<'a> QrCodeWidget<'a> {
    pub fn new(code: &'a QrCode) -> Self {
        QrCodeWidget {
            code: CodeSrc::Ref(code),
            quiet_zone: 1.,
            size: None,
        }
    }

    pub fn from_data(data: &[u8]) -> Result<Self, QrError> {
        let code = QrCode::new(data)?;
        Ok(QrCodeWidget {
            code: CodeSrc::Owned(code),
            quiet_zone: 1.,
            size: None,
        })
    }

    /// Blank modules around the code, 1 by default. Scanners want at least 2 to 4.
    pub fn quiet_zone(mut self, quiet_zone: f32) -> Self {
        self.quiet_zone = quiet_zone;
        self
    }

    /// Draw it `size` points square. Without this it fills the space available.
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }
}

impl Widget for QrCodeWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {

        let code_ref = match &self.code {
            CodeSrc::Ref(code) => code,
            CodeSrc::Owned(q) => q,
        };
        let w = code_ref.width();
        let min_size = self.size.unwrap_or_else(|| {
            let outer = ui.available_size();
            outer.x.min(outer.y)
        });
        let scale = min_size / (w as f32 + (self.quiet_zone * 2.));
        let start = ui.cursor().min + vec2(self.quiet_zone * scale, self.quiet_zone * scale);
        let (response, painter) = ui.allocate_painter(vec2(min_size, min_size), Sense::click());

        painter.rect(response.rect, CornerRadius::ZERO, Color32::WHITE, Stroke::NONE, StrokeKind::Inside);
        let mut ctr = 0;
        for c in code_ref.to_colors() {
            let row = ctr / w;
            let col = ctr % w;
            let c_start = start.floor() + vec2(col as f32 * scale, row as f32 * scale).floor();
            let c_end = c_start.ceil() + vec2(scale, scale).ceil();
            if matches!(c, Color::Dark) {
                painter.rect(
                    Rect::from_min_max(c_start, c_end),
                    CornerRadius::ZERO,
                    match c {
                        Color::Light => Color32::WHITE,
                        Color::Dark => Color32::BLACK,
                    },
                    Stroke::NONE,
                    StrokeKind::Inside
                );
            }
            ctr += 1;
        }
        response
    }
}
