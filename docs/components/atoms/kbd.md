# Kbd

> **Layer:** atom · **Path:** `src/atoms/kbd.rs` · **Exports:** `kbd::Kbd`

A keyboard-key chip: mono (`kbd`) text in a small token-bordered box — e.g. `⌘K`, `Ctrl`, `Esc`. Modeled on shadcn's Kbd.

## Design

- **Purpose / when to use** — display a keyboard shortcut inline (menus, tooltips, command palettes). Read-only.
- **Anatomy** — rounded rect (`muted` fill + `border` stroke, `RADIUS_SM`) with `(SPACE_2, SPACE_1)` padding around a centered `kbd`-style galley in `muted_foreground`.
- **Variants / sizes / states** — none; single appearance, sense is `hover` only.
- **Tokens consumed** — `theme.muted` (fill), `theme.border` (stroke), `theme.muted_foreground` (text), `core::RADIUS_SM`, `core::SPACE_2`/`SPACE_1` (padding), `core::BORDER_THIN`, `typography::kbd`.
- **Accessibility** — bare hover `Response`; no `widget_info`.

## API

| Signature | Effect |
|-----------|--------|
| `Kbd::new(keys: impl Into<String>) -> Self` | Construct with the key text. |
| `.show(self, ui: &mut Ui) -> Response` | Paint and return the hover `Response`. |

## Usage

```rust
use ouroboros_ui::atoms::Kbd;

Kbd::new("Esc").show(ui);
Kbd::new("⌘K").show(ui);
```

## Composition

Atom: paints the box and builds the text galley inline with the `kbd` type style. Does not compose [`Text`](text.md), though `TextRole::Kbd` uses the same style.

## Notes

- The whole string is rendered in one chip; for multi-key combos either pass `"Ctrl+K"` as one string or place several `Kbd`s with separators.

See [tokens](../../tokens.md) · [theming](../../theming.md) · [typography](../../typography.md).
