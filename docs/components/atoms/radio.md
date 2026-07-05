# Radio

> **Layer:** atom · **Path:** `src/atoms/radio.rs` · **Exports:** `radio::Radio`

A single radio button taking `selected: bool` (by value, not a binding) with an optional trailing label. It is a standalone atom: it reports clicks via its `Response`; the consumer (or a future RadioGroup molecule) owns single-selection. Circle is token-painted (border `input`; when selected, an inner `primary` dot); the label is rendered via the [`Text`](text.md) atom.

## Design

- **Purpose / when to use** — one option within a mutually-exclusive group. For independent booleans use [`Checkbox`](checkbox.md).
- **Anatomy** — outline circle (left, vertically centered; border `primary` when selected else `input`) → hover veil → inner `primary` dot (radius × 0.5) when selected → focus ring → optional label `Text`.
- **Variants / sizes / states**

  | Size | Circle size |
  |------|-------------|
  | `Sm` | `ICON_SM` 14 |
  | `Md` (default) | `ICON_MD` 16 |
  | `Lg` | `ICON_LG` 20 |

  **States**: selected (border `primary` + inner dot), hover (`hover_t` veil), focus (ring), disabled (`disabled_color`; sense → hover), non-interactive (`interactive(false)` — display-only).

- **Tokens consumed** — `theme.primary`, `theme.input`, `theme.foreground`, `theme.hover_overlay`, `theme.ring`, `core::SPACE_2` (gap), `core::BORDER_THIN`, `core::hover_t`, `core::disabled_color`, `typography::body`.
- **Accessibility** — emits `WidgetInfo::selected(WidgetType::RadioButton, enabled, selected, label)`. Focus ring via `focus::focus_ring_circle`. Hit target spans circle + gap + label.

## API

| Signature | Effect |
|-----------|--------|
| `Radio::new(selected: bool) -> Self` | Construct with current selection state. |
| `.interactive(interactive: bool) -> Self` | Display-only when `false`. |
| `.size(s: Size) -> Self` / `.sm()` / `.lg()` | Size (`core::Size`). |
| `.label(label: impl Into<String>) -> Self` | Add a trailing label. |
| `.enabled(enabled: bool) -> Self` / `.disabled()` | Enable/disable. |
| `.id_source(id: impl Hash) -> Self` | Stable id (else `response.id`). |
| `.show(self, ui: &mut Ui) -> Response` | Paint and return the `Response` (`clicked` to flip at the consumer). |

## Usage

```rust
use ouroboros_ui::atoms::Radio;

#[derive(PartialEq)] enum Mode { A, B }
let mut mode = Mode::A;

if Radio::new(mode == Mode::A).label("Option A").show(ui).clicked() {
    mode = Mode::A;
}
if Radio::new(mode == Mode::B).label("Option B").show(ui).clicked() {
    mode = Mode::B;
}
```

## Composition

Atom: paints the circle/dot directly; composes the [`Text`](text.md) atom for the label inside a child `Ui`.

## Notes

- Unlike [`Checkbox`](checkbox.md)/[`Switch`](switch.md), `Radio` does **not** take a `&mut` binding and does **not** mutate state — it only reports `.clicked()`. The caller updates the selected value. (No `mark_changed`; check `.clicked()`, not `.changed()`.)
- Label width reserves extra room for the role's tracking so it does not clip.

See [tokens](../../tokens.md) · [theming](../../theming.md) · [typography](../../typography.md) · [guards](../../guards.md).
