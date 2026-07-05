//! NodeToolbar — a floating action bar anchored above a node.
//!
//! Compose tier: a [`Surface`] bar hosting caller-supplied DS widgets, placed just above the
//! node's rect in scene coordinates (so it tracks and scales with the node). Typically shown by
//! the caller only while the node is selected. Emit it *after* the node so its rect is known.

use egui::{Align, Layout, Rect, UiBuilder, Vec2};

use crate::atoms::Surface;
use crate::tokens::core;

use super::canvas::GraphCtx;
use super::NodeId;

impl GraphCtx<'_> {
    /// Draw a toolbar above `node` (a no-op if the node hasn't been emitted yet this frame).
    /// `content` lays out its actions left-to-right inside an elevated surface.
    pub fn node_toolbar(&mut self, node: NodeId, content: impl FnOnce(&mut egui::Ui)) {
        let Some((_, rect)) = self.node_rects.iter().copied().find(|(n, _)| *n == node) else {
            return;
        };

        let height = core::CONTROL_MD + core::SPACE_2;
        let anchor = rect.left_top() - Vec2::new(0.0, height + core::SPACE_2);
        let area = Rect::from_min_size(anchor, Vec2::new(rect.width().max(120.0), height));

        let mut child = self.ui.new_child(
            UiBuilder::new()
                .max_rect(area)
                .layout(Layout::left_to_right(Align::Center)),
        );
        Surface::new()
            .elevated()
            .pad(core::SPACE_1)
            .show(&mut child, |ui| {
                ui.horizontal(|ui| content(ui));
            });
    }
}
