# GraphViewState · NodeDrag · ConnectDrag · MarqueeDrag

> **Layer:** graph · **Path:** `src/graph/state.rs` · **Exports:** `GraphViewState`, `NodeDrag`, `ConnectDrag`, `MarqueeDrag`

The view-state: the **only** thing the library owns across frames. The caller owns the node/edge *data*; this struct owns the *view* — where the camera is, what's selected, and what's mid-drag. It is stored in egui's temp memory keyed by the [`GraphView`](./canvas.md)'s id, so it must be `Clone + Default` to live in the temp store.

## Design

- **Purpose.** Persist the minimum the graph needs between frames. Loaded at the top of `GraphView::show` (`get_temp(id).unwrap_or_default()`), mutated through the frame, written back with `insert_temp` at the end. The caller almost never touches it directly — it observes effects via [`GraphResponse`](./canvas.md) instead.
- **`scene_rect`.** The [`egui::Scene`] view window expressed in **world coordinates**. `GraphView::show` hands a `&mut` to it into `Scene::show`, and Scene mutates it in place on pan/zoom. Controls' zoom buttons rescale it around its center; fit replaces it with the content bounds expanded by `core::SPACE_8`; the minimap recenters it.
- **`Rect::ZERO` sentinel.** The `Default` `scene_rect` is `Rect::ZERO`, the "uninitialised" marker — Scene interprets it as "no camera yet" and **auto-fits the content on the first frame**. After the first frame it holds a real world rect.
- **Selection model.** `selection: HashSet<NodeId>` plus an `Option<(Port, Port)>` edge selection (mutually managed: selecting a node clears the edge selection and vice-versa). Delete/Backspace removes the selected edge first, else drains the selected nodes.
- **Transient drag state.** Three optional sub-structs, each present only while a drag is live: `drag` (node move), `connect` (wire), `marquee` (box-select). `connect` and `edge_selection` round-trip through the Scene closure (read in, resolved, written out); `drag` / `marquee` / `hovered_node` are maintained across frames.

### `GraphViewState` fields

| Field | Type | Role |
|---|---|---|
| `scene_rect` | `Rect` | Scene window in world coords. `Rect::ZERO` ⇒ auto-fit first frame. |
| `selection` | `HashSet<NodeId>` | Currently selected nodes. |
| `edge_selection` | `Option<(Port, Port)>` | Currently selected edge (the two endpoint ports). |
| `hovered_node` | `Option<NodeId>` | Node under the cursor. |
| `drag` | `Option<NodeDrag>` | Active node-move drag. |
| `connect` | `Option<ConnectDrag>` | Active connect (wire) drag. |
| `marquee` | `Option<MarqueeDrag>` | Active box-select drag. |

`#[derive(Clone, Debug)]`; `Default` sets `scene_rect = Rect::ZERO`, all collections empty, all options `None`.

### Helper structs

`NodeDrag` — `#[derive(Clone, Copy, Debug)]`

| Field | Type | Role |
|---|---|---|
| `node` | `NodeId` | Which node grabbed the drag. |
| `accum_world` | `Vec2` | Accumulated world-space delta. Recomputed from origin each frame (accumulator pattern) to avoid drift on slow drags. |

`ConnectDrag` — `#[derive(Clone, Copy, Debug)]`

| Field | Type | Role |
|---|---|---|
| `from` | `Port` | Port the wire was dragged out from. |
| `from_world` | `Pos2` | World anchor of `from`'s handle. |
| `cursor_world` | `Pos2` | Current cursor position (world); the wire trails to it. |

`MarqueeDrag` — `#[derive(Clone, Copy, Debug)]`

| Field | Type | Role |
|---|---|---|
| `start_world` | `Pos2` | Drag origin in world coords (so the box tracks under pan/zoom). |
| `cursor_world` | `Pos2` | Current cursor (world). |
| `additive` | `bool` | Shift held at drag start ⇒ additive selection. |

## API

These are plain data structs — no methods. Construct/inspect their public fields directly. `GraphViewState::default()` is the canonical constructor; the canvas does this for you. All four derive `Clone` (state) / `Clone + Copy` (the three drags) so they round-trip through egui temp storage and the Scene closure cheaply.

## Usage

```rust
use ouroboros_ui::graph::GraphViewState;

// The canvas owns this; you rarely touch it. But you *can* read/seed it:
let id = egui::Id::new("my_graph");

// Pre-position the camera (skip the first-frame auto-fit):
ui.data_mut(|d| {
    let mut st: GraphViewState = d.get_temp(id).unwrap_or_default();
    st.scene_rect = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(800.0, 600.0));
    d.insert_temp(id, st);
});

// Inspect the persisted selection out-of-band (normally use GraphResponse::selection):
let selected: usize = ui.data(|d| {
    d.get_temp::<GraphViewState>(id).map(|s| s.selection.len()).unwrap_or(0)
});
```

## Composition / Notes

- **Single source of camera truth.** There is exactly one `GraphViewState` per `GraphView` id; clobbering it (e.g. inserting a fresh `default()`) resets the camera to the auto-fit sentinel and clears selection.
- **`scene_rect` is world, not screen.** A smaller `scene_rect` means *more zoomed in* (the window covers less world). This is the opposite mental model from a screen rect.
- **Not the same as [`Viewport`](./viewport.md).** `Viewport` is a standalone pan+zoom transform helper that the live canvas does **not** use; the canvas drives `scene_rect` through `egui::Scene`. Don't confuse the two camera representations.
- **Identity vocab.** `NodeId`, `Port`, `PortSide` — see [identity](./identity.md).
- Foundation: [architecture](../../architecture.md) · [tokens](../../tokens.md) · [theming](../../theming.md) · [guards](../../guards.md). Layer overview: [README](./README.md).
