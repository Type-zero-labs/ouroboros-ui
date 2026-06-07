# HandleSpec · HandleVariant

> **Layer:** graph (paint-tier) · **Path:** `src/graph/handle.rs` · **Exports:** `HandleSpec`, `HandleVariant`

A handle is a **port**: a connection anchor painted on a node's edge. It is **paint-tier** — it draws the port circle (filled disc + border ring, token-colored) and owns the connect-drag detection. Inputs anchor on a node's **left** edge, outputs on the **right**, distributed evenly down the side. `HandleSpec` is a pure declaration attached to a [`NodeFrame`](./node.md); the actual painting + interaction run inside `GraphCtx::draw_handles` (a private helper invoked by [`node`](./node.md)) — there is no public `ctx.handle(..)` emit method, handles are emitted as part of the node.

## Design

- **Purpose** — declare where wires can attach to a node and which of those attachment points accept a drag.
- **Anatomy (per handle)** — a filled circle (`handle_fill`) of radius `handle_radius`, with a `core::BORDER_THIN` border ring (`handle_border`). On hover or while it is the active connect source, an extra `core::BORDER_FOCUS` ring in `edge_selected` is drawn. The `Labeled` variant adds a muted caption `Text` (width 72, 18 high) just inside the node beside the dot — left-aligned for inputs, right-aligned for outputs.
- **Anchoring** — `anchor(rect, side, index, count)` places handle `index` of `count` at `y = rect.top() + height * (index+1)/(count+1)` (even vertical distribution) and `x = rect.left()` for `In` / `rect.right()` for `Out`. Inputs and outputs are counted and indexed independently. The computed scene-space position is recorded into `handle_positions` keyed by `Port{node, port, side}`, which is what [`edge`](./edge.md) reads back to anchor wires.
- **Connect-drag** — a connectable handle gets an interaction rect of `2 * handle_radius` square. `drag_started` opens a `ConnectDrag { from, from_world, cursor_world }`; while dragging, `cursor_world` tracks the pointer in world space (a preview wire follows it on top); `drag_stopped` records the world release point. The canvas resolves the release against all known handle positions (`handle_hit_radius`) into a [`GraphResponse::connection`](./canvas.md) (`Out` → `In`). A `fixed()` (non-connectable) handle is painted but skips all interaction.

### Variants / states (`HandleVariant`)

| Variant | Behavior |
|---------|----------|
| `HandleVariant::Base` *(default)* | Plain port circle. |
| `HandleVariant::Labeled(&'static str)` | Port circle with a caption beside it, inside the node. |

`HandleVariant` is `Clone + Copy + Debug + Default + PartialEq + Eq`.

### Tokens consumed

From [`GraphTokens`](./tokens.md): `handle_fill`, `handle_border`, `handle_radius`, `handle_hit_radius`, and `edge_selected` (hover/active-connect highlight ring). Geometry from `core::BORDER_THIN`, `core::BORDER_FOCUS`. Caption labels use foundation [tokens](../../tokens.md) via the `Text` atom.

## API

Handles are declared on a `NodeFrame` (`.handle(..)`, `.input(..)`, `.output(..)`); there is no standalone emit method on `GraphCtx`. Painting/hit-testing happens inside `GraphCtx::node`.

### `HandleSpec` fields

| Field | Type | Meaning |
|-------|------|---------|
| `id` | `PortId` (`u32` newtype) | Port id, unique within the node's side. |
| `side` | `PortSide` | `In` (left edge) or `Out` (right edge). |
| `connectable` | `bool` | Whether the port accepts a connect-drag. |
| `variant` | `HandleVariant` | Visual style (`Base` / `Labeled`). |

`HandleSpec` is `Clone + Copy + Debug`.

### `HandleSpec` builder

| Method | Effect |
|--------|--------|
| `HandleSpec::input(id: u32)` | Input port on the left edge; `connectable = true`, `Base`. |
| `HandleSpec::output(id: u32)` | Output port on the right edge; `connectable = true`, `Base`. |
| `.fixed()` | Mark the port non-connectable (decorative only). |
| `.label(text: &'static str)` | Switch to `HandleVariant::Labeled(text)` — caption beside the dot. |

## Usage

```rust
use ouroboros_ui::graph::{GraphView, NodeFrame, HandleSpec, NodeId};

GraphView::new("ports").show(ui, |g| {
    g.node(
        NodeId(1),
        egui::pos2(20.0, 20.0),
        NodeFrame::base()
            .title("Mixer")
            .handle(HandleSpec::input(0).label("audio in"))
            .handle(HandleSpec::input(1).label("gain"))
            .handle(HandleSpec::output(2).label("out"))
            .handle(HandleSpec::output(3).fixed()), // decorative, no connect
        |ui| { Text::new("body").muted().show(ui); },
    );
});
```

Convenience shorthands on `NodeFrame` (`.input(0)` / `.output(1)`) expand to `Base` handles.

## Composition / Notes

- **Paint-tier**, but `Labeled` borrows the `Text` atom for its caption — still no hand-rolled values (all geometry/colors via tokens / `core`), keeping the `no_raw_values` [guard](../../guards.md) green.
- **Feeds the wire system.** Every handle records its world position into `handle_positions`; [`edge`](./edge.md) anchors on those, and connect-drag releases hit-test against them (`handle_hit_radius`) to produce a `connection`. So handle ids/sides are the join key between nodes and edges — keep them stable in caller data.
- **Connection direction is enforced downstream:** a connect-drag is resolved to `Out` → `In` by the canvas, regardless of which end the drag started from.
- `fixed()` handles render but never start a connect nor accept a release — use for read-only / display-only ports.
- See also: [node](./node.md), [edge](./edge.md), [identity](./identity.md) (`PortId`, `PortSide`, `Port`, `Connection`), [layer README](./README.md), [canvas](./canvas.md).
