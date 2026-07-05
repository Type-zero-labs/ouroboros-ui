//! ColorSwatch atom — a swatch showing a color (base of a color field). [Unity ColorField]
//!
//! Displays an arbitrary `Color32` (consumer data, not a token) in a token-bordered square or
//! circle; click → [`Response`] to open a picker.

use crate::tokens::core;
use crate::Theme;
use egui::{vec2, Color32, CornerRadius, Response, Sense, Stroke, StrokeKind, Ui};

/// A color swatch. Builder; `show` returns the [`Response`].
pub struct ColorSwatch {
    color: Color32,
    size: f32,
    circle: bool,
}

impl ColorSwatch {
    pub fn new(color: Color32) -> Self {
        Self {
            color,
            size: core::ICON_LG,
            circle: false,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    pub fn circle(mut self) -> Self {
        self.circle = true;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let (rect, response) = ui.allocate_exact_size(vec2(self.size, self.size), Sense::click());
        let painter = ui.painter();
        if self.circle {
            let r = self.size / 2.0;
            painter.circle_filled(rect.center(), r, self.color);
            painter.circle_stroke(
                rect.center(),
                r,
                Stroke::new(core::BORDER_THIN, theme.border),
            );
        } else {
            let radius = CornerRadius::same(core::RADIUS_SM as u8);
            painter.rect_filled(rect, radius, self.color);
            painter.rect_stroke(
                rect,
                radius,
                Stroke::new(core::BORDER_THIN, theme.border),
                StrokeKind::Inside,
            );
        }
        response
    }
}
