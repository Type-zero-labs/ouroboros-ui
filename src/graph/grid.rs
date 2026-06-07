//! Background dot-grid — paints the canvas backdrop inside the scene layer.
//!
//! Paint tier: touches the painter directly, but every value comes from [`GraphTokens`]. The
//! grid lives in **scene (world) coordinates**, so it pans and zooms uniformly with the nodes
//! (n8n-style). The caller culls it when the on-screen spacing gets too tight to read.

use egui::{Color32, Painter, Pos2, Rect};

/// Below this on-screen dot spacing (px) the grid is too dense to be useful — caller skips it.
pub const MIN_DOT_SPACING: f32 = 6.0;

/// Paint dots on a `spacing`-aligned lattice covering `visible` (a scene-space rect, e.g.
/// `ui.clip_rect()` inside the scene). `dot_radius`/`color` are scene-space token values.
pub fn paint(painter: &Painter, visible: Rect, spacing: f32, dot_radius: f32, color: Color32) {
    if spacing <= 0.0 {
        return;
    }
    let first_x = (visible.left() / spacing).floor() * spacing;
    let first_y = (visible.top() / spacing).floor() * spacing;
    let mut y = first_y;
    while y < visible.bottom() {
        let mut x = first_x;
        while x < visible.right() {
            painter.circle_filled(Pos2::new(x, y), dot_radius, color);
            x += spacing;
        }
        y += spacing;
    }
}
