# ToolbarButton

> **Layer:** cell · **Path:** `src/cells/toolbar_button.rs` · **Exports:** `toolbar_button::ToolbarButton`

An icon toggle button with an optional hover tooltip, modelled on Unity/O3DE toolbars. It wraps the [`Toggle`](../atoms/toggle.md) atom in icon mode and binds it to a `&mut bool` (the active state). When a tooltip is set the response is wrapped by the [`Tooltip`](../atoms/tooltip.md) atom. Carries lifetime `'a` from the mutable borrow.

## Design

- **Purpose / when to use** — Tool palettes / toolbars where each button is a momentary-or-sticky icon toggle (select / move / rotate, view modes, snapping flags).
- **Anatomy** — A [`Toggle`](../atoms/toggle.md) in `.icon(glyph)` mode bound to the `&mut bool`, optionally decorated by a [`Tooltip`](../atoms/tooltip.md) over the resulting response.
- **Variants / states**

  | State | Source |
  |-------|--------|
  | active / inactive | the bound `&mut bool` (mutated in place by `Toggle`) |
  | tooltip on hover | present only when `tooltip(...)` was set |

- **Tokens / layout consumed** — Inherited from the [`Toggle`](../atoms/toggle.md) atom (sizing/fill); `ToolbarButton` adds none of its own. See [tokens](../../tokens.md).
- **Accessibility** — Tooltip provides the textual label for an otherwise icon-only control; prefer always setting `tooltip(...)`.

## API

| Method | Signature | Effect |
|--------|-----------|--------|
| `new` | `new(active: &'a mut bool, glyph: &'static str) -> Self` | Bind to the active flag with a phosphor glyph. |
| `tooltip` | `tooltip(self, tooltip: impl Into<String>) -> Self` | Hover tooltip text. |
| `id_source` | `id_source(self, id: impl std::hash::Hash) -> Self` | Stable id forwarded to the underlying `Toggle`. |
| `show` | `show(self, ui: &mut Ui) -> Response` | Render the toggle (mutating `active`); returns the `Toggle` response (tooltip-wrapped if set). |

## Usage

```rust
use ouroboros_ui::cells::ToolbarButton;
use ouroboros_ui::egui_phosphor::light;

let mut active = true;
ToolbarButton::new(&mut active, light::CURSOR).tooltip("Select").id_source("tb0").show(ui);
```

```rust
// realistic — a horizontal tool palette over an array of flags
let mut state = [true, false, false];
ui.horizontal(|ui| {
    ToolbarButton::new(&mut state[0], light::CURSOR).tooltip("Select").id_source("tb0").show(ui);
    ToolbarButton::new(&mut state[1], light::ARROWS_OUT).tooltip("Move").id_source("tb1").show(ui);
    ToolbarButton::new(&mut state[2], light::ARROWS_CLOCKWISE).tooltip("Rotate").id_source("tb2").show(ui);
});
```

## Composition

Composes the [`Toggle`](../atoms/toggle.md) atom (icon mode) and, when a tooltip is set, the [`Tooltip`](../atoms/tooltip.md) atom. It paints nothing — all visuals come from `Toggle`. Enforced by [`tests/no_painter_in_molecules.rs`](../../guards.md).

## Notes

- Binding: `active` is `&mut bool` — the toggle flips it in place; there is no `selected`/return-value to read for state, only `.clicked()` on the response for reacting to a press.
- Give each button an `id_source` (e.g. `"tb0"`); buttons sharing the same glyph would otherwise collide on the toggle's auto id.
- Mutually-exclusive tools (radio-like) are the consumer's responsibility — reset the other flags when one is clicked.
