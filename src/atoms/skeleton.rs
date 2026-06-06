//! Skeleton atom — a loading placeholder block. [shadcn Skeleton]
//!
//! A muted rounded rect that gently pulses (opacity) while content loads.

use crate::tokens::core;
use crate::Theme;
use egui::{vec2, CornerRadius, Response, Sense, Ui};

/// A loading placeholder. Builder; `show` returns the [`Response`].
pub struct Skeleton {
    width: Option<f32>,
    height: f32,
    pulse: bool,
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            width: None,
            height: core::SPACE_4,
            pulse: true,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
    pub fn still(mut self) -> Self {
        self.pulse = false;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let width = self.width.unwrap_or_else(|| ui.available_width());
        let (rect, response) = ui.allocate_exact_size(vec2(width, self.height), Sense::hover());
        let mut color = theme.muted;
        if self.pulse {
            let t = ui.input(|i| i.time) as f32;
            let factor = core::OPACITY_MUTED + (1.0 - core::OPACITY_MUTED) * (t * 2.0).sin().abs();
            color = color.gamma_multiply(factor);
            ui.ctx().request_repaint();
        }
        ui.painter()
            .rect_filled(rect, CornerRadius::same(core::RADIUS_SM as u8), color);
        response
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}
