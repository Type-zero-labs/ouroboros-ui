//! Node frame — a draggable, selectable box drawn from the DS [`Surface`] atom.
//!
//! Compose tier (mostly): the body is a [`Surface`] (card fill, border, radius, elevation,
//! selection ring) hosting an arbitrary caller closure of DS widgets; because the whole thing
//! lives inside the scene layer, it scales with zoom for free. Handles (ports) are painted on
//! the node edges via the paint-tier [`handle`](super::handle) helpers. Position is in **world
//! coordinates**; the library reports drag deltas back so the caller moves its own data.

use egui::{Align, Layout, Pos2, Rect, Sense, Stroke, UiBuilder, Vec2};

use crate::atoms::{Badge, BadgeVariant, Divider, Heading, Surface, Text, Tooltip};
use crate::tokens::core;

use super::canvas::GraphCtx;
use super::handle::{anchor, HandleSpec, HandleVariant};
use super::resizer;
use super::state::ConnectDrag;
use super::{NodeId, Port, PortSide};

/// Maximum node width in world units; the body hugs its content up to this.
const NODE_MAX_W: f32 = 240.0;

/// Builder describing one node's chrome and ports. The body content is passed separately to
/// [`GraphCtx::node`]. Variants beyond `base` (group / placeholder / status) land in a later task.
/// Status indicator shown as a small badge in a node's header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeStatus {
    Ok,
    Warning,
    Error,
    Running,
}

impl NodeStatus {
    fn badge(self) -> BadgeVariant {
        match self {
            NodeStatus::Ok => BadgeVariant::Success,
            NodeStatus::Warning => BadgeVariant::Warning,
            NodeStatus::Error => BadgeVariant::Destructive,
            NodeStatus::Running => BadgeVariant::Info,
        }
    }
    fn label(self) -> &'static str {
        match self {
            NodeStatus::Ok => "ok",
            NodeStatus::Warning => "warn",
            NodeStatus::Error => "error",
            NodeStatus::Running => "running",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct NodeFrame {
    title: Option<String>,
    handles: Vec<HandleSpec>,
    size: Option<Vec2>,
    status: Option<NodeStatus>,
    appendix: Option<String>,
    tooltip: Option<String>,
    placeholder: bool,
}

impl NodeFrame {
    /// A plain node: optional titled header over a body.
    pub fn base() -> Self {
        Self::default()
    }
    /// A placeholder node — a muted, dashed-looking empty slot (no shadow, no body chrome).
    pub fn placeholder() -> Self {
        Self {
            placeholder: true,
            ..Self::default()
        }
    }
    /// Add a status badge to the header.
    pub fn status(mut self, status: NodeStatus) -> Self {
        self.status = Some(status);
        self
    }
    /// Add a muted secondary line (appendix) under the body.
    pub fn appendix(mut self, text: impl Into<String>) -> Self {
        self.appendix = Some(text.into());
        self
    }
    /// Attach a hover tooltip to the node.
    pub fn tooltip(mut self, text: impl Into<String>) -> Self {
        self.tooltip = Some(text.into());
        self
    }
    /// Set the header title (rendered as a [`Heading`] over a [`Divider`]).
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    /// Give the node an explicit world-space size (enables the resizer grip when selected).
    /// Without this the node hugs its content.
    pub fn size(mut self, size: Vec2) -> Self {
        self.size = Some(size);
        self
    }
    /// Add a port (see [`HandleSpec::input`] / [`HandleSpec::output`]).
    pub fn handle(mut self, spec: HandleSpec) -> Self {
        self.handles.push(spec);
        self
    }
    /// Convenience: add an input port with the given id.
    pub fn input(self, id: u32) -> Self {
        self.handle(HandleSpec::input(id))
    }
    /// Convenience: add an output port with the given id.
    pub fn output(self, id: u32) -> Self {
        self.handle(HandleSpec::output(id))
    }
}

/// Outcome of emitting one node.
#[derive(Clone, Copy, Debug)]
pub struct NodeResult {
    /// The node body was clicked this frame.
    pub clicked: bool,
    /// World-space move delta applied this frame (the caller commits it), if dragged.
    pub dragged: Option<Vec2>,
    /// The node's rect in scene (world) coordinates.
    pub rect: Rect,
}

impl GraphCtx<'_> {
    /// Emit one node at world position `world_pos`. `body` draws the node's content with a normal
    /// [`Ui`](egui::Ui) (already inside the scene transform). Draws the node's handles, records
    /// their positions for edge/connection anchoring, and returns the interaction result.
    pub fn node(
        &mut self,
        id: NodeId,
        world_pos: Pos2,
        frame: NodeFrame,
        body: impl FnOnce(&mut egui::Ui),
    ) -> NodeResult {
        let egui_id = self.ui.make_persistent_id(("graph_node", id.0));
        let selected = self.selection.contains(&id);

        // Child ui anchored at the world position, hugging content up to NODE_MAX_W.
        let max = Rect::from_min_size(world_pos, Vec2::new(NODE_MAX_W, f32::INFINITY));
        let mut child = self.ui.new_child(
            UiBuilder::new()
                .max_rect(max)
                .layout(Layout::top_down(Align::Min)),
        );

        let mut surface = Surface::new().selected(selected).id_source(egui_id);
        if frame.placeholder {
            surface = surface.fill_none().border_strong();
        } else {
            surface = surface.elevated();
        }
        let inner = surface.show(&mut child, |ui| {
            if let Some(sz) = frame.size {
                ui.set_min_size(sz);
                ui.set_max_width(sz.x);
            } else {
                ui.set_max_width(NODE_MAX_W);
            }
            if let Some(title) = &frame.title {
                ui.horizontal(|ui| {
                    Heading::new(title.clone()).heading().show(ui);
                    if let Some(status) = frame.status {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            Badge::new(status.label())
                                .variant(status.badge())
                                .sm()
                                .dot()
                                .show(ui);
                        });
                    }
                });
                Divider::horizontal().show(ui);
            }
            body(ui);
            if let Some(text) = &frame.appendix {
                Divider::horizontal().show(ui);
                Text::new(text.clone()).caption().muted().show(ui);
            }
        });

        let rect = inner.response.rect;

        // Drag + click interaction over the node body (claims the gesture before the Scene
        // background, so dragging a node moves it instead of panning).
        let mut resp = self
            .ui
            .interact(rect, egui_id.with("interact"), Sense::click_and_drag());
        if let Some(tip) = &frame.tooltip {
            resp = Tooltip::new(tip.clone()).show(resp);
        }

        let mut dragged = None;
        if resp.dragged_by(egui::PointerButton::Primary) {
            // Inside the Scene's transformed layer, `drag_delta` is already in scene (world)
            // coordinates — no zoom division.
            let world_delta = resp.drag_delta();
            if world_delta != Vec2::ZERO {
                // Dragging a node that's part of a multi-selection moves the whole group.
                if self.selection.len() > 1 && self.selection.contains(&id) {
                    let group: Vec<NodeId> = self.selection.iter().copied().collect();
                    for n in group {
                        self.node_moved.push((n, world_delta));
                    }
                } else {
                    self.node_moved.push((id, world_delta));
                }
                dragged = Some(world_delta);
            }
        }
        if resp.clicked() {
            self.clicked_node = Some(id);
            self.click_additive = self.ui.input(|i| i.modifiers.shift);
        }

        self.node_rects.push((id, rect));
        self.content_bounds = Some(match self.content_bounds {
            Some(b) => b.union(rect),
            None => rect,
        });

        self.draw_handles(id, rect, &frame.handles);

        // Resizer grip — only for sized nodes, only while selected.
        if frame.size.is_some() && self.selection.contains(&id) {
            resizer::paint(self.ui.painter(), rect, &self.tokens);
            let grip = resizer::grip_rect(rect);
            let rresp = self
                .ui
                .interact(grip, egui_id.with("resize"), Sense::drag());
            if rresp.dragged_by(egui::PointerButton::Primary) {
                let d = rresp.drag_delta();
                if d != Vec2::ZERO {
                    self.node_resized.push((id, d));
                }
            }
        }

        NodeResult {
            clicked: resp.clicked(),
            dragged,
            rect,
        }
    }

    /// Paint a node's handles and run their connect-drag interaction.
    fn draw_handles(&mut self, node: NodeId, rect: Rect, handles: &[HandleSpec]) {
        let in_count = handles.iter().filter(|h| h.side == PortSide::In).count();
        let out_count = handles.len() - in_count;
        let (mut in_i, mut out_i) = (0usize, 0usize);
        let r = self.tokens.handle_radius;

        for spec in handles {
            let (index, count) = match spec.side {
                PortSide::In => {
                    let v = (in_i, in_count);
                    in_i += 1;
                    v
                }
                PortSide::Out => {
                    let v = (out_i, out_count);
                    out_i += 1;
                    v
                }
            };
            let pos = anchor(rect, spec.side, index, count);
            let port = Port {
                node,
                port: spec.id,
                side: spec.side,
            };

            // Paint: filled circle + border ring.
            self.ui
                .painter()
                .circle_filled(pos, r, self.tokens.handle_fill);
            self.ui.painter().circle_stroke(
                pos,
                r,
                Stroke::new(core::BORDER_THIN, self.tokens.handle_border),
            );

            // Labeled variant: a caption just inside the node, beside the dot.
            if let HandleVariant::Labeled(text) = spec.variant {
                let w = 72.0;
                let x = match spec.side {
                    PortSide::In => pos.x + r * 2.0,
                    PortSide::Out => pos.x - r * 2.0 - w,
                };
                let area = Rect::from_min_size(Pos2::new(x, pos.y - 9.0), Vec2::new(w, 18.0));
                let layout = match spec.side {
                    PortSide::In => Layout::left_to_right(Align::Center),
                    PortSide::Out => Layout::right_to_left(Align::Center),
                };
                let mut c = self
                    .ui
                    .new_child(UiBuilder::new().max_rect(area).layout(layout));
                Text::new(text).caption().muted().show(&mut c);
            }

            self.handle_positions.push((port, pos));

            if !spec.connectable {
                continue;
            }

            // Connect-drag interaction.
            let hit = Rect::from_center_size(pos, Vec2::splat(r * 2.0));
            let hid =
                self.ui
                    .make_persistent_id(("graph_port", node.0, spec.id.0, spec.side as u8));
            let hresp = self.ui.interact(hit, hid, Sense::click_and_drag());

            // Highlight on hover / while connecting.
            if hresp.hovered() || self.connect.is_some_and(|c| c.from == port) {
                self.ui.painter().circle_stroke(
                    pos,
                    r,
                    Stroke::new(core::BORDER_FOCUS, self.tokens.edge_selected),
                );
            }

            if hresp.drag_started() {
                self.connect = Some(ConnectDrag {
                    from: port,
                    from_world: pos,
                    cursor_world: pos,
                });
            }
            if let Some(active) = self.connect.as_mut() {
                if active.from == port && hresp.dragged() {
                    if let Some(p) = self.ui.ctx().pointer_latest_pos() {
                        active.cursor_world = self.to_global.inverse() * p;
                    }
                }
            }
            if hresp.drag_stopped() && self.connect.is_some_and(|c| c.from == port) {
                let release = self
                    .ui
                    .ctx()
                    .pointer_latest_pos()
                    .map(|p| self.to_global.inverse() * p)
                    .unwrap_or(pos);
                self.connect_release = Some(release);
            }
        }
    }
}
