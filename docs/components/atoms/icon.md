# Icon

> **Layer:** atom · **Path:** `src/atoms/icon.rs` · **Exports:** `icon::Icon`

A single Phosphor glyph at an icon-size token, in a theme color. The glyph is a `&'static str` constant from [`egui_phosphor::light`](../../typography.md) (re-exported at the crate root). The font always comes from `typography::icon_font` — atoms never build a `FontId` directly.

## Design

- **Purpose / when to use** — standalone iconography (status marks, decorative glyphs, list bullets). For an icon *inside* a button, prefer `Button::icon_left/icon_right` (single-galley alignment) rather than composing this atom.
- **Anatomy** — a non-selectable `egui::Label` with `RichText::new(glyph).font(icon_font(size)).color(color)`.
- **Variants / sizes / states**

  | Size shorthand | Token |
  |----------------|-------|
  | `.sm()` | `ICON_SM` 14 |
  | `.md()` (default) | `ICON_MD` 16 |
  | `.lg()` | `ICON_LG` 20 |
  | `.xl()` | `ICON_XL` 24 |
  | `.size(f32)` | arbitrary px |

  Color: defaults to `theme.foreground`; `.muted()` → `theme.muted_foreground`; `.color(c)` → explicit. No interactive states.

- **Tokens consumed** — `theme.foreground` / `theme.muted_foreground` (color), `core::ICON_SM..XL` (size), `typography::icon_font` (font).
- **Accessibility** — rendered as **non-selectable** so the glyph never steals a click from an interactive parent. Returns the `Label` `Response`.

## API

| Signature | Effect |
|-----------|--------|
| `Icon::new(glyph: &'static str) -> Self` | Construct from a `light::*` constant. |
| `.size(size: f32) -> Self` | Set size in px. |
| `.sm()` / `.md()` / `.lg()` / `.xl()` | Token-size shorthands. |
| `.muted(self) -> Self` | Use `muted_foreground`. |
| `.color(color: Color32) -> Self` | Explicit color. |
| `.show(self, ui: &mut Ui) -> Response` | Render and return the `Response`. |

## Usage

```rust
use ouroboros_ui::atoms::Icon;
use ouroboros_ui::egui_phosphor::light;

Icon::new(light::GEAR).show(ui);
```

```rust
use ouroboros_ui::atoms::Icon;
use ouroboros_ui::egui_phosphor::light;

Icon::new(light::CHECK_CIRCLE).lg().color(theme.success).show(ui);
Icon::new(light::INFO).muted().show(ui);
```

## Composition

Atom: renders directly via `egui::Label`/`RichText`. Composes no other atoms. Note: [`Button`](button.md)/[`Toggle`](toggle.md) do **not** compose this atom — they inline the glyph into a single galley.

## Notes

- `glyph` must be a `&'static str` (the Phosphor constants are). Arbitrary runtime strings will render whatever codepoints they contain in the icon font.
- `.color()` wins over `.muted()`.

See [tokens](../../tokens.md) · [theming](../../theming.md) · [typography](../../typography.md).
