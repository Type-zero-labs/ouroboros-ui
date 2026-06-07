# ColorSwatch

> **Layer:** atom · **Path:** `src/atoms/color_swatch.rs` · **Exports:** `color_swatch::ColorSwatch`

A swatch displaying an arbitrary `Color32` (consumer data, **not** a theme token) in a token-bordered square or circle. Senses clicks so a parent can open a color picker. Modeled on Unity's ColorField.

## Design

- **Purpose / when to use** — the base of a color field: show the current color and let the user click to edit. The fill color is application data, not a token.
- **Anatomy** — a filled square (`RADIUS_SM` corners) or circle, with a `theme.border` stroke. Fill = the supplied `Color32`.
- **Variants / sizes / states** — square (default) or `circle()`. Size is a free `f32` (default `core::ICON_LG` = 20px). Senses `click` but paints no hover/focus/disabled state.
- **Tokens consumed** — `theme.border` (stroke), `core::RADIUS_SM` (square corners), `core::ICON_LG` (default size). The fill is intentionally non-token (consumer color).
- **Accessibility** — bare `Response` from `Sense::click()`; no `widget_info`.

## API

| Signature | Effect |
|-----------|--------|
| `ColorSwatch::new(color: Color32) -> Self` | Construct with the color to display. |
| `.size(size: f32) -> Self` | Set side/diameter in px (default `ICON_LG`). |
| `.circle(self) -> Self` | Render as a circle instead of a rounded square. |
| `.show(self, ui: &mut Ui) -> Response` | Paint and return the click `Response`. |

## Usage

```rust
use ouroboros_ui::atoms::ColorSwatch;
use egui::Color32;

ColorSwatch::new(Color32::from_rgb(220, 80, 80)).show(ui);
```

```rust
use ouroboros_ui::atoms::ColorSwatch;

let mut color = some_color;
if ColorSwatch::new(color).size(28.0).circle().show(ui).clicked() {
    // open a picker, mutate `color`
}
```

## Composition

Atom: paints fill + border directly with `rect_filled`/`rect_stroke` (or `circle_*`). Composes no other atoms.

## Notes

- The fill is a raw `Color32` from the consumer — it deliberately bypasses the token system, unlike every other painted atom.
- No picker is built in; `show` only returns the click. Wire it to your own color-editing UI.

See [tokens](../../tokens.md) · [theming](../../theming.md).
