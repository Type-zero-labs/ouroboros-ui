# GraphTokens

> **Layer:** graph · **Path:** `src/graph/tokens.rs` · **Exports:** `GraphTokens`

The single resolve point for everything the `graph` layer paints. A flat `Copy` struct of the exact resolved paint values (grid, edge, handle, selection, marquee, minimap), built from a [`Theme`](../../theming.md) (colors) and `core` (geometry). It mirrors the `ButtonTokens` pattern: the paint-tier modules (`grid`, `edge`, `handle`, …) read from a resolved `GraphTokens` instead of touching `Theme` fields or `core` colors ad-hoc — which keeps the `no_raw_values` [guard](../../guards.md) green and gives one place to retune the graph's look.

## Design

- **Purpose / when to use.** Resolve once per frame at the top of `GraphView::show` and thread the value through the paint helpers. Read it inside the emit closure via `GraphCtx::tokens()`. Never read `Theme`/`core` directly in graph paint code — go through `GraphTokens`.
- **Anatomy.** A `#[derive(Clone, Copy, Debug)]` struct of 17 leaf fields in six groups (grid / edges / handles / node selection / marquee / minimap). No nested structs, no methods beyond the two constructors.
- **Color vs geometry split.** Every **color** is a pure `Theme` token; every **size/radius/width** is a `core::*` constant. The one synthesized value is `marquee_fill`, the focus ring re-tinted translucent via `core::tint(theme.ring, core::MARQUEE_ALPHA)`.

### Field → source map

**Background grid**

| Field | Type | Source |
|---|---|---|
| `grid_dot` | `Color32` | `theme.border` |
| `grid_dot_radius` | `f32` | `core::GRID_DOT_RADIUS` |
| `grid_spacing` | `f32` | `core::GRID_SPACING` |

**Edges (wires)**

| Field | Type | Source |
|---|---|---|
| `edge` | `Color32` | `theme.muted_foreground` |
| `edge_hover` | `Color32` | `theme.primary` |
| `edge_selected` | `Color32` | `theme.ring` |
| `edge_width` | `f32` | `core::EDGE_WIDTH` |
| `edge_hit_radius` | `f32` | `core::EDGE_HIT_RADIUS` |

**Handles (ports)**

| Field | Type | Source |
|---|---|---|
| `handle_fill` | `Color32` | `theme.primary` |
| `handle_border` | `Color32` | `theme.border_strong` |
| `handle_radius` | `f32` | `core::HANDLE_RADIUS` |
| `handle_hit_radius` | `f32` | `core::HANDLE_RADIUS * 2.0` |

**Node selection**

| Field | Type | Source |
|---|---|---|
| `node_selected_ring` | `Color32` | `theme.ring` |

**Box-select marquee**

| Field | Type | Source |
|---|---|---|
| `marquee_fill` | `Color32` | `core::tint(theme.ring, core::MARQUEE_ALPHA)` (translucent ring) |
| `marquee_border` | `Color32` | `theme.ring` |

**Minimap**

| Field | Type | Source |
|---|---|---|
| `minimap_node` | `Color32` | `theme.muted_foreground` |
| `minimap_view` | `Color32` | `theme.ring` |

## API

| Method | Signature | Effect |
|---|---|---|
| `resolve` | `fn resolve(theme: &Theme) -> Self` | Map a `Theme` (+ `core` geometry) onto all paint values. |
| `get` | `fn get(ui: &egui::Ui) -> Self` | Convenience: resolve straight from the theme installed in `ui` (`Self::resolve(&Theme::get(ui))`). |

All fields are public and the struct is `Copy`, so pass it by value freely.

## Usage

```rust
use ouroboros_ui::graph::GraphTokens;

// Resolve once (the canvas does this internally each frame):
let gt = GraphTokens::get(ui);            // or GraphTokens::resolve(&theme);

// Paint-tier helper reading resolved values:
painter.circle_filled(handle_pos, gt.handle_radius, gt.handle_fill);
painter.line_segment([a, b], egui::Stroke::new(gt.edge_width, gt.edge_selected));

// Inside the GraphView::show closure:
GraphView::new("g").show(ui, |g| {
    let t = g.tokens();
    let hit = t.edge_hit_radius / g.scale().max(f32::EPSILON); // screen-px hit → world
    // ...
});
```

## Composition / Notes

- **Two tiers, one token source.** Both the paint tier (viewport/grid/edge/handle/resizer) and the compose tier (node/controls/minimap) draw only through `GraphTokens` (colors) and `core::*` (sizes). This is the layer invariant: graph is the one place outside `atoms` that paints, but every value still flows through a token.
- **Hit radii vs draw radii.** `handle_hit_radius` is `2×` the draw radius; `edge_hit_radius` is a screen-px grab distance that callers divide by zoom to test in world space.
- **Retuning.** Change the graph's look in exactly one place — `GraphTokens::resolve`. Adding a new paint value means a new field here plus its `core`/`Theme` source, never a literal in a paint module.
- Foundation: [tokens](../../tokens.md) · [theming](../../theming.md) · [architecture](../../architecture.md) · [guards](../../guards.md). Layer overview: [README](./README.md).
