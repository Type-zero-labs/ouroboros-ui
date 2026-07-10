# Progress

> **Layer:** atom · **Path:** `src/atoms/progress.rs` · **Exports:** `progress::Progress`

A **determinate** progress indicator (`fraction` in `0..=1`), rendered as a continuous bar, a stepped (segmented) bar, or a circular ring. Modeled on shadcn Progress / Unity Progress Bar. For indeterminate loading use [`Spinner`](spinner.md).

## Design

- **Purpose / when to use** — show known completion progress. Use the ring for compact/inline placement, steps for discrete stages, the bar for general progress.
- **Anatomy**
  - **Bar (continuous)**: pill track (`muted`) + a `primary` fill rect from the left, width = `track × fraction`.
  - **Bar (stepped)**: `n` equal pills with `SPACE_1` gaps; the first `round(fraction × n)` are `primary`, the rest `muted`.
  - **Ring**: a `muted` circle stroke + a `primary` arc sweeping `fraction × 360°` from 12 o'clock.
- **Variants / sizes / states**

  | Builder | Render |
  |---------|--------|
  | `new(fraction)` | continuous bar, `SPACE_2` tall, full width |
  | `.steps(n)` | `n` segments (≥1) |
  | `.circular()` | ring at `CONTROL_LG` (38px) diameter |

  No interactive states (sense is `hover`).

- **Tokens consumed** — `theme.muted` (track), `theme.primary` (fill/arc), `core::SPACE_2` (bar height / circular thickness via `SPACE_1`), `core::SPACE_1` (segment gap & ring thickness), `core::CONTROL_LG` (default ring size).
- **Accessibility** — bare hover `Response`; no `widget_info`.

## API

| Signature | Effect |
|-----------|--------|
| `Progress::new(fraction: f32) -> Self` | Construct; `fraction` clamped to `0..=1`. |
| `.steps(n: usize) -> Self` | Render `n` discrete segments (min 1). |
| `.circular(self) -> Self` | Render as a ring at the default diameter. |
| `.show(self, ui: &mut Ui) -> Response` | Paint and return the hover `Response`. |

## Usage

```rust
use ouroboros_ui::atoms::Progress;

Progress::new(0.42).show(ui);
```

```rust
use ouroboros_ui::atoms::Progress;

Progress::new(0.6).steps(5).show(ui);       // stepped bar
Progress::new(0.6).circular().show(ui);     // ring
```

## Composition

Atom: paints rects/strokes/arc directly. Composes no other atoms.

## Notes

- `fraction` is clamped on construction — out-of-range values are safe.
- The bar variant takes `ui.available_width()`; the ring is fixed-size.
- This is determinate only; there is no indeterminate bar — use [`Spinner`](spinner.md).

See [tokens](../../tokens.md) · [theming](../../theming.md).
