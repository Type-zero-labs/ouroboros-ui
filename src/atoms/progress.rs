//! Progress atom — a determinate progress bar. [shadcn Progress / Unity Progress Bar]

use crate::tokens::core;
use crate::Theme;
use egui::{vec2, CornerRadius, Rect, Response, Sense, Ui};

/// A determinate progress bar (`fraction` in `0..=1`). Builder; `show` returns the [`Response`].
pub struct Progress {
    fraction: f32,
}

impl Progress {
    pub fn new(fraction: f32) -> Self {
        Self {
            fraction: fraction.clamp(0.0, 1.0),
        }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let height = core::SPACE_2;
        let width = ui.available_width();
        let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        let pill = CornerRadius::same((height / 2.0) as u8);
        let painter = ui.painter();
        painter.rect_filled(rect, pill, theme.muted);
        let fill_w = rect.width() * self.fraction;
        if fill_w > 0.0 {
            let fill = Rect::from_min_size(rect.min, vec2(fill_w, height));
            painter.rect_filled(fill, pill, theme.primary);
        }
        response
    }
}
