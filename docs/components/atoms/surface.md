# Surface

> **Layer:** atom · **Path:** `src/atoms/surface.rs` · **Exports:** `surface::{Surface, SurfaceBorder, SurfaceFill}`

The one place a "box" is painted — fill, border, radius, shadow, and padding — so molecules can compose a surface instead of hand-rolling an `egui::Frame`. Optionally `interactive` (clickable) and `selected` (ring border) for card-style selectors. Atoms may paint; molecules may not, so reach for this whenever you need a container.

## Design

- **Purpose / when to use** — wrap any content group in a themed container: cards, panels, popovers, selectable tiles. This is the canonical container atom; do not build raw `egui::Frame`s in higher layers.
- **Anatomy** — an `egui::Frame` with corner radius, inner margin (padding), optional fill, optional stroke, optional shadow; runs `content` inside. When `interactive`, re-interacts the painted rect for clicks and draws a `border_strong` outline on hover.
- **Variants / sizes / states**

  **`SurfaceFill`**: `Card` (default → `theme.card`), `Muted` (`theme.muted`), `Background` (`theme.background`), `None` (no fill).

  **`SurfaceBorder`**: `None`, `Default` (default → `theme.border`), `Strong` (`theme.border_strong`).

  Other knobs: `radius(f32)` (default `RADIUS_LG`), `elevated()` (adds `SHADOW_MD`), `pad(f32)` (default `SPACE_4`), `interactive()`, `selected(bool)`.

  **States**: `selected(true)` overrides the border with a `ring` stroke at `BORDER_FOCUS`; `interactive` adds a `border_strong` hover outline.

- **Tokens consumed** — `theme.card`/`theme.muted`/`theme.background` (fill), `theme.border`/`theme.border_strong` (border), `theme.ring` (selected), `core::RADIUS_LG` (default radius), `core::SPACE_4` (default padding), `core::BORDER_THIN`/`BORDER_FOCUS`, `core::SHADOW_MD` (elevation).
- **Accessibility** — when `interactive`, the rect is re-interacted with `Sense::click()`; the returned response carries the click. Non-interactive surfaces return the frame's response.

## API

| Signature | Effect |
|-----------|--------|
| `Surface::new() -> Self` / `Surface::default()` | Card fill, default border, `RADIUS_LG`, `SPACE_4` padding. |
| `.fill(f: SurfaceFill) -> Self` | Set fill. |
| `.muted()` / `.background()` / `.fill_none()` | Fill shorthands. |
| `.border(b: SurfaceBorder) -> Self` | Set border. |
| `.border_none()` / `.border_strong()` | Border shorthands. |
| `.radius(radius: f32) -> Self` | Corner radius. |
| `.elevated(self) -> Self` | Add `SHADOW_MD`. |
| `.pad(padding: f32) -> Self` | Inner margin. |
| `.interactive(self) -> Self` | Sense clicks + hover outline. |
| `.selected(selected: bool) -> Self` | Ring border (overrides `border`). |
| `.id_source(id: impl Hash) -> Self` | Stable id for the interaction (else response id). |
| `.show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R>` | Paint the box, run `content`, return `InnerResponse`. |

**`SurfaceFill`**: `Card` (default), `Muted`, `Background`, `None`. **`SurfaceBorder`**: `None`, `Default` (default), `Strong`.

## Usage

```rust
use ouroboros_ui::atoms::Surface;
use ouroboros_ui::atoms::Text;

Surface::new().show(ui, |ui| {
    Text::new("Card body").show(ui);
});
```

```rust
use ouroboros_ui::atoms::Surface;

let mut chosen = false;
let r = Surface::new()
    .interactive()
    .selected(chosen)
    .elevated()
    .show(ui, |ui| { /* tile content */ });
if r.response.clicked() { chosen = !chosen; }
```

## Composition

Atom: the **only** atom that exposes a content closure and paints the container box. Higher layers compose `Surface` rather than building `egui::Frame`s — this is enforced by the layer rules (see [guards](../../guards.md)).

## Notes

- `show` returns `egui::InnerResponse<R>` — `.inner` is your closure's value, `.response` is the surface's (click-bearing when `interactive`).
- `selected(true)` takes precedence over any `border(..)` setting.
- For repeated/clickable surfaces in a list, set `id_source` to keep interaction stable.

See [tokens](../../tokens.md) · [theming](../../theming.md) · [guards](../../guards.md).
