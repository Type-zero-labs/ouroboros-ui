# GraphView · GraphCtx · GraphResponse

> **Layer:** graph · **Path:** `src/graph/canvas.rs` · **Exports:** `GraphView`, `GraphCtx<'a>`, `GraphResponse`

The single public entry point to the node-editor. `GraphView` is a builder; its `show` method allocates the canvas, runs an [`egui::Scene`] (which owns pan/zoom and scales the real DS widgets uniformly, n8n-style), opens a per-frame emit scope, and returns a `GraphResponse` of intents. The caller describes its nodes and edges inside the `show` closure every frame using `GraphCtx`; the library owns only the view-state ([`GraphViewState`](./state.md)), the caller owns the data. Everything is drawn in **scene (world) coordinates** inside Scene's transformed sublayer.

## Design

- **Purpose / when to use.** Any reactflow-style node graph: pipeline editors, behaviour trees, dataflow. Use it whenever the caller has node + edge data it owns and wants pan/zoom, selection, drag-move, resize, connect-by-drag, delete, fit, minimap, and node-search for free.
- **Frame lifecycle.** `show` runs once per frame: resolve [`GraphTokens`](./tokens.md) → allocate `rect` + paint surface/border → load `GraphViewState` from egui memory (keyed by the builder id) → `Scene::show` against `&mut state.scene_rect` → paint grid (culled when on-screen dot spacing < `grid::MIN_DOT_SPACING`) → reserve an under-node edge layer via `Painter::add(Shape::Noop)` → build the `GraphCtx` and run the caller's closure → resolve any completed connect-drag against recorded handle positions → flush accumulated edge shapes into the reserved slot (always under nodes, regardless of caller interleaving) → draw the pending connect-wire on top → apply selection / delete / controls / minimap → persist state → return `GraphResponse`.
- **Coordinate convention.** A node emitted at world `pos` lands there; Scene scales it. `GraphCtx::scale` is the live scene→screen factor; `to_global` is the `TSTransform`.
- **Zoom range.** Canvas Scene `zoom_range` is `MIN_ZOOM = 0.2` .. `MAX_ZOOM = 4.0` (private consts).
- **Pan binding.** Scene pan is bound to `DragPanButtons::MIDDLE | SECONDARY` only; primary drag is reserved for node move / marquee / connect so it never double-moves against Scene's background pan.

### GraphResponse fields (every field)

| Field | Type | Meaning |
|---|---|---|
| `response` | `egui::Response` | Scene background interaction (pan response). Use for focus / context-menu hooks. |
| `connection` | `Option<Connection>` | A connect-drag completed onto a valid target port. Always oriented `Out → In`. |
| `delete_edge` | `Option<(Port, Port)>` | Selected edge + Delete/Backspace. Deleted before nodes. |
| `delete_nodes` | `Vec<NodeId>` | Selected nodes + Delete/Backspace (only when no edge was selected). |
| `edge_clicked` | `Option<(Port, Port)>` | An edge was clicked this frame. |
| `node_moved` | `Vec<(NodeId, Vec2)>` | World-space move deltas to apply (caller owns positions). |
| `node_resized` | `Vec<(NodeId, Vec2)>` | World-space size deltas from the node resizer. |
| `create_request` | `Option<(NodeKindId, Pos2)>` | "Create a node of this kind at this world position" (from node search). |
| `selection` | `HashSet<NodeId>` | Current selection, mirrored out (e.g. to drive per-node toolbars next frame). |
| `fit_requested` | `bool` | The user hit the controls' fit-to-content button this frame. |

### Tokens consumed

`show` resolves a [`GraphTokens`](./tokens.md) and threads it through the paint helpers (grid, edge, handle). Canvas chrome uses `Theme` directly: `theme.background` fill, `theme.border` stroke at `core::BORDER_THIN`, corner radius `core::RADIUS_LG`. Fit padding is `core::SPACE_8`.

## API

### `GraphView` (builder)

| Method | Signature | Effect |
|---|---|---|
| `new` | `fn new(id_source: impl Hash) -> Self` | Construct with a stable id; the view-state is keyed by it. |
| `size` | `fn size(self, size: Vec2) -> Self` | Explicit canvas size. Default: full available width × `420.0`. |
| `grid` | `fn grid(self, on: bool) -> Self` | Toggle the dot grid (default `true`). |
| `controls` | `fn controls(self, on: bool) -> Self` | Toggle the floating zoom/fit overlay (default `false`). |
| `minimap` | `fn minimap(self, on: bool) -> Self` | Toggle the minimap overlay (default `false`). |
| `show` | `fn show(self, ui: &mut egui::Ui, build: impl FnOnce(&mut GraphCtx)) -> GraphResponse` | Run the canvas and return intents. |

### `GraphCtx<'a>` (per-frame emit surface)

Public methods. Node/edge/toolbar emit methods (`node`, `edge`, `node_toolbar`) are added by other modules via separate `impl GraphCtx` blocks (`node.rs`, `edge.rs`, `toolbar.rs`) — see [layer README](./README.md). All fields are crate-private; interact only through these methods.

| Method | Signature | Effect |
|---|---|---|
| `scale` | `fn scale(&self) -> f32` | The scene→screen scale (current zoom factor). |
| `visible_rect` | `fn visible_rect(&self) -> Rect` | The visible region in scene (world) coordinates. |
| `tokens` | `fn tokens(&self) -> GraphTokens` | The resolved graph paint tokens. |
| `screen_delta_to_world` | `fn screen_delta_to_world(&self, delta: Vec2) -> Vec2` | Convert a screen-space delta (e.g. a `Response::drag_delta`) to a world delta. |
| `screen_to_world` | `fn screen_to_world(&self, screen: Pos2) -> Pos2` | Convert a global screen point to a scene (world) point. |

Emit methods (documented under their own pages):

| Method | Signature |
|---|---|
| `node` | `fn node(&mut self, id: NodeId, world_pos: Pos2, frame: NodeFrame, body: impl FnOnce(&mut egui::Ui)) -> NodeResult` |
| `edge` | `fn edge(&mut self, from: Port, to: Port, style: EdgeStyle) -> EdgeResult` |
| `node_toolbar` | `fn node_toolbar(&mut self, node: NodeId, content: impl FnOnce(&mut egui::Ui))` |

### `GraphResponse`

`#[derive(Clone, Debug)]`. All fields public; see the field table above. Defaults to "nothing happened" (empty/`None`).

## Usage

```rust
use ouroboros_ui::graph::{GraphView, GraphCtx, GraphResponse};
use ouroboros_ui::graph::{NodeId, PortId, Port, PortSide, EdgeStyle, NodeFrame};

// Caller owns nodes + edges; lib owns only view-state.
let resp: GraphResponse = GraphView::new("my_graph")
    .size(egui::vec2(720.0, 420.0))
    .grid(true)
    .controls(true)
    .minimap(true)
    .show(ui, |g: &mut GraphCtx| {
        // Nodes first, so their handle positions are recorded before edges resolve.
        for (id, pos, label) in &nodes {
            let frame = NodeFrame::base().title(label.clone()).input(0).output(1);
            g.node(NodeId(*id), *pos, frame, |ui| {
                Text::new("body content").muted().show(ui);
            });
        }
        // Edges after nodes (anchored on handles, drawn under the nodes).
        for (from, to) in &edges {
            g.edge(
                Port { node: NodeId(*from), port: PortId(1), side: PortSide::Out },
                Port { node: NodeId(*to),   port: PortId(0), side: PortSide::In  },
                EdgeStyle::Default,
            );
        }
    });

// Commit the intents back into caller-owned data.
for (moved_id, delta) in &resp.node_moved {
    if let Some(pos) = positions.get_mut(moved_id) { *pos += *delta; }
}
if let Some(c) = resp.connection { edges.push((c.from.node.0, c.to.node.0)); }
if let Some((from, to)) = resp.delete_edge { /* drop edge */ }
for n in &resp.delete_nodes { /* drop node + its edges */ }
```

## Composition / Notes

- **Emit order matters.** Emit nodes before edges — `edge` anchors on handle positions recorded by `node`, and returns a default (no-op) `EdgeResult` if either endpoint hasn't been emitted yet. Within a frame the *paint* order is fixed regardless of caller interleaving: edges flush into a reserved slot under the nodes.
- **Ownership.** The lib persists exactly one value per `GraphView` id in egui temp memory — `GraphViewState` (camera + selection + transient drag state). All node/edge data is the caller's; every mutation is reported as an intent in `GraphResponse`, never applied to caller data by the lib.
- **Connect resolution.** A released connect-drag is resolved at scope end: first a precise handle hit within `tokens.handle_hit_radius`, else the nearest compatible (opposite-side) port of whatever node body the release landed in. Output is oriented `Out → In`.
- **Paint tier vs compose tier.** The canvas itself is the paint shell (surface, grid, edge layer, connect-wire) plus the compose overlays (`controls`, `minimap`) it drives from `GraphView` flags. `node` / `edge` / `node_toolbar` are compose-tier and live in sibling modules.
- **Identity.** `NodeId`, `PortId`, `Port`, `PortSide`, `Connection`, `NodeKindId` — see [identity](./identity.md).
- Foundation: [architecture](../../architecture.md) · [tokens](../../tokens.md) · [theming](../../theming.md) · [guards](../../guards.md).
