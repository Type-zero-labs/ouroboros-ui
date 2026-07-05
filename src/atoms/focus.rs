//! Focus-ring painters — the single source of truth for the keyboard-focus outline.
//!
//! Interactive atoms draw the same ring inside `if response.has_focus()`: a 2px
//! [`core::BORDER_FOCUS`] stroke in `theme.ring`, offset [`core::RING_OFFSET`] outside the
//! widget. Two shapes cover every atom — rectangular ([`focus_ring_rect`], for
//! button/checkbox/switch) and circular ([`focus_ring_circle`], for radio/slider). Keeping the
//! offset/stroke in one place guarantees the five atoms never drift.

use crate::tokens::core;
use egui::{Color32, CornerRadius, Painter, Pos2, Rect, Stroke, StrokeKind};

/// Focus ring around a rectangular widget — drawn just outside `rect`, matching its corners.
pub(crate) fn focus_ring_rect(
    painter: &Painter,
    rect: Rect,
    radius: impl Into<CornerRadius>,
    ring: Color32,
) {
    painter.rect_stroke(
        rect.expand(core::RING_OFFSET),
        radius,
        Stroke::new(core::BORDER_FOCUS, ring),
        StrokeKind::Outside,
    );
}

/// Focus ring around a circular widget — drawn just outside the circle of the given radius.
pub(crate) fn focus_ring_circle(painter: &Painter, center: Pos2, radius: f32, ring: Color32) {
    painter.circle_stroke(
        center,
        radius + core::RING_OFFSET,
        Stroke::new(core::BORDER_FOCUS, ring),
    );
}
