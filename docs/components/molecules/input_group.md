# InputGroup

> **Layer:** molecule · **Path:** `src/molecules/input_group.rs` · **Exports:** `input_group::{InputGroup, Slot}`

A text input (or multi-line [`Textarea`](../atoms/textarea.md)) with addons sharing one muted [`Surface`](../atoms/surface.md). Addons — icons, text, or buttons — sit in four slots: leading/trailing inline (on the field's centerline) and block start/end (their own rows above/below). `.multiline(rows)` switches the editing substrate to a `Textarea`. Models shadcn's `InputGroupAddon` / `InputGroupText` / `InputGroupButton`.

## Design

- **Purpose / when to use** — Inputs that need affordances: a leading search icon, a trailing clear button, a `$`/unit prefix, or a labeled block addon over a textarea.
- **Anatomy** — `Surface::muted().pad(SPACE_1).radius(RADIUS_MD)` → vertical stack: a `BlockStart` row (if any) → the field → a `BlockEnd` row (if any). In single-line mode the field is a fixed-height (`CONTROL_MD`) center-aligned row of `LeadingInline` addons + a frameless `TextEdit::singleline` (placeholder + text styled from [typography](../../typography.md)) + `TrailingInline` addons. In multiline mode it's a [`Textarea`](../atoms/textarea.md) (inline addons ignored).
- **Slots (`Slot`)** — `LeadingInline`, `TrailingInline`, `BlockStart`, `BlockEnd`.
- **Addon kinds** — icon (muted [`Icon`](../atoms/icon.md)), text (muted [`Text`](../atoms/text.md)), button (ghost icon-only [`Button`](../atoms/button.md), runs an `FnMut` on click).
- **Tokens / layout consumed** — `core::SPACE_1` (surface pad / block gaps), `SPACE_2` (addon spacing + trailing reserve), `CONTROL_MD` (inline row height), `RADIUS_MD`. See [tokens](../../tokens.md).

## API

| Method | Effect |
|---|---|
| `InputGroup::new(buf: &'a mut String) -> Self` | Bind the text buffer. |
| `.placeholder(text: impl Into<String>) -> Self` | Hint text shown when empty. |
| `.multiline(rows: usize) -> Self` | Switch to a `Textarea` of `rows` rows (≥1; inline addons ignored). |
| `.id_source(id: impl std::hash::Hash) -> Self` | Stable id for the editor. |
| `.icon(slot: Slot, glyph: &'static str) -> Self` | Add an icon addon in `slot`. |
| `.text(slot: Slot, text: impl Into<String>) -> Self` | Add a text addon in `slot`. |
| `.button(slot: Slot, glyph: &'static str, action: impl FnMut() + 'a) -> Self` | Add a clickable icon-button addon; `action` runs on click. |
| `.leading_icon(glyph: &'static str) -> Self` | Sugar — icon in `LeadingInline`. |
| `.leading_text(text: impl Into<String>) -> Self` | Sugar — text in `LeadingInline`. |
| `.show(self, ui: &mut Ui) -> Response` | Render; returns the field `Response` (`.changed()` on edit). |

**`Slot`** — `LeadingInline`, `TrailingInline`, `BlockStart`, `BlockEnd`.

## Usage

```rust
use ouroboros_ui::molecules::{InputGroup, Slot};
use ouroboros_ui::egui_phosphor::light;

// minimal — leading search icon + trailing clear button
InputGroup::new(&mut query)
    .leading_icon(light::MAGNIFYING_GLASS)
    .button(Slot::TrailingInline, light::X, || { query.clear(); })
    .placeholder("Search…")
    .id_source("ig_search")
    .show(ui);
```

```rust
use ouroboros_ui::molecules::{InputGroup, Slot};

// text prefix/suffix
InputGroup::new(&mut price)
    .leading_text("$")
    .text(Slot::TrailingInline, "USD")
    .placeholder("0.00")
    .id_source("ig_price")
    .show(ui);

// block addon over a multiline textarea
InputGroup::new(&mut note)
    .text(Slot::BlockStart, "Description")
    .multiline(3)
    .placeholder("Markdown supported…")
    .id_source("ig_note")
    .show(ui);
```

## Composition

Composes [`Surface`](../atoms/surface.md) + [`Textarea`](../atoms/textarea.md) + [`Icon`](../atoms/icon.md) + [`Text`](../atoms/text.md) + [`Button`](../atoms/button.md). The single-line editor is an `egui::TextEdit::singleline` (the editing substrate, not a paint call), styled from [typography](../../typography.md). It never paints primitives — see the [guards](../../guards.md).

## Notes

- `InputGroup<'a>` carries a lifetime — the button `action` is `Box<dyn FnMut() + 'a>` and may capture/mutate surrounding state (and even the bound buffer, as in the clear example).
- In single-line mode, trailing-inline addons reserve `(CONTROL_MD + SPACE_2)` width each so the text doesn't run under them.
- `.multiline(rows)` ignores inline addons — only block addons apply to a textarea.
- The returned `Response` is the field's; `.changed()` is true when the text was edited this frame.
- [`SearchField`](search_field.md) is a thin preset over this molecule.
