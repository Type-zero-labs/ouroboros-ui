# Divider

> **Layer:** atom · **Path:** `src/atoms/divider.rs` · **Exports:** `divider::{Axis, Divider}`

A hairline rule in the `border` token, horizontal or vertical. Default thickness is `core::BORDER_THIN`; a horizontal divider fills the available width, a vertical one fills the available height. Color and weight are overridable.

## Design

- **Purpose / when to use** — separate content groups, underline a tab indicator (`thick()`), or draw a red rule (`destructive()`). For an interactive resize band between panels use [`SplitterHandle`](splitter_handle.md).
- **Anatomy** — a single `hline`/`vline` stroke. Horizontal: width = `ui.available_width()`, allocated height = weight. Vertical: height = `ui.available_height()`, allocated width = weight.
- **Variants / sizes / states**

  | Builder | Effect |
  |---------|--------|
  | `horizontal()` | Axis::Horizontal, `border` color, `BORDER_THIN`. |
  | `vertical()` | Axis::Vertical, `border` color, `BORDER_THIN`. |
  | `.thick()` | Weight = `BORDER_FOCUS` (2px) — e.g. tab underline. |
  | `.destructive()` | Color = `theme.destructive`. |
  | `.color(c)` | Explicit color override (wins over destructive). |

  No hover/focus/disabled states (sense is `hover` only).

- **Tokens consumed** — `theme.border` (default color), `theme.destructive` (when `destructive()`), `core::BORDER_THIN` (default weight), `core::BORDER_FOCUS` (when `thick()`).
- **Accessibility** — bare hover `Response`; no `widget_info`.

## API

| Signature | Effect |
|-----------|--------|
| `Divider::horizontal() -> Self` | Construct a horizontal rule. |
| `Divider::vertical() -> Self` | Construct a vertical rule. |
| `.color(color: Color32) -> Self` | Override color. |
| `.destructive(self) -> Self` | Use the `destructive` token. |
| `.thick(self) -> Self` | Use `BORDER_FOCUS` weight. |
| `.show(self, ui: &mut Ui) -> Response` | Paint and return the hover `Response`. |

**`Axis`** (enum): `Horizontal` (default), `Vertical`.

## Usage

```rust
use ouroboros_ui::atoms::Divider;

Divider::horizontal().show(ui);
```

```rust
use ouroboros_ui::atoms::Divider;

// tab underline indicator
Divider::horizontal().thick().color(theme.primary).show(ui);

// vertical separator in a toolbar row
Divider::vertical().show(ui);
```

## Composition

Atom: paints a single stroke directly. Composes no other atoms.

## Notes

- A horizontal divider greedily takes the full available width; place it where that is intended, or constrain the parent `Ui`.
- `Axis` is re-exported and reused by [`SplitterHandle`](splitter_handle.md).

See [tokens](../../tokens.md) · [theming](../../theming.md).
