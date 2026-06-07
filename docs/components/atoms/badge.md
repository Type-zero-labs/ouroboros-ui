# Badge

> **Layer:** atom · **Path:** `src/atoms/badge.rs` · **Exports:** `badge::{Badge, BadgeVariant}`

A static, non-interactive pill label with the shadcn variant set plus three domain status variants (`Success`/`Warning`/`Info`). Variant resolves to a [`BadgeTokens`] bundle (fill/foreground/border/underline); the atom paints a rounded pill, an optional leading status dot, and a centered text galley.

## Design

- **Purpose / when to use** — annotate state, count, or category inline (e.g. "Beta", "3", a status dot). It is read-only; for a clickable chip use [`Button`](button.md) or [`Toggle`](toggle.md).
- **Anatomy** — pill rect (radius = half-height) → optional fill → optional border stroke → optional leading dot (in `foreground`) → centered text galley.
- **Variants / sizes / states** — no hover/focus/disabled (sense is `hover` only).

  **Variants** (`BadgeVariant`): `Default`, `Secondary`, `Destructive`, `Outline`, `Ghost`, `Link`, `Success`, `Warning`, `Info`. Each maps to a `BadgeTokens::*` constructor; `Link` carries `underline = true`.

  | Size | Padding (x, y) | Text style |
  |------|----------------|------------|
  | `Sm` | `(SPACE_1, SPACE_1)` | `caption()` |
  | `Md` (default) | `(SPACE_2, SPACE_1)` | `caption()` |
  | `Lg` | `(SPACE_3, SPACE_1)` | `label()` |

- **Tokens consumed** — `BadgeTokens` (fill/border/foreground/underline, per variant), `core::SPACE_1..3` (padding/gap/dot), `core::BORDER_THIN` (border + underline stroke). Fill/border are only painted when alpha > 0.
- **Accessibility** — none beyond the bare `Response`; no `widget_info`.

## API

| Signature | Effect |
|-----------|--------|
| `Badge::new(text: impl Into<String>) -> Self` | Construct with label text. |
| `.variant(variant: BadgeVariant) -> Self` | Set variant. |
| `.size(size: Size) -> Self` | Set size (`core::Size`). |
| `.sm(self) -> Self` / `.lg(self) -> Self` | Size shorthands. |
| `.secondary()` / `.destructive()` / `.outline()` / `.ghost()` / `.link()` / `.success()` / `.warning()` / `.info()` | Variant shorthands. |
| `.dot(self) -> Self` | Show a leading colored status dot. |
| `.show(self, ui: &mut Ui) -> Response` | Paint and return the hover `Response`. |

**`BadgeVariant`** (enum): `Default` (default), `Secondary`, `Destructive`, `Outline`, `Ghost`, `Link`, `Success`, `Warning`, `Info`.

## Usage

```rust
use ouroboros_ui::atoms::Badge;

Badge::new("New").show(ui);
```

```rust
use ouroboros_ui::atoms::{Badge, BadgeVariant};

Badge::new("Online").success().dot().show(ui);
Badge::new("Deprecated").variant(BadgeVariant::Destructive).show(ui);
```

## Composition

Atom: paints the pill and builds the text galley inline with token fonts/colors. Does not compose [`Text`](text.md) or [`Icon`](icon.md).

## Notes

- The dot uses `core::SPACE_2` diameter and is filled with `bt.foreground`.
- `Link` is the only variant that draws an underline (via `BadgeTokens.underline`).
- Pill corner radius is computed from the final rect height, so it stays fully rounded at any size.

See [tokens](../../tokens.md) · [theming](../../theming.md) · [typography](../../typography.md).
