# Graph extras — internal / advanced support pieces

> **Layer:** graph · **Paths:** `src/graph/{grid,resizer,minimap,toolbar,controls}.rs`

These are the internal/advanced support pieces of the graph layer — mostly module-level `show`/`paint` helpers (plus one `GraphCtx` method) that the canvas wires up for you. **None of them has a top-level re-export in `mod.rs`**; reach them through their modules (`ouroboros_ui::graph::{grid, resizer, minimap, toolbar, controls}`) only when building a custom canvas. In normal use you never call them directly — [`GraphView`](./canvas.md) toggles them via `.grid(bool)`, `.controls(bool)`, `.minimap(bool)`, and `GraphCtx::node_toolbar`, and surfaces their results on [`GraphResponse`](./canvas.md).

Several have `pub(crate)` visibility (`resizer`, `minimap`, `controls::show`), i.e. they are crate-internal; `grid` and `controls::ControlsAction` are `pub`. All obey the layer's **paint-but-token** invariant — colours from [`GraphTokens`](./tokens.md)/[`Theme`](../../tokens.md), sizes from `tokens::core` — checked by [guards](../../guards.md). See the [layer README](./README.md) for the paint-vs-compose tier split and [identity](./identity.md) for `NodeId`/`Pos2`/`Vec2` vocabulary.

---

## Grid

**What it is.** The canvas backdrop dot-grid. Paints dots on a `spacing`-aligned lattice in **scene (world) coordinates**, so it pans and zooms uniformly with the nodes (n8n-style), rather than as a fixed screen overlay.

**Tier.** paint. Touches the painter directly, but every value comes from [`GraphTokens`].

**Module:** `ouroboros_ui::graph::grid` (`pub`).

```rust
/// Below this on-screen dot spacing (px) the grid is too dense to read — caller skips it.
pub const MIN_DOT_SPACING: f32 = 6.0;

pub fn paint(
    painter: &egui::Painter,
    visible: egui::Rect,   // scene-space rect to cover (e.g. ui.clip_rect() inside the Scene)
    spacing: f32,          // GraphTokens.grid_spacing
    dot_radius: f32,       // GraphTokens.grid_dot_radius
    color: egui::Color32,  // GraphTokens.grid_dot
);
```

`paint` returns early if `spacing <= 0.0`. It snaps the first dot to the lattice (`floor(visible.left()/spacing)*spacing`, same for top) and fills `circle_filled` dots across the visible rect. `dot_radius`/`color` are scene-space token values.

**How the canvas uses it.** Inside the Scene closure, in scene coords, the canvas culls before painting:

```rust
if show_grid && tokens.grid_spacing * to_global.scaling >= grid::MIN_DOT_SPACING {
    grid::paint(sui.painter(), sui.clip_rect(),
                tokens.grid_spacing, tokens.grid_dot_radius, tokens.grid_dot);
}
```

i.e. when the **on-screen** spacing (`grid_spacing × Scene zoom`) drops below `MIN_DOT_SPACING`, the grid is skipped entirely (too dense to be useful). Enabled via `GraphView::new(id).grid(true)`.

---

## Resizer

**What it is.** A bottom-right corner grip for resizing a node that was given an explicit size. Only meaningful for nodes with [`NodeFrame::size`](./canvas.md) — content-hugging nodes have no size to drive. Draws the grip and (via the canvas's drag handling) reports a world-space size delta the caller applies to its own width/height.

**Tier.** paint.

**Module:** `ouroboros_ui::graph::resizer` (items are `pub(crate)` — crate-internal).

```rust
/// Side length (world px) of the corner grip.
pub(crate) const GRIP: f32 = 10.0;

/// Scene rect of the grip at a node's bottom-right corner.
pub(crate) fn grip_rect(node: egui::Rect) -> egui::Rect;

/// Paint the grip — a small filled square in the node's selection-ring color.
pub(crate) fn paint(painter: &egui::Painter, node: egui::Rect, tokens: &GraphTokens);
```

`grip_rect` centres a `GRIP × GRIP` square on `node.right_bottom()`. `paint` fills it with radius `core::RADIUS_SM` in `tokens.node_selected_ring`.

**How the canvas uses it.** Drawn per sized node; the resulting drag is accumulated into `CtxOut.node_resized` and surfaced on **`GraphResponse.node_resized: Vec<(NodeId, Vec2)>`** (world-space size deltas). The caller owns sizes and applies the deltas. See [canvas](./canvas.md).

---

## MiniMap

**What it is.** A fixed-size corner overview of the whole graph with the current viewport outlined. Each node is a small token-coloured rect; the visible region is a stroked outline. Click or drag inside it to recenter the view on that world point.

**Tier.** compose (overlay). Drawn in a foreground [`Area`] **above** the Scene so it receives clicks; it paints geometric rects only (token-coloured). O(nodes) per frame — fine for hundreds of nodes.

**Module:** `ouroboros_ui::graph::minimap` (`show` is `pub(crate)`).

```rust
/// MiniMap footprint (screen px) — internal const.
const MINI: Vec2 = Vec2::new(168.0, 120.0);

pub(crate) fn show(
    ui: &mut egui::Ui,
    canvas: egui::Rect,             // screen rect of the canvas (anchors top-right)
    nodes: &[(NodeId, egui::Rect)], // node rects, scene coords
    world_bounds: egui::Rect,       // union of content, scene coords
    view: egui::Rect,               // current visible region, scene coords
) -> Option<egui::Pos2>;            // requested new view center (world) on click/drag
```

It maps the region `world_bounds.union(view).expand(core::SPACE_4)` into a fixed `MINI`-sized panel pinned to the canvas's top-right (offset `-MINI.x - SPACE_3, SPACE_3`). Returns `None` if that source region is degenerate (zero width/height). On click **or** drag whose pointer is inside the panel, it returns the corresponding world `Pos2`. Colours: backdrop `theme.popover`, border `theme.border`, nodes `tokens.minimap_node`, viewport outline `tokens.minimap_view`.

**How the canvas uses it.** Enabled via `GraphView::new(id).minimap(true)`. The canvas calls `minimap::show(...)` after the Scene; a returned center recenters the view by mutating the Scene's `scene_rect`. See [canvas](./canvas.md).

---

## Toolbar

**What it is.** A floating action bar anchored just above a node, hosting caller-supplied DS widgets (e.g. a delete button). Placed in **scene coordinates**, so it tracks and scales with the node. Typically shown only while the node is selected. Unlike the others this is a method on `GraphCtx`, not a free `show` fn.

**Tier.** compose (overlay). A [`Surface`](../atoms/surface.md) bar — never paints inline.

**Module:** `ouroboros_ui::graph::toolbar` (provides an `impl GraphCtx<'_>` method).

```rust
impl GraphCtx<'_> {
    /// Draw a toolbar above `node`. No-op if the node wasn't emitted yet this frame.
    /// `content` lays its actions out left-to-right inside an elevated surface.
    pub fn node_toolbar(&mut self, node: NodeId, content: impl FnOnce(&mut egui::Ui));
}
```

It looks up `node`'s rect from `self.node_rects` (emitted earlier this frame) and returns early if absent — so **call it after the node**. The bar height is `core::CONTROL_MD + core::SPACE_2`; it anchors at the node's `left_top` minus `(0, height + SPACE_2)`, width `max(node.width(), 120.0)`, inside an `.elevated().pad(core::SPACE_1)` [`Surface`] laying `content` out horizontally.

**How the canvas uses it.** Called by the caller inside the `GraphView::show` closure, once per node that should show a toolbar (commonly gated on selection). Example in `examples/storybook.rs` (`page_graph_live`): a trash [`Button`](../atoms/button.md) per selected node, pushing into the caller's own delete list.

---

## Controls

**What it is.** A floating zoom/fit cluster overlaid on the canvas in **screen space** (bottom-left). Holds a zoom-out (`−`), zoom-in (`+`), a percent readout, and a fit (corners-out) button. It mutates nothing itself — it returns the requested action for [`GraphView`](./canvas.md) to apply to the `scene_rect`.

**Tier.** compose (overlay). A [`Surface`] of DS [`Button`]s drawn in a foreground [`Area`] so it sits above the Scene and receives clicks.

**Module:** `ouroboros_ui::graph::controls` (`ControlsAction` is `pub`; `show` is `pub(crate)`).

```rust
/// What the user asked the controls to do this frame.
#[derive(Clone, Copy, Debug, Default)]
pub struct ControlsAction {
    pub zoom_in: bool,
    pub zoom_out: bool,
    pub fit: bool,
}

pub(crate) fn show(
    ui: &mut egui::Ui,
    canvas: egui::Rect,  // screen rect of the canvas (anchors bottom-left)
    percent: i32,        // current zoom %, shown in the readout
) -> ControlsAction;
```

Anchors at `canvas.left_bottom() + (core::SPACE_3, -SPACE_3 - core::CONTROL_LG)`. Buttons are ghost, `sm`, icon-only ([`egui_phosphor`] `MINUS`/`PLUS`/`CORNERS_OUT`); the readout is `Text::new(format!("{percent}%")).caption().muted()`. Each press flips the matching `ControlsAction` flag for that frame.

**How the canvas uses it.** Enabled via `GraphView::new(id).controls(true)`. The canvas calls `controls::show(ui, rect, percent)` after the Scene and applies the returned action to the Scene's `scene_rect` — `zoom_in`/`zoom_out` adjust zoom, `fit` requests fit-to-content (also surfaced as **`GraphResponse.fit_requested: bool`**). See [canvas](./canvas.md).
