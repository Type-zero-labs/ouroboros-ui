# Heading

> **Layer:** atom · **Path:** `src/atoms/heading.rs` · **Exports:** `heading::{Heading, HeadingLevel}`

A title rendered in the `foreground` token at one of four heading levels. Each level maps to a foundation [`typography`](../../typography.md) type style (font, size, line-height, tracking). Sibling of [`Text`](text.md), specialized for titles.

## Design

- **Purpose / when to use** — section/page titles. For body copy, labels, captions, or code use [`Text`](text.md).
- **Anatomy** — a single non-selectable `egui::Label` built from a `RichText` carrying the level's `font_id`, `line_height`, `tracking`, and `theme.foreground`. Wrap mode is `Extend` (no wrap).
- **Variants / sizes / states**

  | Level | Type style |
  |-------|------------|
  | `Display` | `typography::display()` (largest) |
  | `H1` | `typography::h1()` |
  | `H2` (default) | `typography::h2()` |
  | `Heading` | `typography::heading()` (smallest) |

  No interactive states.

- **Tokens consumed** — `theme.foreground` (color), the per-level `typography` style (font/size/line-height/tracking).
- **Accessibility** — renders a non-selectable label; returns the `Label`'s `Response`.

## API

| Signature | Effect |
|-----------|--------|
| `Heading::new(content: impl Into<String>) -> Self` | Construct with title text. |
| `.level(level: HeadingLevel) -> Self` | Set the level. |
| `.display()` / `.h1()` / `.h2()` / `.heading()` | Level shorthands. |
| `.show(self, ui: &mut Ui) -> Response` | Render and return the `Response`. |

**`HeadingLevel`** (enum): `Display`, `H1`, `H2` (default), `Heading`.

## Usage

```rust
use ouroboros_ui::atoms::Heading;

Heading::new("Settings").show(ui);
```

```rust
use ouroboros_ui::atoms::{Heading, HeadingLevel};

Heading::new("Welcome").level(HeadingLevel::Display).show(ui);
Heading::new("Section").heading().show(ui);
```

## Composition

Atom: renders directly via `egui::Label`/`RichText` with token styling. Does not compose [`Text`](text.md) (it is a parallel specialization).

## Notes

- Wrap mode is `Extend` — headings do not wrap; size the parent or keep titles short.
- Unlike [`Text`](text.md), there is no color override; the color is always `theme.foreground`.

See [tokens](../../tokens.md) · [theming](../../theming.md) · [typography](../../typography.md).
