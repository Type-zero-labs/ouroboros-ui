//! Edges (wires) — cubic-bezier connections between two ports.
//!
//! Paint tier: samples a cubic bezier between the two handle anchors and draws it as a token-
//! colored polyline into the canvas's reserved under-node slot. Control points leave each port
//! perpendicular to its side, so wires fan out cleanly. Hover/selection are hit-tested
//! geometrically (no egui Response — edges are too thin for a rect) against the cursor in scene
//! space, with a screen-constant grab radius. Variants: default, animated, with-button, with-label.

use egui::{Pos2, Shape, Stroke, Vec2};

use crate::atoms::{Badge, Button};
use crate::tokens::core;

use super::canvas::GraphCtx;
use super::{Port, PortSide};

/// Number of segments the bezier is flattened into (draw + hit-test).
const SAMPLES: usize = 24;
/// Minimum horizontal control-point reach, so short edges still bow.
const MIN_REACH: f32 = 40.0;

/// Visual style of an edge.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EdgeStyle {
    /// A plain wire.
    #[default]
    Default,
    /// A wire with a dot travelling along it to show flow direction.
    Animated,
    /// A wire with a small icon-button at its midpoint (e.g. delete / insert).
    WithButton,
    /// A wire with a text badge at its midpoint.
    WithLabel,
}

/// Outcome of emitting one edge.
#[derive(Clone, Copy, Debug, Default)]
pub struct EdgeResult {
    pub hovered: bool,
    pub selected: bool,
    /// The wire was clicked this frame (selects it).
    pub clicked: bool,
    /// The midpoint button (WithButton) was clicked this frame.
    pub button_clicked: bool,
}

/// Direction a wire leaves a port, by side (horizontal flow).
fn side_dir(side: PortSide) -> Vec2 {
    match side {
        PortSide::Out => Vec2::new(1.0, 0.0),
        PortSide::In => Vec2::new(-1.0, 0.0),
    }
}

/// The four control points of the cubic bezier from `a` (side `a_side`) to `b` (side `b_side`).
fn control_points(a: Pos2, a_side: PortSide, b: Pos2, b_side: PortSide) -> [Pos2; 4] {
    let reach = ((a.x - b.x).abs() * 0.5).max(MIN_REACH);
    let c1 = a + side_dir(a_side) * reach;
    let c2 = b + side_dir(b_side) * reach;
    [a, c1, c2, b]
}

/// Sample the cubic bezier into `SAMPLES + 1` points.
fn sample(cps: [Pos2; 4]) -> Vec<Pos2> {
    let [p0, p1, p2, p3] = cps;
    (0..=SAMPLES)
        .map(|i| {
            let t = i as f32 / SAMPLES as f32;
            let u = 1.0 - t;
            let w = [u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t];
            Pos2::new(
                w[0] * p0.x + w[1] * p1.x + w[2] * p2.x + w[3] * p3.x,
                w[0] * p0.y + w[1] * p1.y + w[2] * p2.y + w[3] * p3.y,
            )
        })
        .collect()
}

/// Point at parameter `t` (used for midpoint decorations).
fn point_at(cps: [Pos2; 4], t: f32) -> Pos2 {
    let [p0, p1, p2, p3] = cps;
    let u = 1.0 - t;
    let w = [u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t];
    Pos2::new(
        w[0] * p0.x + w[1] * p1.x + w[2] * p2.x + w[3] * p3.x,
        w[0] * p0.y + w[1] * p1.y + w[2] * p2.y + w[3] * p3.y,
    )
}

/// Shortest distance from `p` to the polyline `pts`.
fn dist_to_polyline(pts: &[Pos2], p: Pos2) -> f32 {
    pts.windows(2)
        .map(|w| dist_to_segment(p, w[0], w[1]))
        .fold(f32::INFINITY, f32::min)
}

fn dist_to_segment(p: Pos2, a: Pos2, b: Pos2) -> f32 {
    let ab = b - a;
    let len_sq = ab.length_sq();
    if len_sq < f32::EPSILON {
        return p.distance(a);
    }
    let t = ((p - a).dot(ab) / len_sq).clamp(0.0, 1.0);
    (a + ab * t).distance(p)
}

impl GraphCtx<'_> {
    /// Emit a bezier edge from port `from` to port `to`. Anchored on the ports' handles (so it
    /// must be emitted after both nodes). Returns hover/selection/click results; the wire is
    /// drawn under the nodes.
    pub fn edge(&mut self, from: Port, to: Port, style: EdgeStyle) -> EdgeResult {
        let (Some(a), Some(b)) = (self.handle_pos(&from), self.handle_pos(&to)) else {
            return EdgeResult::default();
        };

        let cps = control_points(a, from.side, b, to.side);
        let pts = sample(cps);

        // Geometric hover hit-test (grab radius constant in screen px → divide by zoom).
        let hover_thresh = self.tokens.edge_hit_radius / self.scale().max(f32::EPSILON);
        let hovered = self
            .pointer_world()
            .is_some_and(|p| dist_to_polyline(&pts, p) <= hover_thresh);

        let key = (from, to);
        let selected = self.edge_selection == Some(key);

        let mut clicked = false;
        if hovered && self.ui.input(|i| i.pointer.primary_pressed()) {
            self.edge_selection = Some(key);
            self.edge_clicked = Some(key);
            clicked = true;
        }

        let color = if selected {
            self.tokens.edge_selected
        } else if hovered {
            self.tokens.edge_hover
        } else {
            self.tokens.edge
        };
        let width = if selected || hovered {
            core::EDGE_WIDTH + 0.5
        } else {
            self.tokens.edge_width
        };
        self.edge_shapes
            .push(Shape::line(pts, Stroke::new(width, color)));

        // Animated: a dot travelling along the wire (time-driven), drawn on top of the wire.
        if style == EdgeStyle::Animated {
            let t = self.ui.input(|i| i.time).rem_euclid(1.0) as f32;
            let dot = point_at(cps, t);
            self.edge_shapes
                .push(Shape::circle_filled(dot, self.tokens.edge_width, color));
            self.ui.ctx().request_repaint();
        }

        let mut button_clicked = false;
        let mid = point_at(cps, 0.5);
        match style {
            EdgeStyle::WithButton => {
                // A small floating icon-button at the midpoint (own child ui in scene coords).
                let size = Vec2::splat(self.tokens.handle_hit_radius * 2.0);
                let rect = egui::Rect::from_center_size(mid, size);
                let mut child = self.ui.new_child(egui::UiBuilder::new().max_rect(rect));
                let r = Button::new("")
                    .ghost()
                    .sm()
                    .icon_only()
                    .icon_left(egui_phosphor_x())
                    .id_source(("graph_edge_btn", key_hash(key)))
                    .show(&mut child);
                button_clicked = r.clicked();
            }
            EdgeStyle::WithLabel => {
                let rect = egui::Rect::from_center_size(mid, Vec2::new(80.0, 22.0));
                let mut child = self.ui.new_child(egui::UiBuilder::new().max_rect(rect));
                Badge::new("edge").show(&mut child);
            }
            _ => {}
        }

        EdgeResult {
            hovered,
            selected,
            clicked,
            button_clicked,
        }
    }
}

/// The phosphor glyph used on the WithButton edge (an "x" delete affordance).
fn egui_phosphor_x() -> &'static str {
    egui_phosphor::light::X
}

/// Stable-ish hash of an edge key for widget ids.
fn key_hash(key: (Port, Port)) -> u64 {
    let (f, t) = key;
    let mut h = f.node.0.wrapping_mul(0x9E3779B97F4A7C15);
    h ^= (f.port.0 as u64).wrapping_add(1);
    h = h.wrapping_mul(31).wrapping_add(t.node.0);
    h.wrapping_mul(31).wrapping_add(t.port.0 as u64)
}
