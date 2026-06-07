# Alert

> **Layer:** molecule · **Path:** `src/molecules/alert.rs` · **Exports:** `alert::{Alert, AlertVariant}`

A status callout — an inline banner that surfaces a build result, validation outcome, or notice. The variant drives both the leading glyph and an accent color pulled from the active [`Theme`]. Analogous to the shadcn Alert / Unity Help Box.

## Design

- **Purpose / when to use** — Communicate a contextual status (info / success / warning / error) inline in a panel. Not a toast; it stays in the layout flow.
- **Anatomy** — [`Surface`](../atoms/surface.md) container, padded `SPACE_3` → horizontal row of a status [`Icon`](../atoms/icon.md) (accent-colored) + a vertical text stack of an optional accent [`Text`](../atoms/text.md) title (`body_strong`) and the muted message body.
- **Variants / states**

  | Variant | Glyph (`egui_phosphor::light`) | Theme color |
  |---|---|---|
  | `Info` *(default)* | `INFO` | `theme.info` |
  | `Success` | `CHECK_CIRCLE` | `theme.success` |
  | `Warning` | `WARNING` | `theme.warning` |
  | `Error` | `WARNING_CIRCLE` | `theme.error` |

- **Tokens / layout consumed** — `core::SPACE_3` (surface pad), `SPACE_2` (icon→text gap), `SPACE_1` (title→message gap); colors from [`Theme`](../../theming.md). See [tokens](../../tokens.md).

## API

| Method | Effect |
|---|---|
| `Alert::new(message: impl Into<String>) -> Self` | Construct with the body message; variant defaults to `Info`. |
| `.title(title: impl Into<String>) -> Self` | Optional accent-colored title above the message. |
| `.variant(variant: AlertVariant) -> Self` | Set the variant explicitly. |
| `.info() / .success() / .warning() / .error() -> Self` | Sugar for `.variant(...)`. |
| `.show(self, ui: &mut Ui) -> Response` | Render; returns the surface's `Response`. |

**`AlertVariant`** — `Info` (default), `Success`, `Warning`, `Error`.

## Usage

```rust
use ouroboros_ui::molecules::Alert;

// minimal
Alert::new("Build finished in 2.3s.").show(ui);
```

```rust
use ouroboros_ui::molecules::{Alert, AlertVariant};

// realistic — titled error
Alert::new("Shader failed to compile.")
    .title("Notice")
    .variant(AlertVariant::Error)
    .show(ui);
```

## Composition

Composes [`Surface`](../atoms/surface.md) + [`Icon`](../atoms/icon.md) + [`Text`](../atoms/text.md) only, laid out with `ui.horizontal` / `ui.vertical`. It never paints primitives — see the [guards](../../guards.md).

## Notes

- The message is required; the title is optional and renders in the variant accent color.
- Color resolution happens at `show` time via `Theme::get(ui)`, so alerts re-theme automatically.
