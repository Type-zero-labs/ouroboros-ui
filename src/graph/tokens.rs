//! Graph component tokens — the single resolve point for everything the `graph` layer paints.
//!
//! Mirrors the [`ButtonTokens`](crate::tokens::component::ButtonTokens) pattern: a flat struct
//! of the exact paint values, built from a [`Theme`] (colors) and [`core`] (geometry). The
//! paint-tier modules (`grid`, `edge`, `handle`, …) read from a resolved `GraphTokens` instead
//! of touching `Theme` fields or `core` colors ad-hoc, which keeps the `no_raw_values` guard
//! green and gives one place to retune the graph's look.

use crate::tokens::core;
use crate::Theme;
use egui::Color32;

/// Resolved paint values for the node-graph canvas. Build once per frame with
/// [`GraphTokens::resolve`] and thread it through the paint helpers.
#[derive(Clone, Copy, Debug)]
pub struct GraphTokens {
    // ── background grid ──
    pub grid_dot: Color32,
    pub grid_dot_radius: f32,
    pub grid_spacing: f32,
    // ── edges (wires) ──
    pub edge: Color32,
    pub edge_hover: Color32,
    pub edge_selected: Color32,
    pub edge_width: f32,
    pub edge_hit_radius: f32,
    // ── handles (ports) ──
    pub handle_fill: Color32,
    pub handle_border: Color32,
    pub handle_radius: f32,
    pub handle_hit_radius: f32,
    // ── node selection ──
    pub node_selected_ring: Color32,
    // ── box-select marquee ──
    pub marquee_fill: Color32,
    pub marquee_border: Color32,
    // ── minimap ──
    pub minimap_node: Color32,
    pub minimap_view: Color32,
}

impl GraphTokens {
    /// Map a [`Theme`] (+ `core` geometry) onto the graph's paint values. Colors are pure
    /// `Theme` tokens; the marquee fill is the focus ring re-tinted translucent via
    /// [`core::tint`].
    pub fn resolve(theme: &Theme) -> Self {
        Self {
            grid_dot: theme.border,
            grid_dot_radius: core::GRID_DOT_RADIUS,
            grid_spacing: core::GRID_SPACING,

            edge: theme.muted_foreground,
            edge_hover: theme.primary,
            edge_selected: theme.ring,
            edge_width: core::EDGE_WIDTH,
            edge_hit_radius: core::EDGE_HIT_RADIUS,

            handle_fill: theme.primary,
            handle_border: theme.border_strong,
            handle_radius: core::HANDLE_RADIUS,
            handle_hit_radius: core::HANDLE_RADIUS * 2.0,

            node_selected_ring: theme.ring,

            marquee_fill: core::tint(theme.ring, core::MARQUEE_ALPHA),
            marquee_border: theme.ring,

            minimap_node: theme.muted_foreground,
            minimap_view: theme.ring,
        }
    }

    /// Convenience: resolve straight from the theme installed in `ui`.
    pub fn get(ui: &egui::Ui) -> Self {
        Self::resolve(&Theme::get(ui))
    }
}
