//! Slider atom — a draggable numeric value over a range. [shadcn/Unity/O3DE Slider]
//!
//! Track (`muted`) + filled portion (`primary`) + a `primary` thumb. Drag or click to set.

use crate::tokens::core;
use crate::Theme;
use egui::{pos2, vec2, CornerRadius, Rect, Response, Sense, Stroke, Ui};

/// A slider bound to a `&mut f32`. Builder; `show` returns the [`Response`].
pub struct Slider<'a> {
    value: &'a mut f32,
    min: f32,
    max: f32,
    step: Option<f32>,
}

impl<'a> Slider<'a> {
    pub fn new(value: &'a mut f32) -> Self {
        Self {
            value,
            min: 0.0,
            max: 1.0,
            step: None,
        }
    }

    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }
    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let height = core::ICON_MD;
        let width = ui.available_width();
        let (rect, mut response) =
            ui.allocate_exact_size(vec2(width, height), Sense::click_and_drag());

        let thumb_r = height / 2.0 - core::BORDER_THIN;
        let left = rect.left() + thumb_r;
        let right = rect.right() - thumb_r;
        let usable = (right - left).max(1.0);
        let span = (self.max - self.min).max(f32::EPSILON);

        if response.dragged() || response.clicked() {
            if let Some(p) = response.interact_pointer_pos() {
                let t = ((p.x - left) / usable).clamp(0.0, 1.0);
                let mut v = self.min + t * span;
                if let Some(step) = self.step {
                    if step > 0.0 {
                        v = (v / step).round() * step;
                    }
                }
                *self.value = v.clamp(self.min, self.max);
                response.mark_changed();
            }
        }

        let t = ((*self.value - self.min) / span).clamp(0.0, 1.0);
        let cy = rect.center().y;
        let thumb_x = left + t * usable;
        let track_h = core::SPACE_1;
        let pill = CornerRadius::same((track_h / 2.0) as u8);
        let painter = ui.painter();

        let track = Rect::from_min_max(
            pos2(left, cy - track_h / 2.0),
            pos2(right, cy + track_h / 2.0),
        );
        painter.rect_filled(track, pill, theme.muted);
        let fill = Rect::from_min_max(
            pos2(left, cy - track_h / 2.0),
            pos2(thumb_x, cy + track_h / 2.0),
        );
        painter.rect_filled(fill, pill, theme.primary);

        painter.circle_filled(pos2(thumb_x, cy), thumb_r, theme.primary);
        painter.circle_stroke(
            pos2(thumb_x, cy),
            thumb_r,
            Stroke::new(core::BORDER_THIN, theme.background),
        );
        if response.has_focus() {
            super::focus::focus_ring_circle(painter, pos2(thumb_x, cy), thumb_r, theme.ring);
        }
        response
    }
}
