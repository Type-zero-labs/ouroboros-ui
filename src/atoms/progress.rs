//! Progress atom — a determinate bar (continuous / stepped) or a circular ring.
//! [shadcn Progress / Unity Progress Bar]

use crate::tokens::core;
use crate::tokens::layout;
use crate::Theme;
use egui::{vec2, CornerRadius, Rect, Response, Sense, Shape, Stroke, Ui, Vec2};

/// A determinate progress indicator (`fraction` in `0..=1`).
pub struct Progress {
    fraction: f32,
    steps: Option<usize>,
    circular: Option<f32>,
}

impl Progress {
    pub fn new(fraction: f32) -> Self {
        Self {
            fraction: fraction.clamp(0.0, 1.0),
            steps: None,
            circular: None,
        }
    }
    /// Render as `n` discrete segments.
    pub fn steps(mut self, n: usize) -> Self {
        self.steps = Some(n.max(1));
        self
    }
    /// Render as a circular ring (default diameter).
    pub fn circular(mut self) -> Self {
        self.circular = Some(core::CONTROL_LG);
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        if let Some(size) = self.circular {
            let (rect, response) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
            let thickness = core::SPACE_1;
            let center = rect.center();
            let r = size / 2.0 - thickness / 2.0;
            let painter = ui.painter();
            painter.circle_stroke(center, r, Stroke::new(thickness, theme.muted));
            if self.fraction > 0.0 {
                let start = -std::f32::consts::FRAC_PI_2;
                let sweep = self.fraction * std::f32::consts::TAU;
                let n = 48;
                let points: Vec<_> = (0..=n)
                    .map(|i| {
                        let a = start + sweep * (i as f32 / n as f32);
                        center + Vec2::angled(a) * r
                    })
                    .collect();
                painter.add(Shape::line(points, Stroke::new(thickness, theme.primary)));
            }
            return response;
        }

        let height = core::SPACE_2;
        // Fill the panel, but keep the track legible in narrow panels (intrinsic floor).
        let width = ui.available_width().max(layout::PROGRESS_MIN_W);
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
