# ColorField

> **Layer:** molecule · **Path:** `src/molecules/color_field.rs` · **Exports:** `color_field::ColorField`

A color editor bound to a `&mut Color32`: a [`ColorSwatch`](../atoms/color_swatch.md) preview next to an editable hex [`Input`](../atoms/input.md). Clicking the swatch opens a full HSV/RGB/hex picker in a popover. Inspired by Unity / Figma color fields.

## Design

- **Purpose / when to use** — Edit a single color value: a tint, a material albedo, a UI accent override.
- **Anatomy** — `ui.horizontal` row of: a [`ColorSwatch`](../atoms/color_swatch.md) showing the current color, `SPACE_2`, then an [`Input`](../atoms/input.md) holding the hex string (`#RRGGBB`). A popover menu anchored on the swatch hosts egui's `color_picker_color32`.
- **States** — alpha editing optional via `.alpha(true)` (switches the picker's `Alpha` mode from `Opaque` to `OnlyBlend`).
- **Tokens / layout consumed** — `core::SPACE_2` (swatch→input gap), `core::CONTROL_LG * 6.0` (picker popover max width). See [tokens](../../tokens.md).

## API

| Method | Effect |
|---|---|
| `ColorField::new(color: &'a mut Color32) -> Self` | Bind the color. Alpha off by default. |
| `.alpha(alpha: bool) -> Self` | Enable editing the alpha channel in the picker. |
| `.id_source(id: impl std::hash::Hash) -> Self` | Stable id for the hex input (defaults to `"color_field"`). |
| `.show(self, ui: &mut Ui) -> Response` | Render; returns the hex [`Input`](../atoms/input.md) `Response`. |

## Usage

```rust
use ouroboros_ui::molecules::ColorField;
use egui::Color32;

// minimal
let mut c = Color32::from_rgb(26, 188, 156);
ColorField::new(&mut c).id_source("cf1").show(ui);
```

```rust
use ouroboros_ui::molecules::ColorField;

// realistic — alpha editing
ColorField::new(&mut c)
    .alpha(true)
    .id_source("material_albedo")
    .show(ui);
```

## Composition

Composes [`ColorSwatch`](../atoms/color_swatch.md) + [`Input`](../atoms/input.md), plus `egui::Popup::menu` hosting `egui::color_picker::color_picker_color32` — a built-in egui widget, not a paint call. It never paints primitives directly — see the [guards](../../guards.md).

## Notes

- Two-way binding: typing a valid hex into the input (`Color32::from_hex`) writes back into `*color`; the picker writes directly into `*color`.
- The hex string is recomputed from the bound color every frame as `#{:02X}{:02X}{:02X}` (RGB only — alpha is edited through the picker).
- Pass a unique `id_source` per field — the default `"color_field"` collides if several share a frame.
- The returned `Response` is the hex input's, so `.changed()` fires on hex edits.
