# Spinner

> **Layer:** atom · **Path:** `src/atoms/spinner.rs` · **Exports:** `spinner::Spinner`

An **indeterminate** loading arc: a ~270° stroked arc that rotates over time (requesting a repaint each frame). Size and color are tokens. Implements `Default`. For known progress use [`Progress`](progress.md).

## Design

- **Purpose / when to use** — signal in-flight work of unknown duration. Inline or centered in a panel; the same arc form is inlined into [`Button`](button.md)`::loading`.
- **Anatomy** — a single 32-point polyline approximating a `0.75·TAU` (270°) arc, stroked at `BORDER_FOCUS`, with the start angle = `time × TAU` so it spins.
- **Variants / sizes / states** — `.sm()` (`ICON_SM` 14), default `ICON_MD` 16, `.lg()` (`ICON_LG` 20), or `.size(f32)`. Color defaults to `muted_foreground`, overridable via `.color(c)`. No interactive states.
- **Tokens consumed** — `theme.muted_foreground` (default color), `core::ICON_SM/MD/LG` (size), `core::BORDER_FOCUS` (stroke width + radius inset).
- **Accessibility** — bare hover `Response`; no `widget_info`. Animates continuously (`request_repaint`).

## API

| Signature | Effect |
|-----------|--------|
| `Spinner::new() -> Self` | Construct (`ICON_MD`, `muted_foreground`). |
| `Spinner::default()` | Same as `new()`. |
| `.size(size: f32) -> Self` | Set diameter. |
| `.sm()` / `.lg()` | Token-size shorthands. |
| `.color(color: Color32) -> Self` | Override color. |
| `.show(self, ui: &mut Ui) -> Response` | Paint the spinning arc and return the hover `Response`. |

## Usage

```rust
use ouroboros_ui::atoms::Spinner;

Spinner::new().show(ui);
```

```rust
use ouroboros_ui::atoms::Spinner;

Spinner::new().lg().color(theme.primary).show(ui);
```

## Composition

Atom: paints the arc directly via a `Shape::line` polyline. Composes no other atoms.

## Notes

- Calls `ui.ctx().request_repaint()` every frame it is shown — only render it while actually loading.
- The arc form is duplicated (not composed) inside `Button::loading`.

See [tokens](../../tokens.md) · [theming](../../theming.md).
