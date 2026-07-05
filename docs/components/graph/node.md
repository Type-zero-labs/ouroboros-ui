# NodeFrame · NodeResult · NodeStatus

> **Layer:** graph (compose-tier) · **Path:** `src/graph/node.rs` · **Exports:** `NodeFrame`, `NodeResult`, `NodeStatus`

A node is a draggable, selectable box drawn from the DS [`Surface`](../atoms/surface.md) atom hosting an arbitrary caller closure of DS widgets. It is **compose-tier**: it does not paint inline shapes for its body — it reuses `Surface` (card fill, border, radius, elevation, selection ring) plus atoms (`Heading`, `Divider`, `Badge`, `Text`, `Tooltip`) and lets the caller fill the body with a normal `egui::Ui`. Because the whole node lives inside the [`GraphView`](./canvas.md) scene layer, it scales with zoom for free.

Ports (handles) painted on the node edges are delegated to the paint-tier [`handle`](./handle.md) helpers. Position is in **world coordinates**; the library reports drag deltas back via [`GraphResponse`](./canvas.md) so the caller moves its own data — the library never mutates node positions.

## Design

- **Purpose** — emit one logical node of a node-graph. Use whenever you need a draggable box with optional header, status badge, body, footer and ports.
- **Anatomy** (top → bottom, inside one `Surface`):
  - Header (optional): `Heading` (title) + right-aligned status `Badge` (dot, `sm`), followed by a horizontal `Divider`.
  - Body: the caller's `body(&mut egui::Ui)` closure.
  - Appendix (optional): horizontal `Divider` + muted caption `Text`.
  - Handles: drawn on the left (`In`) / right (`Out`) edges by `draw_handles` (see [handle.md](./handle.md)).
  - Resizer grip: painted only when the node has an explicit `size(..)` **and** is selected.
- **Surface mode** — `placeholder()` nodes use `Surface::fill_none().border_strong()` (muted empty slot, no shadow/body chrome); all others use `Surface::elevated()`. Selection ring is driven by `Surface::selected(..)`.
- **Sizing** — without `size(..)`, the body hugs its content up to a max width of `NODE_MAX_W = 240.0` world units. With `size(..)`, the node takes a fixed world-space size and gains the resizer grip when selected.

### Variants / states (`NodeStatus`)

| Variant | Badge variant | Header label |
|---------|---------------|--------------|
| `NodeStatus::Ok` | `Success` | `ok` |
| `NodeStatus::Warning` | `Warning` | `warn` |
| `NodeStatus::Error` | `Destructive` | `error` |
| `NodeStatus::Running` | `Info` | `running` |

Status renders only when `NodeFrame::title(..)` is also set (the badge lives in the header row).

### Tokens consumed

Node chrome inherits from `Surface` / atoms (foundation [tokens](../../tokens.md)). Directly from [`GraphTokens`](./tokens.md) via `draw_handles` / `resizer`: `handle_radius`, `handle_fill`, `handle_border`, `handle_hit_radius`, `edge_selected` (port hover/connect highlight), plus `node_selected_ring` (through the resizer/selection path). Geometry constants come from `core::*` (`BORDER_THIN`, `BORDER_FOCUS`).

## API

### Emit method (on `GraphCtx`)

```rust
impl GraphCtx<'_> {
    pub fn node(
        &mut self,
        id: NodeId,
        world_pos: Pos2,
        frame: NodeFrame,
        body: impl FnOnce(&mut egui::Ui),
    ) -> NodeResult
}
```

Emits one node at world position `world_pos`; `body` draws content with a normal `egui::Ui` already inside the scene transform. Records handle positions for edge/connection anchoring, claims the primary-drag gesture (so dragging a node moves it instead of panning the Scene), pushes any drag delta into `node_moved` (the whole multi-selection moves together when the dragged node is part of a selection of >1), records clicks into the selection machinery, and returns `NodeResult`. Drag deltas are already in world coordinates (no zoom division).

### `NodeFrame` builder

| Method | Effect |
|--------|--------|
| `NodeFrame::base()` | Plain node: optional titled header over a body (default). |
| `NodeFrame::placeholder()` | Muted dashed-looking empty slot — no shadow, no body chrome (`fill_none` + `border_strong`). |
| `.title(impl Into<String>)` | Header title, rendered as a `Heading` over a `Divider`. |
| `.status(NodeStatus)` | Status badge in the header (requires a title). |
| `.appendix(impl Into<String>)` | Muted secondary line (caption) under the body, after a `Divider`. |
| `.tooltip(impl Into<String>)` | Hover tooltip on the node body (`Tooltip` atom). |
| `.size(Vec2)` | Explicit world-space size; enables the resizer grip when selected. Without it the node hugs content. |
| `.handle(HandleSpec)` | Add a port. See [`HandleSpec`](./handle.md). |
| `.input(u32)` | Convenience: `.handle(HandleSpec::input(id))`. |
| `.output(u32)` | Convenience: `.handle(HandleSpec::output(id))`. |

`NodeFrame` is `Clone + Debug + Default` (default == `base()`).

### `NodeResult` (read-back)

| Field | Type | Meaning |
|-------|------|---------|
| `clicked` | `bool` | The node body was clicked this frame. |
| `dragged` | `Option<Vec2>` | World-space move delta applied this frame (caller commits it), `None` if not dragged. |
| `rect` | `Rect` | The node's rect in scene (world) coordinates. |

`NodeResult` is `Clone + Copy + Debug`. Note: the actual position commit happens through `GraphResponse::node_moved` at scope end — `dragged` is the same delta, surfaced inline for convenience.

## Usage

```rust
use ouroboros_ui::graph::{GraphView, NodeFrame, NodeStatus, NodeId, HandleSpec};

GraphView::new("my_graph").show(ui, |g| {
    let res = g.node(
        NodeId(1),
        egui::pos2(20.0, 20.0),
        NodeFrame::base()
            .title("Status")
            .status(NodeStatus::Running)
            .appendix("last run 2s ago")
            .tooltip("a node with a status badge")
            .handle(HandleSpec::input(0).label("in"))
            .handle(HandleSpec::output(1).label("out")),
        |ui| {
            Text::new("body content").muted().show(ui);
        },
    );
    if res.clicked {
        // react to selection
    }
});
```

A placeholder slot and a fixed-size (resizable) node:

```rust
g.node(NodeId(2), egui::pos2(300.0, 20.0),
    NodeFrame::placeholder().title("Placeholder"),
    |ui| { Text::new("drop a node here").muted().show(ui); });

g.node(NodeId(3), egui::pos2(20.0, 200.0),
    NodeFrame::base().title("Sized").size(egui::vec2(180.0, 96.0)).input(0),
    |ui| { Text::new("fixed size (select to resize)").muted().show(ui); });
```

## Composition / Notes

- **Compose-tier, not paint-tier.** The body never hand-rolls shapes — it composes `Surface` + atoms. Ports/resizer are the only painted parts and those route through paint-tier helpers ([handle.md](./handle.md)) and `resizer`.
- **Emit nodes before edges.** `node(..)` records each handle's world position into `handle_positions`; [`edge`](./edge.md) looks those up to anchor wires, so all nodes a wire touches must be emitted first in the same `show` closure.
- **Intents, not mutation.** Drags push `(NodeId, Vec2)` into `node_moved`; resizes push into `node_resized`; clicks feed the selection. The caller reads these from [`GraphResponse`](./canvas.md) after `show` returns and commits them to its own data model.
- **Multi-select drag** moves every selected node by the same delta.
- See also: [identity](./identity.md) (`NodeId`, `Port`, `PortSide`), [layer README](./README.md), [guards](../../guards.md) (`no_raw_values` — graph paints only via tokens).
