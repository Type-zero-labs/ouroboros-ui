# Typography

`src/theme/typography.rs`. Registers the bundled fonts and exposes composite type styles
(family + size + line-height + tracking) for the named roles. Sizes and leadings come
from [core](./tokens.md); this layer composes them into usable styles.

---

## Faces

Two type families are vendored under `assets/fonts/` and embedded with `include_bytes!`:

- **Iosevka** (UI) — five weights: Light, Regular, Medium, SemiBold, Bold.
- **IosevkaTerm** (code/keyboard) — Regular, Bold.
- **Phosphor Light** — icon glyphs, via `egui-phosphor`.

Each weight is registered under its own named `FontFamily::Name`, so a `TypeStyle` can
target an exact face. The default `Proportional` stack is Iosevka Regular; `Monospace` is
IosevkaTerm. Phosphor is appended as an **icon fallback to every face**, so inline icons
resolve no matter which type style renders them.

```rust
pub enum Weight { Light, Regular, Medium, SemiBold, Bold }
```

---

## Named type styles

Each function returns a `TypeStyle { family, size, line_height, tracking }`. Build an
`egui::FontId` with `.font_id()`; the line-height and tracking are applied by the text
atom when it lays out the galley.

| Style | Face / weight | Size | Tracking | Use |
|-------|---------------|------|----------|-----|
| `display()` | Iosevka Bold | 30 | normal | largest title |
| `h1()` | Iosevka SemiBold | 24 | normal | page title |
| `h2()` | Iosevka SemiBold | 20 | normal | section title |
| `heading()` | Iosevka SemiBold | 16 | sm | sub-section heading |
| `body()` | Iosevka **Light** | 14 | md | default body text |
| `body_strong()` | Iosevka Medium | 14 | md | emphasized body |
| `label()` | Iosevka **Light** | 13 | lg | default label |
| `label_strong()` | Iosevka Medium | 13 | lg | emphasized label |
| `caption()` | Iosevka Regular | 12 | wide | small / caption |
| `code()` | IosevkaTerm Regular | 13 | lg | inline code |
| `kbd()` | IosevkaTerm Bold | 12 | wide | keyboard key cap |

Notes on the choices:

- **Body and label default to Light**, not Regular — the dense IDE aesthetic. Reach for
  the `_strong` (Medium) variants when a line needs to assert itself.
- **Headings keep normal tracking**; the smaller the text, the wider the tracking
  (legibility scale is inverse to size, defined in [core](./tokens.md#typography-primitives)).
- **`kbd` is Bold mono** because a Medium mono weight isn't vendored, and Bold reads as a
  key cap anyway.

---

## `TypeStyle`

```rust
pub struct TypeStyle {
    pub family: FontFamily,   // includes weight (named family)
    pub size: f32,
    pub line_height: f32,     // px = size × leading
    pub tracking: f32,        // extra px per glyph
}

impl TypeStyle {
    pub fn font_id(&self) -> FontId;   // family + size
}
```

`line_height` and `tracking` are not part of egui's `FontId`, so they are carried on the
`TypeStyle` and applied by the [`Text`](./components/atoms/text.md) atom during layout.
That is why text should go through the `Text`/`Heading` atoms rather than raw
`ui.label` — only the atoms honor the full type style.

---

## Icons

```rust
pub fn icon_font(size: f32) -> FontId;   // Phosphor glyph at `size`
```

Phosphor glyphs are PUA codepoints resolved via the proportional stack's icon fallback.
Atoms call `icon_font` instead of building a `FontId` by hand. Glyph constants come from
the re-exported crate:

```rust
use ouroboros_ui::egui_phosphor::light::GEAR;   // a &'static str glyph
```

The [`Icon`](./components/atoms/icon.md) atom is the normal way to render one; the raw
font is for atoms that fold an icon into a larger galley (e.g. `Button`).

---

## The `Size` → type-style mapping

The mapping from the shared control [`Size`](./tokens.md#size-enum--the-shared-control-scale)
scale to a type style lives **here**, not in `core`, so the token layer stays a leaf
(`theme` may reference `tokens`, not the reverse):

```rust
impl Size {
    pub fn text_style(self) -> TypeStyle {
        match self {
            Size::Lg => body_strong(),       // roomy controls
            Size::Sm | Size::Md => label(),  // dense controls
        }
    }
}
```
</content>
