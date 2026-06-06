//! NumericField atom — a scrubbable numeric input. [Unity Numeric Field]
//!
//! Token box wrapping an egui [`DragValue`](egui::DragValue): drag to scrub, click to type.
//! The editing substrate is egui's; the casing is token.

use crate::tokens::core;
use crate::Theme;
use egui::{
    vec2, Align, Color32, CornerRadius, DragValue, Layout, Response, Sense, Stroke, StrokeKind, Ui,
    UiBuilder,
};

/// A scrubbable numeric field bound to a `&mut f32`. Builder; `show` returns the [`Response`].
pub struct NumericField<'a> {
    value: &'a mut f32,
    min: f32,
    max: f32,
    speed: f32,
    suffix: Option<String>,
    enabled: bool,
}

impl<'a> NumericField<'a> {
    pub fn new(value: &'a mut f32) -> Self {
        Self {
            value,
            min: f32::NEG_INFINITY,
            max: f32::INFINITY,
            speed: 0.1,
            suffix: None,
            enabled: true,
        }
    }

    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    pub fn disabled(self) -> Self {
        self.enabled(false)
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let height = core::CONTROL_MD;
        let width = ui.available_width();
        let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        let dim = |c: Color32| {
            if self.enabled {
                c
            } else {
                c.gamma_multiply(core::OPACITY_DISABLED)
            }
        };
        let radius = CornerRadius::same(core::RADIUS_MD as u8);
        let painter = ui.painter().clone();
        painter.rect_filled(rect, radius, dim(theme.muted));

        let inner = rect.shrink2(vec2(core::SPACE_2, 0.0));
        let mut cui = ui.new_child(
            UiBuilder::new()
                .max_rect(inner)
                .layout(Layout::left_to_right(Align::Center)),
        );
        let mut dv = DragValue::new(self.value)
            .speed(self.speed)
            .range(self.min..=self.max);
        if let Some(suffix) = self.suffix {
            dv = dv.suffix(suffix);
        }
        let resp = cui.add_enabled(self.enabled, dv);

        let (border, w) = if resp.has_focus() {
            (theme.ring, core::BORDER_FOCUS)
        } else {
            (theme.input, core::BORDER_THIN)
        };
        painter.rect_stroke(
            rect,
            radius,
            Stroke::new(w, dim(border)),
            StrokeKind::Inside,
        );
        resp
    }
}
