# Viewport

> **Layer:** graph · **Path:** `src/graph/viewport.rs` · **Exports:** `Viewport`

A pure, `Copy` world↔screen transform value type — pan + zoom, nothing else. It stores only `pan` and `zoom`, never the canvas rect; the canvas origin is passed in per call so the math stays testable without an `egui::Ui` and the whole thing is trivially storable in egui memory. The zoom-anchored math is ported from the studio's `events/canvas.rs`, cleaned up and unit-tested.

> **CRITICAL — the live canvas does not use this.** [`GraphView`](./canvas.md) drives its camera through `egui::Scene` directly (zoom range `0.2..4.0`, stored as `scene_rect` in [`GraphViewState`](./state.md)). `Viewport` is a **standalone utility** with its own clamps (`0.25..2.5`) and is **not currently wired into the canvas**. Treat it as a reusable transform helper / reference implementation, not the canvas's camera.

## Design

- **Purpose / when to use.** A self-contained pan+zoom transform you can use anywhere you need world↔screen math without `egui::Scene` — custom overlays, off-canvas hit-testing, or porting the studio's camera. For the live node editor, the camera is `Scene`/`scene_rect`, not this.
- **Conventions.** `world` is the graph's own coordinate space (node positions live here); `screen` is egui pixels. `pan` is the screen-space offset of the world origin relative to the canvas top-left; `zoom` is screen-px per world-unit. `canvas_origin` (the canvas rect's `left_top()`) is supplied per call.
- **State.** Two public fields only: `pan: Vec2`, `zoom: f32`. `Default` is `pan 0, zoom 1`. Derives `Clone, Copy, Debug, PartialEq`.
- **Clamps.** `MIN_ZOOM = 0.25`, `MAX_ZOOM = 2.5` (associated consts). Applied by `zoom_around` and `fit`. (Distinct from the canvas's `0.2..4.0`.)

## API

### Associated constants

| Const | Value | Meaning |
|---|---|---|
| `Viewport::MIN_ZOOM` | `0.25` | Lower zoom clamp — nodes never shrink to dust. |
| `Viewport::MAX_ZOOM` | `2.5` | Upper zoom clamp — nodes never balloon past usefulness. |

### Fields

| Field | Type | Meaning |
|---|---|---|
| `pan` | `Vec2` | Screen-space offset of the world origin relative to the canvas top-left. |
| `zoom` | `f32` | Screen px per world unit. |

### Methods

| Method | Signature | Effect |
|---|---|---|
| `default` | `fn default() -> Self` | `pan 0`, `zoom 1`. |
| `world_to_screen` | `fn world_to_screen(&self, canvas_origin: Pos2, world: Pos2) -> Pos2` | `canvas_origin + pan + world.to_vec2() * zoom`. |
| `screen_to_world` | `fn screen_to_world(&self, canvas_origin: Pos2, screen: Pos2) -> Pos2` | Inverse of `world_to_screen`. |
| `scale` | `fn scale(&self, world_len: f32) -> f32` | Scale a world length to on-screen length (`world_len * zoom`). |
| `pan_by` | `fn pan_by(&mut self, delta_screen: Vec2)` | Pan by a screen-space delta (e.g. a drag delta). |
| `zoom_around` | `fn zoom_around(&mut self, canvas_origin: Pos2, anchor: Pos2, factor: f32)` | Multiply zoom by `factor`, keeping the world point under `anchor` (a screen point, usually the cursor) pinned. Clamped to `MIN_ZOOM..=MAX_ZOOM`; no-op when the clamp pins it. |
| `fit` | `fn fit(&mut self, content_world: Rect, canvas: Rect, margin: f32)` | Frame `content_world` centered inside `canvas`, leaving `margin` screen px each side. No-op for an empty/degenerate content rect; resulting zoom is clamped. |

## Usage

```rust
use ouroboros_ui::graph::Viewport;

let canvas = ui.max_rect();
let origin = canvas.left_top();
let mut vp = Viewport::default();

// Pan from a drag delta:
vp.pan_by(response.drag_delta());

// Zoom toward the cursor on scroll:
if let Some(cursor) = ui.ctx().pointer_latest_pos() {
    let factor = 1.0 + ui.input(|i| i.smooth_scroll_delta.y) * 0.001;
    vp.zoom_around(origin, cursor, factor);
}

// Fit all nodes with a 32px margin:
vp.fit(content_bounds_world, canvas, 32.0);

// Project a node position to the screen for painting:
let screen = vp.world_to_screen(origin, node_world_pos);
```

## Composition / Notes

- **Paint-tier utility.** `Viewport` sits in the paint tier alongside grid/edge/handle — a value-level transform with no `Ui` dependency. It does not own a canvas rect, selection, or any drag state (that is [`GraphViewState`](./state.md)).
- **Not the canvas camera (again).** Because the live canvas uses `egui::Scene`, changing `Viewport`'s clamps or math has no effect on `GraphView`. If you need to alter the live zoom range, edit the `MIN_ZOOM`/`MAX_ZOOM` consts in `canvas.rs` (`0.2`/`4.0`), not here.
- **Unit tests.** `viewport.rs` carries a `#[cfg(test)]` module that locks the contract:
  - `world_screen_round_trips` — `screen_to_world(world_to_screen(w)) == w` across sample points with non-trivial pan/zoom.
  - `zoom_keeps_point_under_cursor` — the world point under the anchor is invariant across `zoom_around`, and zoom lands on the requested factor.
  - `zoom_clamps` — repeated zoom-in/out saturates exactly at `MAX_ZOOM` / `MIN_ZOOM`.
  - `fit_centers_content` — content center lands on canvas center and fits within the margins.
  - `fit_ignores_degenerate` — a zero-size content rect leaves the viewport unchanged.
- Foundation: [architecture](../../architecture.md) · [tokens](../../tokens.md) · [theming](../../theming.md) · [guards](../../guards.md). Layer overview: [README](./README.md). Identity: [identity](./identity.md).
