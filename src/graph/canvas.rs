//! `GraphView` — the canvas widget and its per-frame emit scope, built on [`egui::Scene`].
//!
//! The single public entry point. The caller describes its nodes/edges inside a `show` closure
//! every frame; `Scene` handles pan/zoom and scales the real DS widgets uniformly (n8n-style),
//! and the library returns intents in [`GraphResponse`]. The library owns only
//! [`GraphViewState`] (in egui memory, as a `scene_rect` + transient interaction state); the
//! caller owns the data.
//!
//! Everything is drawn in **scene (world) coordinates** inside Scene's transformed sublayer.
//! Edges and the pending connect-wire are resolved at scope end (once every handle position is
//! known) and painted *under* the nodes via a reserved [`Painter::add`] slot — independent of
//! the order the caller interleaves `node`/`edge` calls.

use std::collections::HashSet;
use std::hash::Hash;

use egui::emath::TSTransform;
use egui::{DragPanButtons, Pos2, Rect, Scene, Sense, Shape, Stroke, StrokeKind, UiBuilder, Vec2};

use super::controls;
use super::grid;
use super::minimap;
use super::state::{ConnectDrag, GraphViewState};
use super::tokens::GraphTokens;
use super::{Connection, NodeId, NodeKindId, Port, PortSide};
use crate::tokens::core;
use crate::Theme;

/// Lower / upper zoom bounds for the canvas (Scene `zoom_range`). Allows zooming in past 1:1
/// (n8n-like "everything gets big"), at the cost of slight raster blur on text.
const MIN_ZOOM: f32 = 0.2;
const MAX_ZOOM: f32 = 4.0;
/// Canvas corner rounding.
const CANVAS_RADIUS: u8 = core::RADIUS_LG as u8;

/// Intents produced by one frame of the graph. The caller inspects these and commits the ones
/// it cares about to its own model. Everything defaults to "nothing happened".
#[derive(Clone, Debug)]
pub struct GraphResponse {
    /// Background interaction (Scene's pan response — for focus / context-menu hooks).
    pub response: egui::Response,
    /// A connect-drag completed onto a valid target port.
    pub connection: Option<Connection>,
    /// An edge the caller should delete (selected edge + Delete).
    pub delete_edge: Option<(Port, Port)>,
    /// Nodes the caller should delete (selected nodes + Delete).
    pub delete_nodes: Vec<NodeId>,
    /// An edge was clicked this frame.
    pub edge_clicked: Option<(Port, Port)>,
    /// World-space move deltas to apply to nodes (the caller owns positions).
    pub node_moved: Vec<(NodeId, Vec2)>,
    /// World-space size deltas from the node resizer.
    pub node_resized: Vec<(NodeId, Vec2)>,
    /// "Create a node of this kind at this world position" (from node search).
    pub create_request: Option<(NodeKindId, Pos2)>,
    /// The current selection, mirrored out for the caller.
    pub selection: HashSet<NodeId>,
    /// The user asked to fit-to-content (controls button).
    pub fit_requested: bool,
}

/// The canvas widget. Builder; `show` runs it.
pub struct GraphView {
    id_source: egui::Id,
    size: Option<Vec2>,
    show_grid: bool,
    show_controls: bool,
    show_minimap: bool,
}

impl GraphView {
    /// New canvas with a stable id (its view state is keyed by this).
    pub fn new(id_source: impl Hash) -> Self {
        Self {
            id_source: egui::Id::new(id_source),
            size: None,
            show_grid: true,
            show_controls: false,
            show_minimap: false,
        }
    }

    /// Explicit canvas size. Defaults to all available width × a sensible height.
    pub fn size(mut self, size: Vec2) -> Self {
        self.size = Some(size);
        self
    }
    pub fn grid(mut self, on: bool) -> Self {
        self.show_grid = on;
        self
    }
    pub fn controls(mut self, on: bool) -> Self {
        self.show_controls = on;
        self
    }
    pub fn minimap(mut self, on: bool) -> Self {
        self.show_minimap = on;
        self
    }

    /// Allocate the canvas, run the Scene (pan/zoom), paint the grid, open the emit scope, and
    /// return the accumulated intents.
    pub fn show(self, ui: &mut egui::Ui, build: impl FnOnce(&mut GraphCtx)) -> GraphResponse {
        let theme = Theme::get(ui);
        let tokens = GraphTokens::resolve(&theme);
        let id = self.id_source;

        let size = self
            .size
            .unwrap_or_else(|| Vec2::new(ui.available_width(), 420.0));
        let (rect, _bg) = ui.allocate_exact_size(size, Sense::hover());

        // Canvas surface + border (screen space, behind the scene layer).
        ui.painter()
            .rect_filled(rect, CANVAS_RADIUS, theme.background);
        ui.painter().rect_stroke(
            rect,
            CANVAS_RADIUS,
            Stroke::new(core::BORDER_THIN, theme.border),
            StrokeKind::Inside,
        );

        let mut state: GraphViewState = ui.data_mut(|d| d.get_temp(id).unwrap_or_default());

        // Child ui bounded to the canvas rect, so Scene fills exactly `rect` (not all the
        // remaining scroll height) and is clipped to it.
        let mut child = ui.new_child(UiBuilder::new().max_rect(rect).layout(*ui.layout()));
        child.set_clip_rect(rect);

        // Pan with middle / right drag only — primary drag is reserved for node move, selection
        // and connect, so it never fights the Scene's background pan (which would double-move).
        let scene = Scene::new()
            .zoom_range(egui::Rangef::new(MIN_ZOOM, MAX_ZOOM))
            .drag_pan_buttons(DragPanButtons::MIDDLE | DragPanButtons::SECONDARY);
        let show_grid = self.show_grid;
        let connect_in = state.connect;

        let inner = scene.show(&mut child, &mut state.scene_rect, |sui| {
            let to_global = sui
                .ctx()
                .layer_transform_to_global(sui.layer_id())
                .unwrap_or(TSTransform::IDENTITY);

            // Grid in scene coords, culled when on-screen spacing is too dense.
            if show_grid && tokens.grid_spacing * to_global.scaling >= grid::MIN_DOT_SPACING {
                grid::paint(
                    sui.painter(),
                    sui.clip_rect(),
                    tokens.grid_spacing,
                    tokens.grid_dot_radius,
                    tokens.grid_dot,
                );
            }

            // Reserve the edge layer (painted under nodes, filled at scope end).
            let edge_layer = sui.painter().add(Shape::Noop);

            let mut ctx = GraphCtx {
                ui: sui,
                tokens,
                to_global,
                selection: state.selection.clone(),
                handle_positions: Vec::new(),
                node_rects: Vec::new(),
                content_bounds: None,
                edge_shapes: Vec::new(),
                edge_selection: state.edge_selection,
                edge_clicked: None,
                node_moved: Vec::new(),
                node_resized: Vec::new(),
                clicked_node: None,
                click_additive: false,
                connect: connect_in,
                connect_release: None,
                connection: None,
                create_request: None,
            };
            build(&mut ctx);

            let GraphCtx {
                ui: sui,
                handle_positions,
                node_rects,
                content_bounds,
                edge_shapes,
                edge_selection,
                edge_clicked,
                node_moved,
                node_resized,
                clicked_node,
                click_additive,
                connect,
                connect_release,
                connection: conn0,
                create_request,
                ..
            } = ctx;

            // Resolve a completed connect-drag against all known handle positions.
            let mut connection = conn0;
            let mut connect_next = connect;
            if let Some(release) = connect_release {
                if let Some(active) = connect {
                    if let Some(target) = resolve_target(
                        &handle_positions,
                        &node_rects,
                        active.from,
                        release,
                        tokens.handle_hit_radius,
                    ) {
                        connection = Some(orient(active.from, target));
                    }
                }
                connect_next = None;
            }

            // Flush accumulated edge shapes into the reserved under-node slot.
            sui.painter().set(edge_layer, Shape::Vec(edge_shapes));

            // Pending connect-wire follows the cursor on top.
            if let Some(active) = connect_next {
                sui.painter().line_segment(
                    [active.from_world, active.cursor_world],
                    Stroke::new(tokens.edge_width, tokens.edge_selected),
                );
            }

            CtxOut {
                node_moved,
                node_resized,
                clicked_node,
                click_additive,
                connection,
                connect: connect_next,
                edge_selection,
                edge_clicked,
                create_request,
                content_bounds,
                node_rects,
                scale: to_global.scaling,
            }
        });

        let egui::InnerResponse {
            inner: out,
            response: bg,
        } = inner;

        state.connect = out.connect;
        state.edge_selection = out.edge_selection;

        // Selection: a clicked node toggles (shift) or replaces; a click on truly empty space
        // (no node and no edge) clears everything.
        if let Some(n) = out.clicked_node {
            if out.click_additive {
                if !state.selection.remove(&n) {
                    state.selection.insert(n);
                }
            } else {
                state.selection.clear();
                state.selection.insert(n);
            }
            state.edge_selection = None;
        } else if bg.clicked() && out.edge_clicked.is_none() {
            state.selection.clear();
            state.edge_selection = None;
        }

        // Delete/Backspace: remove the selected edge first, else the selected nodes.
        let delete_pressed =
            ui.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace));
        let mut delete_edge = None;
        let mut delete_nodes = Vec::new();
        if delete_pressed {
            if let Some(e) = state.edge_selection.take() {
                delete_edge = Some(e);
            } else if !state.selection.is_empty() {
                delete_nodes = state.selection.drain().collect();
            }
        }

        // Floating zoom/fit controls (screen-space overlay); they mutate the scene_rect.
        let mut fit_requested = false;
        if self.show_controls {
            let percent = (out.scale * 100.0).round() as i32;
            let act = controls::show(ui, rect, percent);
            if act.zoom_in {
                state.scene_rect = scale_rect(state.scene_rect, 1.0 / ZOOM_STEP);
            }
            if act.zoom_out {
                state.scene_rect = scale_rect(state.scene_rect, ZOOM_STEP);
            }
            if act.fit {
                if let Some(b) = out.content_bounds {
                    state.scene_rect = b.expand(core::SPACE_8);
                    fit_requested = true;
                }
            }
        }

        // MiniMap (screen-space overlay above the Scene); clicking recenters the view.
        if self.show_minimap {
            if let Some(center) = minimap::show(
                ui,
                rect,
                &out.node_rects,
                out.content_bounds.unwrap_or(state.scene_rect),
                state.scene_rect,
            ) {
                let size = state.scene_rect.size();
                state.scene_rect = Rect::from_center_size(center, size);
            }
        }

        // Persist view state (Scene mutated scene_rect in place).
        ui.data_mut(|d| d.insert_temp(id, state.clone()));
        GraphResponse {
            response: bg,
            connection: out.connection,
            delete_edge,
            delete_nodes,
            edge_clicked: out.edge_clicked,
            node_moved: out.node_moved,
            node_resized: out.node_resized,
            create_request: out.create_request,
            selection: state.selection.clone(),
            fit_requested,
        }
    }
}

/// Multiplicative zoom step for the controls' + / − buttons.
const ZOOM_STEP: f32 = 1.25;

/// Scale a rect around its center (smaller rect = more zoomed in).
fn scale_rect(r: Rect, factor: f32) -> Rect {
    Rect::from_center_size(r.center(), r.size() * factor)
}

/// Orient a connection so it always runs `Out → In`, regardless of which end was dragged.
fn orient(a: Port, b: Port) -> Connection {
    if a.side == PortSide::Out {
        Connection { from: a, to: b }
    } else {
        Connection { from: b, to: a }
    }
}

/// Resolve a connect-drag release into a target port: first a precise handle hit, then — if the
/// release landed anywhere inside a different node's body — that node's nearest compatible port.
fn resolve_target(
    handles: &[(Port, Pos2)],
    node_rects: &[(NodeId, Rect)],
    from: Port,
    release: Pos2,
    hit: f32,
) -> Option<Port> {
    if let Some(p) = nearest_target(handles, from, release, hit) {
        return Some(p);
    }
    // Dropped over a node body → connect to its nearest opposite-side port.
    let (node, _) = node_rects
        .iter()
        .find(|(n, r)| *n != from.node && r.contains(release))?;
    let want = match from.side {
        PortSide::Out => PortSide::In,
        PortSide::In => PortSide::Out,
    };
    handles
        .iter()
        .filter(|(p, _)| p.node == *node && p.side == want)
        .map(|(p, pos)| (*p, pos.distance(release)))
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(p, _)| p)
}

/// Nearest opposite-side, different-node handle within `hit` screen px of `release` (scene pos).
fn nearest_target(handles: &[(Port, Pos2)], from: Port, release: Pos2, hit: f32) -> Option<Port> {
    handles
        .iter()
        .filter(|(p, _)| p.side != from.side && p.node != from.node)
        .map(|(p, pos)| (*p, pos.distance(release)))
        .filter(|(_, d)| *d <= hit)
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(p, _)| p)
}

/// Owned accumulators returned out of the scene closure.
struct CtxOut {
    node_moved: Vec<(NodeId, Vec2)>,
    node_resized: Vec<(NodeId, Vec2)>,
    clicked_node: Option<NodeId>,
    click_additive: bool,
    connection: Option<Connection>,
    connect: Option<ConnectDrag>,
    edge_selection: Option<(Port, Port)>,
    edge_clicked: Option<(Port, Port)>,
    create_request: Option<(NodeKindId, Pos2)>,
    content_bounds: Option<Rect>,
    node_rects: Vec<(NodeId, Rect)>,
    scale: f32,
}

/// The per-frame emit surface handed to the `show` closure. Drawing happens in **scene (world)
/// coordinates**: a node emitted at world `pos` lands there and Scene scales it.
pub struct GraphCtx<'a> {
    pub(crate) ui: &'a mut egui::Ui,
    pub(crate) tokens: GraphTokens,
    /// scene→screen transform (zoom + pan of the Scene layer).
    pub(crate) to_global: TSTransform,
    /// Current selection, snapshot in for read (node rings); changes flow out via `clicked_node`.
    pub(crate) selection: HashSet<NodeId>,
    /// Handle anchor positions (scene coords) recorded as nodes are emitted.
    pub(crate) handle_positions: Vec<(Port, Pos2)>,
    /// Node rects (scene coords) — for drop-on-node connection targeting.
    pub(crate) node_rects: Vec<(NodeId, Rect)>,
    /// Union of node rects (scene coords) — for fit-to-content.
    pub(crate) content_bounds: Option<Rect>,
    /// Edge shapes accumulated by `edge`, flushed under the nodes at scope end.
    pub(crate) edge_shapes: Vec<Shape>,
    /// Edge selected last frame (read for highlight; mutated on edge click).
    pub(crate) edge_selection: Option<(Port, Port)>,
    /// An edge clicked this frame.
    pub(crate) edge_clicked: Option<(Port, Port)>,
    pub(crate) node_moved: Vec<(NodeId, Vec2)>,
    pub(crate) node_resized: Vec<(NodeId, Vec2)>,
    pub(crate) clicked_node: Option<NodeId>,
    /// Shift was held on the click → additive (toggle) selection.
    pub(crate) click_additive: bool,
    pub(crate) connect: Option<ConnectDrag>,
    pub(crate) connect_release: Option<Pos2>,
    pub(crate) connection: Option<Connection>,
    pub(crate) create_request: Option<(NodeKindId, Pos2)>,
}

impl GraphCtx<'_> {
    /// The scene→screen scale (current zoom factor).
    pub fn scale(&self) -> f32 {
        self.to_global.scaling
    }
    /// The visible region in scene (world) coordinates.
    pub fn visible_rect(&self) -> Rect {
        self.ui.clip_rect()
    }
    /// The resolved graph paint tokens.
    pub fn tokens(&self) -> GraphTokens {
        self.tokens
    }
    /// Convert a screen-space delta (e.g. a `Response::drag_delta`) to a world-space delta.
    pub fn screen_delta_to_world(&self, delta: Vec2) -> Vec2 {
        delta / self.to_global.scaling
    }
    /// Convert a global screen point to a scene (world) point.
    pub fn screen_to_world(&self, screen: Pos2) -> Pos2 {
        self.to_global.inverse() * screen
    }

    /// Scene position of a port's handle this frame, if it has been emitted.
    pub(crate) fn handle_pos(&self, port: &Port) -> Option<Pos2> {
        self.handle_positions
            .iter()
            .find(|(p, _)| p == port)
            .map(|(_, pos)| *pos)
    }

    /// Current pointer position in scene (world) coordinates, if any.
    pub(crate) fn pointer_world(&self) -> Option<Pos2> {
        self.ui
            .ctx()
            .pointer_latest_pos()
            .map(|p| self.to_global.inverse() * p)
    }
}
