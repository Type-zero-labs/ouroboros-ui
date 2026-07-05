//! NodeResizer — a bottom-right corner grip for resizing a sized node.
//!
//! Paint tier: draws the grip and reports a world-space size delta. Only meaningful for nodes
//! given an explicit [`NodeFrame::size`](super::node::NodeFrame::size); content-hugging nodes
//! have no size to drive. The caller applies the delta to its own width/height.

use egui::{Rect, Vec2};

use super::tokens::GraphTokens;

/// Side length (world px) of the corner grip.
pub(crate) const GRIP: f32 = 10.0;

/// Scene rect of the grip at a node's bottom-right corner.
pub(crate) fn grip_rect(node: Rect) -> Rect {
    Rect::from_center_size(node.right_bottom(), Vec2::splat(GRIP))
}

/// Paint the grip (a small filled square in the ring color).
pub(crate) fn paint(painter: &egui::Painter, node: Rect, tokens: &GraphTokens) {
    use crate::tokens::core;
    let r = grip_rect(node);
    painter.rect_filled(r, core::RADIUS_SM as u8, tokens.node_selected_ring);
}
