# NumericField

> **Layer:** atom · **Path:** `src/atoms/numeric_field.rs` · **Exports:** `numeric_field::NumericField`

A scrubbable numeric input bound to a `&mut f32`, modeled on Unity's Numeric Field. A token box wraps an egui `DragValue` (drag to scrub, click to type); the value is right-aligned. `.stepper()` flanks it with ghost `−`/`+` icon buttons, and `.suffix()` appends a unit. The editing substrate is egui's; the casing is token.

## Design

- **Purpose / when to use** — numeric entry where dragging/scrubbing helps (positions, scales, counts). For a bounded value you mostly slide, use [`Slider`](slider.md); for text use [`Input`](input.md).
- **Anatomy** — filled rect (`muted`) → hover veil → inner content: a right-aligned `DragValue`, optionally flanked by `−` (left) and `+` (right) ghost [`Button`](button.md)s → border stroke encoding state.
- **Variants / sizes / states**

  | Size | Height |
  |------|--------|
  | `Sm` | `CONTROL_SM` 26 |
  | `Md` (default) | `CONTROL_MD` 32 |
  | `Lg` | `CONTROL_LG` 38 |

  **Border state** (precedence): error → `destructive`; else focused → `ring` @ `BORDER_FOCUS`; else `input`. Disabled dims and disables the `DragValue` + stepper buttons.

- **Tokens consumed** — `theme.muted`, `theme.input`/`theme.destructive`/`theme.ring`, `theme.hover_overlay`, `core::RADIUS_MD`, `core::SPACE_2` (inner inset), `core::BORDER_THIN`/`BORDER_FOCUS`, `core::hover_t`, `core::disabled_color`, `core::Size`.
- **Accessibility** — focus/keyboard come from egui's `DragValue`; the border switches to the ring on focus.

## API

| Signature | Effect |
|-----------|--------|
| `NumericField::new(value: &mut f32) -> Self` | Bind to an `f32`. |
| `.range(min: f32, max: f32) -> Self` | Clamp range (default `-INF..INF`). |
| `.speed(speed: f32) -> Self` | Drag sensitivity (default `0.1`). |
| `.step(step: f32) -> Self` | Stepper increment (default `1.0`). |
| `.stepper(self) -> Self` | Flank with `−`/`+` buttons. |
| `.suffix(suffix: impl Into<String>) -> Self` | Append a unit string. |
| `.full_width(self) -> Self` | Fill the available width (drops the `FIELD_NUM_W` cap; the floor still applies). |
| `.fixed_width(self) -> Self` | Pin a constant width (`NUMERIC_STEPPER_W`), ignoring `available_width` — for a stepper in a squeezed panel so the value never slides behind the `−`. Takes precedence over `.full_width()`. |
| `.enabled(enabled: bool) -> Self` / `.disabled()` | Enable/disable. |
| `.error(error: bool) -> Self` | Force the destructive border. |
| `.size(s: Size) -> Self` / `.sm()` / `.lg()` | Size (`core::Size`). |
| `.show(self, ui: &mut Ui) -> Response` | Return the inner `DragValue` `Response`. |

## Usage

```rust
use ouroboros_ui::atoms::NumericField;

let mut scale = 1.0_f32;
NumericField::new(&mut scale).range(0.0, 10.0).speed(0.05).show(ui);
```

```rust
use ouroboros_ui::atoms::NumericField;

let mut count = 3.0_f32;
NumericField::new(&mut count)
    .range(0.0, 99.0)
    .step(1.0)
    .stepper()
    .suffix(" items")
    .show(ui);
```

## Composition

Atom: paints the box/border/veil directly and embeds an egui `DragValue` in a child `Ui`. Composes the [`Button`](button.md) atom (ghost, sm, icon-only) for the `−`/`+` steppers.

## Notes

- Binding is `&mut f32`. The stepper buttons mutate the value directly (clamped to range) when clicked; the returned `Response` is the `DragValue`'s.
- Stepper buttons get glyphs `light::MINUS` / `light::PLUS`; the `+` is pinned right and the value fills the remaining width.
- **Width model:** by default fills `available_width` clamped to `NUMERIC_MIN_W..=FIELD_NUM_W` (stepper floors higher at `NUMERIC_STEPPER_MIN_W`) so numbers stay column-aligned. `.full_width()` drops the cap; `.fixed_width()` ignores `available_width` entirely and pins `NUMERIC_STEPPER_W` — use it for steppers in resizable inspector panels where the box must not shrink under the value.

See [tokens](../../tokens.md) · [theming](../../theming.md) · [guards](../../guards.md).
