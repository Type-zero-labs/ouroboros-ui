# TileGrid

> **Layer:** atom · **Path:** `src/atoms/tile_grid.rs` · **Exports:** `tile_grid::{TileGrid, TileRect}`

A texture (an atlas) split into a `cols × rows` tile grid, with a selectable **block** of
tiles: click picks one tile, drag picks the rectangle from the press tile to the release tile.
Modeled on the Tiled / Godot TileSet picker.

## Design

- **Purpose / when to use** — pick tiles out of a material/tileset atlas to paint with
  (studio texture painting: a 2×2 block paints a 2×2 stamp). The texture is consumer data.
- **Why it exists** — the DS had no selectable grid; the only marquee is the node graph's
  (`graph/state.rs`), tied to node selection. Use/extend did not cover it (W2 of the E2E
  fixes, finding A7).
- **Anatomy** — the texture (drawn 1:1 or scaled down to `max_width`), grid lines, a drag
  preview rectangle and the committed selection ring.
- **States** — idle; dragging (preview in `theme.muted_foreground`); selected
  (`theme.ring`, `BORDER_FOCUS`). A selection outside the grid is not drawn.
- **Tokens consumed** — `theme.scrim` (grid lines), `theme.muted_foreground` (drag preview),
  `theme.ring` (selection), `core::BORDER_THIN`, `core::BORDER_FOCUS`, `core::TINT_NONE`
  (neutral texture tint).
- **Accessibility** — bare `Response` from `Sense::click_and_drag()`; no `widget_info`.

## API

| Signature | Effect |
|-----------|--------|
| `TileGrid::new(texture: TextureId, size: Vec2, cols: u32, rows: u32, selection: &mut Option<TileRect>) -> Self` | Texture pixel size + grid + the selection it edits. |
| `.max_width(width: f32) -> Self` | Cap the drawn width (never scales up). |
| `.id_salt(salt) -> Self` | Distinguish two grids in the same `Ui`. |
| `.show(self, ui: &mut Ui) -> Response` | Paint; `changed()` on the frame a selection is committed. |
| `TileRect { col, row, w, h }` / `TileRect::spanning(a, b)` | A tile block; `spanning` orders two corner tiles. |

## Usage

```rust
use ouroboros_ui::atoms::{TileGrid, TileRect};

let mut sel: Option<TileRect> = None;
if TileGrid::new(tex.id(), tex.size_vec2(), 4, 4, &mut sel).max_width(320.0).show(ui).changed() {
    // paint with `sel`
}
```
