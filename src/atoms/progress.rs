//! Progress atom — a determinate bar, continuous or stepped. [shadcn Progress / Unity Progress Bar]

use crate::tokens::core;
use crate::Theme;
use egui::{vec2, CornerRadius, Rect, Response, Sense, Ui};

/// A determinate progress bar (`fraction` in `0..=1`). `.steps(n)` renders `n` discrete segments.
pub struct Progress {
    fraction: f32,
    steps: Option<usize>,
}

impl Progress {
    pub fn new(fraction: f32) -> Self {
        Self {
            fraction: fraction.clamp(0.0, 1.0),
            steps: None,
        }
    }
    /// Render as `n` discrete segments (filled by rounded fraction).
    pub fn steps(mut self, n: usize) -> Self {
        self.steps = Some(n.max(1));
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let height = core::SPACE_2;
        let width = ui.available_width();
        let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        let pill = CornerRadius::same((height / 2.0) as u8);
        let painter = ui.painter();

        match self.steps {
            None => {
                painter.rect_filled(rect, pill, theme.muted);
                let fill_w = rect.width() * self.fraction;
                if fill_w > 0.0 {
                    let fill = Rect::from_min_size(rect.min, vec2(fill_w, height));
                    painter.rect_filled(fill, pill, theme.primary);
                }
            }
            Some(n) => {
                let gap = core::SPACE_1;
                let seg_w = ((rect.width() - gap * (n - 1) as f32) / n as f32).max(0.0);
                let filled = (self.fraction * n as f32).round() as usize;
                for i in 0..n {
                    let x = rect.left() + i as f32 * (seg_w + gap);
                    let seg = Rect::from_min_size(egui::pos2(x, rect.top()), vec2(seg_w, height));
                    let color = if i < filled {
                        theme.primary
                    } else {
                        theme.muted
                    };
                    painter.rect_filled(seg, pill, color);
                }
            }
        }
        response
    }
}
