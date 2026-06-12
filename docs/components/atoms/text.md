# Text

> **Layer:** atom · **Path:** `src/atoms/text.rs` · **Exports:** `text::{Text, TextRole}`

A run of text at a typography role, in a theme color. Every visual comes from a token: font/size/line-height/tracking from a [`typography`](../../typography.md) type style, color from the [`Theme`]. The DS's universal text primitive; many other atoms compose it for their labels.

## Design

- **Purpose / when to use** — all body copy, labels, captions, inline code, and key glyphs. For titles use [`Heading`](heading.md).
- **Anatomy** — a non-selectable `egui::Label` built from `RichText` with the role's `font_id` + `tracking` + color, optional underline, and a wrap mode.
- **Variants / sizes / states**

  **`TextRole`** → type style:

  | Role | Style |
  |------|-------|
  | `Body` (default) | `typography::body()` |
  | `BodyStrong` | `body_strong()` |
  | `Label` | `label()` |
  | `LabelStrong` | `label_strong()` |
  | `Caption` | `caption()` |
  | `Code` | `code()` |
  | `Kbd` | `kbd()` |

  Color: `theme.foreground` (default), `.muted()` → `muted_foreground`, `.color(c)` → explicit. `.wrap()` enables wrapping (and line-height); `.underline()` for links; `.italic()` for asides/hints (combines freely, e.g. `.muted().italic()`). No interactive states.

- **Tokens consumed** — `theme.foreground` / `theme.muted_foreground` (color), the per-role `typography` style.
- **Accessibility** — rendered **non-selectable** so UI text never captures the pointer (it would steal clicks from interactive parents and show a text cursor).

## API

| Signature | Effect |
|-----------|--------|
| `Text::new(content: impl Into<String>) -> Self` | Construct with text. |
| `.role(role: TextRole) -> Self` | Set role. |
| `.body_strong()` / `.label()` / `.label_strong()` / `.caption()` / `.code()` / `.kbd()` | Role shorthands. |
| `.muted(self) -> Self` | Use `muted_foreground`. |
| `.color(color: Color32) -> Self` | Explicit color (e.g. `theme.success`). |
| `.wrap(self) -> Self` | Wrap on available width (default: extend/no wrap). |
| `.underline(self) -> Self` | Underline (e.g. a link). |
| `.italic(self) -> Self` | Italicize (e.g. an aside/hint nuance). |
| `.show(self, ui: &mut Ui) -> Response` | Render and return the `Response`. |

**`TextRole`** (enum): `Body` (default), `BodyStrong`, `Label`, `LabelStrong`, `Caption`, `Code`, `Kbd`.

## Usage

```rust
use ouroboros_ui::atoms::Text;

Text::new("Hello world").show(ui);
```

```rust
use ouroboros_ui::atoms::Text;

Text::new("Saved").caption().color(theme.success).show(ui);
Text::new("A long paragraph that should wrap…").wrap().show(ui);
Text::new("No layers selected").muted().italic().show(ui);
```

## Composition

Atom: renders directly via `egui::Label`/`RichText` with token styling. It is itself composed by [`Checkbox`](checkbox.md), [`Radio`](radio.md), and [`Tooltip`](tooltip.md) for their labels.

## Notes

- Default wrap mode is `Extend` (no wrap); line-height is only applied when `.wrap()` is set — applying leading to a single line inflates the row and decenters the glyph inside parents like buttons.
- `.color()` wins over `.muted()`.

See [tokens](../../tokens.md) · [theming](../../theming.md) · [typography](../../typography.md).
