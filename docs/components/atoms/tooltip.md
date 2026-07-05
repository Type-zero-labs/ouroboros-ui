# Tooltip

> **Layer:** atom · **Path:** `src/atoms/tooltip.rs` · **Exports:** `tooltip::Tooltip`

A DS-styled hover tooltip attached to an existing `Response`. `Tooltip::new("…").show(response)` shows the text on hover, composing the [`Text`](text.md) atom inside egui's `on_hover_ui`. Unlike every other atom, its `show` takes a `Response` (not a `&mut Ui`).

## Design

- **Purpose / when to use** — attach explanatory hover text to any widget's response (icon buttons, truncated labels, controls). Keep it short; for rich popovers build a molecule.
- **Anatomy** — egui's hover container, with the body rendered by a single [`Text`](text.md) atom (default `Body` role / `foreground` color).
- **Variants / sizes / states** — none; shown only while the host response is hovered.
- **Tokens consumed** — indirectly via the [`Text`](text.md) atom (typography `body`, `theme.foreground`). The hover container chrome is egui's default styling.
- **Accessibility** — uses egui's `on_hover_ui` (hover-triggered).

## API

| Signature | Effect |
|-----------|--------|
| `Tooltip::new(text: impl Into<String>) -> Self` | Construct with the tooltip text. |
| `.show(self, response: Response) -> Response` | Attach the tooltip to `response` (shown on hover); returns the same `Response` for chaining. |

## Usage

```rust
use ouroboros_ui::atoms::{Button, Tooltip};
use ouroboros_ui::egui_phosphor::light;

let resp = Button::new("").icon_left(light::TRASH).icon_only().ghost().show(ui);
let resp = Tooltip::new("Delete").show(resp);
if resp.clicked() { /* … */ }
```

## Composition

Atom: composes the [`Text`](text.md) atom for its body inside `Response::on_hover_ui`. Paints nothing itself.

## Notes

- `show` takes and returns a `Response` — it wraps an already-shown widget rather than allocating in a `Ui`. This is the API outlier among the atoms.
- It returns the response, so you can chain `.clicked()`/`.hovered()` directly.

See [tokens](../../tokens.md) · [theming](../../theming.md) · [typography](../../typography.md).
