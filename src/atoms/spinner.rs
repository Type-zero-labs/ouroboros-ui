//! Spinner atom — an indeterminate loading arc.
//!
//! A ~270° stroked arc that rotates over time (`request_repaint` each frame). Size and
//! color are tokens.

use crate::tokens::core;
use crate::Theme;
use egui::{vec2, Color32, Response, Sense, Shape, Stroke, Ui, Vec2};

/// An indeterminate spinner. Builder; `show` returns the [`Response`].
pub struct Spinner {
    size: f32,
    color: Option<Color32>,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            size: core::ICON_MD,
            color: None,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    pub fn sm(self) -> Self {
        self.size(core::ICON_SM)
    }
    pub fn lg(self) -> Self {
        self.size(core::ICON_LG)
    }
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let (rect, response) = ui.allocate_exact_size(vec2(self.size, self.size), Sense::hover());
        let color = self.color.unwrap_or(theme.muted_foreground);

        let t = ui.input(|i| i.time) as f32;
        let center = rect.center();
        let r = self.size / 2.0 - core::BORDER_FOCUS;
        let start = t * std::f32::consts::TAU;
        let sweep = 0.75 * std::f32::consts::TAU;
        let n = 32;
        let points: Vec<_> = (0..=n)
            .map(|i| {
                let a = start + sweep * (i as f32 / n as f32);
                center + Vec2::angled(a) * r
            })
            .collect();
        ui.painter()
            .add(Shape::line(points, Stroke::new(core::BORDER_FOCUS, color)));
        ui.ctx().request_repaint();

        response
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}
