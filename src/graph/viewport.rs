//! Viewport — the world↔screen transform for the graph canvas.
//!
//! A pure value type (`Copy`): it stores only `pan` + `zoom`, never the canvas rect. The
//! canvas origin (`canvas_rect.left_top()`) is passed in per call, so the math stays testable
//! without an `egui::Ui` and the whole thing is trivially storable in egui memory.
//!
//! Conventions: `world` is the graph's own coordinate space (node positions live here);
//! `screen` is egui pixels. `pan` is the screen-space offset of the world origin relative to
//! the canvas top-left; `zoom` is screen-px per world-unit. The zoom-anchored math is ported
//! from the studio's `events/canvas.rs`, cleaned up.

use egui::{Pos2, Rect, Vec2};

/// World↔screen transform. Build with [`Viewport::default`] (pan 0, zoom 1) and mutate via
/// [`pan_by`](Viewport::pan_by) / [`zoom_around`](Viewport::zoom_around) / [`fit`](Viewport::fit).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    /// Screen-space offset of the world origin, relative to the canvas top-left.
    pub pan: Vec2,
    /// Screen px per world unit.
    pub zoom: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            pan: Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

impl Viewport {
    /// Lower zoom clamp — nodes never shrink to dust.
    pub const MIN_ZOOM: f32 = 0.25;
    /// Upper zoom clamp — nodes never balloon past usefulness.
    pub const MAX_ZOOM: f32 = 2.5;

    /// World point → screen point. `canvas_origin` is the canvas rect's top-left.
    pub fn world_to_screen(&self, canvas_origin: Pos2, world: Pos2) -> Pos2 {
        canvas_origin + self.pan + world.to_vec2() * self.zoom
    }

    /// Screen point → world point. Inverse of [`world_to_screen`](Viewport::world_to_screen).
    pub fn screen_to_world(&self, canvas_origin: Pos2, screen: Pos2) -> Pos2 {
        (((screen - canvas_origin) - self.pan) / self.zoom).to_pos2()
    }

    /// Scale a world length to its on-screen length.
    pub fn scale(&self, world_len: f32) -> f32 {
        world_len * self.zoom
    }

    /// Pan by a screen-space delta (e.g. a drag delta).
    pub fn pan_by(&mut self, delta_screen: Vec2) {
        self.pan += delta_screen;
    }

    /// Multiply zoom by `factor`, keeping the world point currently under `anchor` (a screen
    /// point, usually the cursor) pinned in place. Clamped to `MIN_ZOOM..=MAX_ZOOM`.
    pub fn zoom_around(&mut self, canvas_origin: Pos2, anchor: Pos2, factor: f32) {
        let old = self.zoom;
        let new = (old * factor).clamp(Self::MIN_ZOOM, Self::MAX_ZOOM);
        if new == old {
            return;
        }
        // world point under the anchor before zooming…
        let world = ((anchor - canvas_origin) - self.pan) / old;
        // …must remain under it after: solve pan from world_to_screen(anchor) == anchor.
        self.pan = (anchor - canvas_origin) - world * new;
        self.zoom = new;
    }

    /// Frame `content_world` (the world-space bounding rect of all nodes) inside `canvas`,
    /// centered, leaving `margin` screen px on every side. No-op for an empty/degenerate
    /// content rect. Resulting zoom is clamped.
    pub fn fit(&mut self, content_world: Rect, canvas: Rect, margin: f32) {
        let cw = content_world.width();
        let ch = content_world.height();
        if cw <= f32::EPSILON || ch <= f32::EPSILON {
            return;
        }
        let avail = Vec2::new(
            (canvas.width() - 2.0 * margin).max(1.0),
            (canvas.height() - 2.0 * margin).max(1.0),
        );
        let zoom = (avail.x / cw)
            .min(avail.y / ch)
            .clamp(Self::MIN_ZOOM, Self::MAX_ZOOM);
        // Place the content's center at the canvas center.
        // screen_center = origin + pan + world_center*zoom  ⇒  pan = (canvas.center-origin) - wc*zoom
        let origin = canvas.left_top();
        let wc = content_world.center().to_vec2();
        self.zoom = zoom;
        self.pan = (canvas.center() - origin) - wc * zoom;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{pos2, vec2};

    fn approx(a: Pos2, b: Pos2) {
        assert!((a - b).length() < 1e-3, "{a:?} != {b:?}");
    }

    #[test]
    fn world_screen_round_trips() {
        let origin = pos2(100.0, 50.0);
        let vp = Viewport {
            pan: vec2(20.0, -10.0),
            zoom: 1.7,
        };
        for &w in &[pos2(0.0, 0.0), pos2(42.0, -13.0), pos2(-200.0, 999.0)] {
            let s = vp.world_to_screen(origin, w);
            approx(vp.screen_to_world(origin, s), w);
        }
    }

    #[test]
    fn zoom_keeps_point_under_cursor() {
        let origin = pos2(0.0, 0.0);
        let mut vp = Viewport::default();
        let cursor = pos2(300.0, 200.0);
        let world_before = vp.screen_to_world(origin, cursor);
        vp.zoom_around(origin, cursor, 1.8);
        let world_after = vp.screen_to_world(origin, cursor);
        approx(world_before, world_after);
        assert!((vp.zoom - 1.8).abs() < 1e-3);
    }

    #[test]
    fn zoom_clamps() {
        let origin = pos2(0.0, 0.0);
        let mut vp = Viewport::default();
        for _ in 0..50 {
            vp.zoom_around(origin, pos2(10.0, 10.0), 2.0);
        }
        assert!((vp.zoom - Viewport::MAX_ZOOM).abs() < 1e-3);
        for _ in 0..50 {
            vp.zoom_around(origin, pos2(10.0, 10.0), 0.5);
        }
        assert!((vp.zoom - Viewport::MIN_ZOOM).abs() < 1e-3);
    }

    #[test]
    fn fit_centers_content() {
        let canvas = Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 600.0));
        let content = Rect::from_min_size(pos2(100.0, 100.0), vec2(200.0, 100.0));
        let mut vp = Viewport::default();
        vp.fit(content, canvas, 32.0);
        // content center should land on canvas center
        let sc = vp.world_to_screen(canvas.left_top(), content.center());
        approx(sc, canvas.center());
        // and content must fit within the margins
        assert!(vp.scale(content.width()) <= canvas.width() - 2.0 * 32.0 + 1.0);
        assert!(vp.scale(content.height()) <= canvas.height() - 2.0 * 32.0 + 1.0);
    }

    #[test]
    fn fit_ignores_degenerate() {
        let canvas = Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 600.0));
        let mut vp = Viewport::default();
        let before = vp;
        vp.fit(
            Rect::from_min_size(pos2(0.0, 0.0), vec2(0.0, 0.0)),
            canvas,
            32.0,
        );
        assert_eq!(vp, before);
    }
}
