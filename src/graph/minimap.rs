//! MiniMap — a corner overview of the whole graph with the current viewport outlined.
//!
//! Drawn in a foreground [`Area`] (above the Scene, so it receives clicks) in the canvas's
//! top-right corner. Shows every node as a small rect and the visible region as an outline;
//! clicking or dragging inside it recenters the view on that world point. Paints geometric rects
//! only (token-colored) — O(nodes) per frame, fine for hundreds of nodes.

use egui::{Area, Order, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};

use super::tokens::GraphTokens;
use super::NodeId;
use crate::tokens::core;
use crate::Theme;

/// MiniMap footprint (screen px).
const MINI: Vec2 = Vec2::new(168.0, 120.0);

/// Draw the minimap. `nodes`/`world_bounds`/`view` are in scene (world) coordinates. Returns a
/// requested new view center (world) if the user clicked/dragged inside it.
pub(crate) fn show(
    ui: &mut egui::Ui,
    canvas: Rect,
    nodes: &[(NodeId, Rect)],
    world_bounds: Rect,
    view: Rect,
) -> Option<Pos2> {
    let theme = Theme::get(ui);
    let tokens = GraphTokens::resolve(&theme);

    // World region the minimap maps from: content + current view, padded a touch.
    let src = world_bounds.union(view).expand(core::SPACE_4);
    if src.width() <= f32::EPSILON || src.height() <= f32::EPSILON {
        return None;
    }

    let pos = canvas.right_top() + Vec2::new(-MINI.x - core::SPACE_3, core::SPACE_3);
    let mut requested = None;

    Area::new(ui.id().with("graph_minimap"))
        .order(Order::Foreground)
        .fixed_pos(pos)
        .show(ui.ctx(), |ui| {
            let (frame, resp) = ui.allocate_exact_size(MINI, Sense::click_and_drag());
            let painter = ui.painter_at(frame);

            // Backdrop + border.
            painter.rect_filled(frame, core::RADIUS_MD as u8, theme.popover);
            painter.rect_stroke(
                frame,
                core::RADIUS_MD as u8,
                Stroke::new(core::BORDER_THIN, theme.border),
                StrokeKind::Inside,
            );

            // Fit `src` into the inner frame, centered.
            let inner = frame.shrink(core::SPACE_1);
            let scale = (inner.width() / src.width()).min(inner.height() / src.height());
            let offset = inner.center().to_vec2() - src.center().to_vec2() * scale;
            let to_mini = |p: Pos2| -> Pos2 { (p.to_vec2() * scale + offset).to_pos2() };
            let from_mini = |p: Pos2| -> Pos2 { ((p.to_vec2() - offset) / scale).to_pos2() };

            // Nodes.
            for (_, r) in nodes {
                let mr = Rect::from_min_max(to_mini(r.min), to_mini(r.max));
                painter.rect_filled(mr, core::RADIUS_NONE as u8, tokens.minimap_node);
            }
            // Viewport outline.
            let vr = Rect::from_min_max(to_mini(view.min), to_mini(view.max));
            painter.rect_stroke(
                vr,
                core::RADIUS_NONE as u8,
                Stroke::new(core::BORDER_FOCUS, tokens.minimap_view),
                StrokeKind::Inside,
            );

            // Click/drag → recenter.
            if resp.clicked() || resp.dragged() {
                if let Some(p) = resp.interact_pointer_pos() {
                    if frame.contains(p) {
                        requested = Some(from_mini(p));
                    }
                }
            }
        });

    requested
}
