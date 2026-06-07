# SearchField

> **Layer:** molecule · **Path:** `src/molecules/search_field.rs` · **Exports:** `search_field::SearchField`

A search input preset: a thin wrapper over [`InputGroup`](input_group.md) configured with a leading magnifier icon. Inspired by Unity's Search Field.

## Design

- **Purpose / when to use** — Any filter/search box. Use it instead of hand-wiring an `InputGroup` with a search glyph.
- **Anatomy** — An [`InputGroup`](input_group.md) bound to your buffer, with `leading_icon(light::MAGNIFYING_GLASS)` and an optional placeholder.
- **Tokens / layout consumed** — inherited from [`InputGroup`](input_group.md). See [tokens](../../tokens.md).

## API

| Method | Effect |
|---|---|
| `SearchField::new(buf: &'a mut String) -> Self` | Bind the search buffer. |
| `.placeholder(text: impl Into<String>) -> Self` | Hint text shown when empty. |
| `.show(self, ui: &mut Ui) -> Response` | Render; returns the field `Response` (`.changed()` on edit). |

## Usage

```rust
use ouroboros_ui::molecules::SearchField;

// minimal
SearchField::new(&mut query).show(ui);
```

```rust
use ouroboros_ui::molecules::SearchField;

// with placeholder; react to edits
if SearchField::new(&mut query)
    .placeholder("Search assets…")
    .show(ui)
    .changed()
{
    refilter(&query);
}
```

## Composition

Composes [`InputGroup`](input_group.md) only (which in turn composes [`Surface`](../atoms/surface.md) + [`Icon`](../atoms/icon.md) + the text editor). It never paints — see the [guards](../../guards.md).

## Notes

- A deliberately minimal molecule — no id_source setter; the underlying `InputGroup` falls back to egui's auto id. If you need a stable id or trailing clear button, use [`InputGroup`](input_group.md) directly.
- The returned `Response` is the field's; `.changed()` fires on edit.
