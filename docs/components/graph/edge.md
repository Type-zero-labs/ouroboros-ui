# EdgeStyle · EdgeResult

> **Layer:** graph (paint-tier) · **Path:** `src/graph/edge.rs` · **Exports:** `EdgeStyle`, `EdgeResult`

An edge is a **cubic-bezier wire** between two ports. It is **paint-tier**: it samples a bezier between the two handle anchors and pushes a token-colored polyline into the canvas's reserved under-node slot (`edge_shapes`), so wires always draw *beneath* nodes. Control points leave each port perpendicular to its side (horizontal flow: `Out` → +x, `In` → −x), so wires fan out cleanly even when nodes overlap vertically.

Edges carry no `egui::Response` — they are too thin for a hit rect. Hover/selection are hit-tested **geometrically** against the cursor in scene space, using a grab radius held constant in screen pixels (the token radius is divided by the current zoom). Clicks select the wire and report it as an intent.

## Design

- **Purpose** — connect an output port to an input port. A connection always runs `Out` → `In` (see [identity](./identity.md)).
- **Anatomy** — four control points `[a, c1, c2, b]` where `c1 = a + side_dir(a) * reach`, `c2 = b + side_dir(b) * reach`, and `reach = max(|a.x − b.x| * 0.5, MIN_REACH=40.0)` so even short edges bow. The curve is flattened into `SAMPLES + 1 = 25` points for both drawing and hit-testing. Midpoint decorations (button/label) anchor at `t = 0.5`.
- **Anchoring** — endpoints are resolved via `handle_pos(port)` against positions recorded by [`node`](./node.md). If either port has no recorded handle, `edge` returns `EdgeResult::default()` and paints nothing — emit nodes first.
- **Width** — `edge_width` token normally; `core::EDGE_WIDTH + 0.5` when hovered or selected.

### Variants / states (`EdgeStyle`)

| Variant | Behavior |
|---------|----------|
| `EdgeStyle::Default` *(default)* | A plain wire. |
| `EdgeStyle::Animated` | A dot travels along the wire (`i.time`-driven, `t = time % 1.0`), drawn on top; requests repaint each frame. |
| `EdgeStyle::WithButton` | A small ghost icon-button (phosphor `X`, an "x" delete affordance) at the midpoint; click surfaces via `EdgeResult::button_clicked`. |
| `EdgeStyle::WithLabel` | A `Badge` ("edge") at the midpoint. |

`EdgeStyle` is `Clone + Copy + Debug + Default + PartialEq + Eq`.

Color state: `edge_selected` when selected, else `edge_hover` when hovered, else `edge`.

### Tokens consumed

From [`GraphTokens`](./tokens.md): `edge`, `edge_hover`, `edge_selected`, `edge_width`, `edge_hit_radius` (geometric grab radius, screen-constant), `handle_hit_radius` (sizes the `WithButton` icon-button). Geometry from `core::EDGE_WIDTH`. The midpoint atoms (`Button`, `Badge`) carry their own foundation [tokens](../../tokens.md).

## API

### Emit method (on `GraphCtx`)

```rust
impl GraphCtx<'_> {
    pub fn edge(&mut self, from: Port, to: Port, style: EdgeStyle) -> EdgeResult
}
```

Emits a bezier edge from port `from` to port `to`. Anchored on the ports' handles, so it must be emitted **after both nodes** in the same `show` closure. Runs the geometric hover/click hit-test, records selection into `edge_selection` and clicks into `edge_clicked` (surfaced via [`GraphResponse::edge_clicked`](./canvas.md)), pushes the wire (and any animated dot) into the under-node `edge_shapes` slot, and renders any midpoint decoration. Returns `EdgeResult`. If either endpoint handle is unknown, returns `EdgeResult::default()`.

### `EdgeResult` (read-back)

| Field | Type | Meaning |
|-------|------|---------|
| `hovered` | `bool` | Cursor is within the grab radius of the wire this frame. |
| `selected` | `bool` | This `(from, to)` is the current `edge_selection`. |
| `clicked` | `bool` | The wire was clicked this frame (selects it). |
| `button_clicked` | `bool` | The `WithButton` midpoint button was clicked this frame. |

`EdgeResult` is `Clone + Copy + Debug + Default`.

## Usage

```rust
use ouroboros_ui::graph::{GraphView, NodeFrame, EdgeStyle, NodeId, PortId, Port, PortSide};

GraphView::new("wires").show(ui, |g| {
    // Nodes first so their handle positions are recorded.
    g.node(NodeId(1), egui::pos2(20.0, 40.0),
        NodeFrame::base().title("Source").output(1), |_ui| {});
    g.node(NodeId(2), egui::pos2(360.0, 40.0),
        NodeFrame::base().title("Sink").input(0), |_ui| {});

    // Then the edge: Out → In.
    let e = g.edge(
        Port { node: NodeId(1), port: PortId(1), side: PortSide::Out },
        Port { node: NodeId(2), port: PortId(0), side: PortSide::In },
        EdgeStyle::WithButton,
    );
    if e.button_clicked {
        // caller deletes the connection from its own model
    }
});
```

## Composition / Notes

- **Paint-tier.** Edges push raw `Shape`s into `edge_shapes`; they do not allocate egui widgets except the optional midpoint `Button` / `Badge` for the decorated variants.
- **Drawn under nodes.** The canvas flushes `edge_shapes` into a layer beneath the nodes at scope end, so wires never occlude node bodies regardless of emit order — but endpoints still require their nodes to have been emitted earlier so the anchors exist.
- **Geometric hit-test.** No `Response`. Hover uses `dist_to_polyline(samples, cursor) <= edge_hit_radius / zoom`; click on hover sets selection. To delete a wire, watch `EdgeResult::button_clicked` (WithButton) or `GraphResponse::edge_clicked` and remove it from caller data — the library never deletes connections itself (it only reports `delete_edge` / `edge_clicked` intents).
- **Animated** edges request a repaint every frame; use sparingly.
- See also: [node](./node.md), [handle](./handle.md), [identity](./identity.md), [layer README](./README.md), [guards](../../guards.md).
