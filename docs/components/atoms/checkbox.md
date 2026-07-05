# Checkbox

> **Layer:** atom · **Path:** `src/atoms/checkbox.rs` · **Exports:** `checkbox::Checkbox`

A boolean checkbox bound to a `&mut bool`, with an optional trailing label. The box is painted from tokens (border `input`; when on, `primary` fill + a centered check glyph in `primary_foreground`); the label is rendered via the [`Text`](text.md) atom. Supports an indeterminate (dash) presentation, focus ring, and disabled dim.

## Design

- **Purpose / when to use** — toggle a single boolean, or one item in a multi-select set. For an on/off setting that reads like a switch use [`Switch`](switch.md); for mutually-exclusive options use [`Radio`](radio.md).
- **Anatomy** — square box (left, vertically centered) → optional `primary` fill when on → border stroke → hover veil → check/minus glyph when on → focus ring → optional label `Text` to the right.
- **Variants / sizes / states**

  | Size | Box size |
  |------|----------|
  | `Sm` | `ICON_SM` 14 |
  | `Md` (default) | `ICON_MD` 16 |
  | `Lg` | `ICON_LG` 20 |

  **States**: checked (`primary` fill + `light::CHECK`), indeterminate (`light::MINUS`, takes visual precedence over checked, presentational only), hover (`hover_t` veil), focus (ring), disabled (`disabled_color` dim; sense drops to hover), non-interactive (`interactive(false)` — display-only, no toggle).

- **Tokens consumed** — `theme.primary`, `theme.primary_foreground`, `theme.input`, `theme.foreground`, `theme.hover_overlay`, `theme.ring`, `core::RADIUS_SM`, `core::SPACE_2` (gap), `core::BORDER_THIN`, `core::hover_t`, `core::disabled_color`, `typography::body`, `typography::icon_font`.
- **Accessibility** — emits `WidgetInfo::selected(WidgetType::Checkbox, enabled, on, label)`. Focus ring via `focus::focus_ring_rect`. Hit target spans box + gap + label.

## API

| Signature | Effect |
|-----------|--------|
| `Checkbox::new(checked: &mut bool) -> Self` | Bind to a boolean. |
| `.interactive(interactive: bool) -> Self` | Display-only when `false` (no click/toggle). |
| `.indeterminate(indeterminate: bool) -> Self` | Show the mixed/dash state (visual precedence). |
| `.size(s: Size) -> Self` / `.sm()` / `.lg()` | Size (`core::Size`). |
| `.label(label: impl Into<String>) -> Self` | Add a trailing label. |
| `.enabled(enabled: bool) -> Self` / `.disabled()` | Enable/disable. |
| `.id_source(id: impl Hash) -> Self` | Stable id (else `response.id`). |
| `.show(self, ui: &mut Ui) -> Response` | Toggle on click, `mark_changed`, return `Response`. |

## Usage

```rust
use ouroboros_ui::atoms::Checkbox;

let mut agreed = false;
if Checkbox::new(&mut agreed).label("I agree").show(ui).changed() {
    // agreed flipped
}
```

```rust
use ouroboros_ui::atoms::Checkbox;

// tri-state header in a select-all pattern
let mut all = false;
Checkbox::new(&mut all)
    .indeterminate(some_selected && !all)
    .label("Select all")
    .show(ui);
```

## Composition

Atom: paints the box and glyph directly; composes the [`Text`](text.md) atom for the label inside a child `Ui`.

## Notes

- Binding is `&mut bool`. On a click the atom flips `*checked` and calls `mark_changed()`, so check `.changed()`.
- `indeterminate(true)` only changes the painted glyph (dash); it does not alter `*checked`. The consumer is expected to clear it on the next interaction.
- `interactive(false)` makes it display-only — clicks pass through (useful inside a clickable card).
- Label width reserves extra room for the role's letter-spacing so it does not clip.

See [tokens](../../tokens.md) · [theming](../../theming.md) · [typography](../../typography.md) · [guards](../../guards.md).
