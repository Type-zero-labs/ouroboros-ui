# Skeleton

> **Layer:** atom · **Path:** `src/atoms/skeleton.rs` · **Exports:** `skeleton::Skeleton`

A loading placeholder block: a `muted` rounded rect that gently pulses (opacity) while content loads. Modeled on shadcn Skeleton. Implements `Default`.

## Design

- **Purpose / when to use** — reserve layout space and signal loading before real content arrives. For an active spinner use [`Spinner`](spinner.md); for known progress use [`Progress`](progress.md).
- **Anatomy** — a single `muted`-filled rounded rect (`RADIUS_SM`). When pulsing, its opacity oscillates via a sine of `ui.input(time)` between `OPACITY_MUTED` and `1.0`, requesting a repaint each frame.
- **Variants / sizes / states** — `width(f32)` (default: `ui.available_width()`), `height(f32)` (default `SPACE_4` = 16px), `still()` to disable the pulse. No interactive states (sense `hover`).
- **Tokens consumed** — `theme.muted` (fill), `core::RADIUS_SM`, `core::SPACE_4` (default height), `core::OPACITY_MUTED` (pulse floor).
- **Accessibility** — bare hover `Response`; no `widget_info`.

## API

| Signature | Effect |
|-----------|--------|
| `Skeleton::new() -> Self` | Construct (pulsing, full-width, 16px tall). |
| `Skeleton::default()` | Same as `new()`. |
| `.width(width: f32) -> Self` | Fixed width. |
| `.height(height: f32) -> Self` | Set height. |
| `.still(self) -> Self` | Disable the pulse animation. |
| `.show(self, ui: &mut Ui) -> Response` | Paint and return the hover `Response`. |

## Usage

```rust
use ouroboros_ui::atoms::Skeleton;

Skeleton::new().show(ui);                       // full-width pulsing line
```

```rust
use ouroboros_ui::atoms::Skeleton;

// an avatar-sized still placeholder
Skeleton::new().width(32.0).height(32.0).still().show(ui);
```

## Composition

Atom: paints one rect directly. Composes no other atoms.

## Notes

- Default width is greedy (`ui.available_width()`); pass `.width(..)` for a fixed block.
- The pulse drives continuous repaints — call `.still()` for large grids of placeholders to avoid animating many at once.

See [tokens](../../tokens.md) · [theming](../../theming.md).
