# Slider

> **Layer:** atom · **Path:** `src/atoms/slider.rs` · **Exports:** `slider::Slider`

A draggable numeric value over a range, bound to a `&mut f32`. A `muted` track + a `primary` filled portion + a `primary` thumb; drag or click anywhere on the track to set. Modeled on shadcn/Unity/O3DE sliders. For precise numeric typing use [`NumericField`](numeric_field.md).

## Design

- **Purpose / when to use** — pick a continuous (or stepped) value within a known range where coarse adjustment is fine. Default range is `0.0..=1.0`.
- **Anatomy** — thin pill track (`SPACE_1` tall, `muted`) → `primary` fill from left to thumb → circular `primary` thumb with a `background` stroke ring → hover veil on the thumb → focus ring on the thumb.
- **Variants / sizes / states**

  | Size | Control height (thumb area) |
  |------|-----------------------------|
  | `Sm` | `ICON_SM` 14 |
  | `Md` (default) | `ICON_MD` 16 |
  | `Lg` | `ICON_LG` 20 |

  **States**: drag/click sets value (`Sense::click_and_drag`, `mark_changed`); hover (`hover_t` veil on thumb); focus (ring on thumb, only when enabled); disabled (`disabled_color`, sense → hover).

- **Tokens consumed** — `theme.muted` (track), `theme.primary` (fill + thumb), `theme.background` (thumb stroke), `theme.hover_overlay`, `theme.ring`, `core::SPACE_1` (track thickness), `core::BORDER_THIN`, `core::hover_t`, `core::disabled_color`, `core::Size`.
- **Accessibility** — focus ring via `focus::focus_ring_circle`. (No explicit `widget_info` is set.)

## API

| Signature | Effect |
|-----------|--------|
| `Slider::new(value: &mut f32) -> Self` | Bind to an `f32` (range defaults `0..=1`). |
| `.range(min: f32, max: f32) -> Self` | Set the range. |
| `.step(step: f32) -> Self` | Quantize to multiples of `step`. |
| `.enabled(enabled: bool) -> Self` / `.disabled()` | Enable/disable. |
| `.size(s: Size) -> Self` / `.sm()` / `.lg()` | Size (`core::Size`). |
| `.show(self, ui: &mut Ui) -> Response` | Drag/click sets value, `mark_changed`, return `Response`. |

## Usage

```rust
use ouroboros_ui::atoms::Slider;

let mut volume = 0.5_f32;
if Slider::new(&mut volume).show(ui).changed() {
    // volume changed
}
```

```rust
use ouroboros_ui::atoms::Slider;

let mut zoom = 1.0_f32;
Slider::new(&mut zoom).range(0.25, 4.0).step(0.25).show(ui);
```

## Composition

Atom: paints track/fill/thumb directly. Composes no other atoms.

## Notes

- Binding is `&mut f32`; the value is clamped to range and (if `step > 0`) snapped. Check `.changed()`.
- Greedily takes `ui.available_width()`.
- Clicking anywhere on the track jumps the thumb to that position (not just dragging the thumb).

See [tokens](../../tokens.md) · [theming](../../theming.md) · [guards](../../guards.md).
