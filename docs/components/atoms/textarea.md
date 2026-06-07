# Textarea

> **Layer:** atom · **Path:** `src/atoms/textarea.rs` · **Exports:** `textarea::Textarea`

A multi-line text field over a `&mut String` — the sibling of [`Input`](input.md). A token-painted box (fill `muted`, border `input`/`destructive`/`ring`, animated hover veil) wraps a frameless multiline `egui::TextEdit`. Height derives from the requested row count.

## Design

- **Purpose / when to use** — multi-line free text (descriptions, notes, JSON blobs). For single-line use [`Input`](input.md).
- **Anatomy** — filled rect (`muted`) → hover veil → inner frameless multiline `TextEdit` (`body` font, `foreground` text, `muted_foreground` placeholder, `SPACE_2` inset) → state border stroke.
- **Variants / sizes / states**
  - `rows(n)` (min 1, default 3) — sets height = `n × body.line_height + 2·SPACE_2`.
  - **Border state** (precedence): error → `destructive`; else focused → `ring` @ `BORDER_FOCUS`; else `input`. Disabled dims and uses `add_enabled(false, …)`. (No size-scale variants, unlike Input.)
- **Tokens consumed** — `theme.muted` (fill), `theme.input`/`theme.destructive`/`theme.ring` (border), `theme.foreground` (text), `theme.muted_foreground` (placeholder), `theme.hover_overlay`, `core::RADIUS_MD`, `core::SPACE_2` (padding), `core::BORDER_THIN`/`BORDER_FOCUS`, `core::hover_t`, `core::disabled_color`, `typography::body`.
- **Accessibility** — focus is egui's `TextEdit` focus; the border switches to the ring on focus.

## API

| Signature | Effect |
|-----------|--------|
| `Textarea::new(buf: &mut String) -> Self` | Bind to a string buffer (3 rows). |
| `.rows(rows: usize) -> Self` | Visible row count (min 1). |
| `.placeholder(text: impl Into<String>) -> Self` | Hint text when empty. |
| `.error(error: bool) -> Self` | Force the destructive border. |
| `.enabled(enabled: bool) -> Self` / `.disabled()` | Enable/disable. |
| `.id_source(id: impl Hash) -> Self` | Stable id for the inner `TextEdit` (else auto-id). |
| `.show(self, ui: &mut Ui) -> Response` | Return the `TextEdit` `Response` (`changed` when edited). |

## Usage

```rust
use ouroboros_ui::atoms::Textarea;

let mut notes = String::new();
Textarea::new(&mut notes).placeholder("Notes…").show(ui);
```

```rust
use ouroboros_ui::atoms::Textarea;

let mut desc = String::new();
let resp = Textarea::new(&mut desc).rows(6).id_source("desc").show(ui);
if resp.changed() { /* … */ }
```

## Composition

Atom: paints the box/border/veil directly and embeds a frameless multiline `egui::TextEdit` in a child `Ui`. Composes no other DS atoms.

## Notes

- Binding is `&mut String`; the returned `Response` is the inner `TextEdit`'s.
- Height is fixed by `rows` (it does not auto-grow with content); width is `ui.available_width()`.
- Unlike [`Input`](input.md), there is **no** `Size` variant — only `rows`.

See [tokens](../../tokens.md) · [theming](../../theming.md) · [typography](../../typography.md) · [guards](../../guards.md).
