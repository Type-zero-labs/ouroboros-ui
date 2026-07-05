# CheckboxCard

> **Layer:** molecule · **Path:** `src/molecules/checkbox_card.rs` · **Exports:** `checkbox_card::CheckboxCard`

A selectable card bound to a `&mut bool`: an interactive [`Surface`](../atoms/surface.md) wrapping a **display-only** [`Checkbox`](../atoms/checkbox.md) plus a label and optional description. The whole card is the click target — clicking anywhere toggles the bound bool (the inner checkbox is non-interactive, so there is no double-toggle).

## Design

- **Purpose / when to use** — A larger, more legible alternative to a bare checkbox for opt-in settings, where a description helps.
- **Anatomy** — `Surface::new().interactive().selected(checked)` padded `SPACE_3` → horizontal row of a display [`Checkbox`](../atoms/checkbox.md) (`.interactive(false)`) + a vertical stack of a `body_strong` [`Text`](../atoms/text.md) label and an optional muted caption description.
- **States** — selected visual driven by `Surface::selected(*checked)`; hover/press handled by `Surface::interactive()`.
- **Tokens / layout consumed** — `core::SPACE_3` (pad + checkbox→text gap). See [tokens](../../tokens.md).

## API

| Method | Effect |
|---|---|
| `CheckboxCard::new(checked: &'a mut bool, label: impl Into<String>) -> Self` | Bind state + set label. |
| `.description(description: impl Into<String>) -> Self` | Optional muted caption under the label. |
| `.id_source(id: impl std::hash::Hash) -> Self` | Stable surface id (use when several share a frame). |
| `.show(self, ui: &mut Ui) -> Response` | Render; toggles `*checked` on click. Returns the surface `Response`. |

## Usage

```rust
use ouroboros_ui::molecules::CheckboxCard;

// minimal
let mut on = true;
CheckboxCard::new(&mut on, "Enable notifications").show(ui);
```

```rust
use ouroboros_ui::molecules::CheckboxCard;

// realistic — with description + stable id
CheckboxCard::new(&mut on, "Enable notifications")
    .description("Email + in-app alerts")
    .id_source("cc")
    .show(ui);
```

## Composition

Composes [`Surface`](../atoms/surface.md) + [`Checkbox`](../atoms/checkbox.md) + [`Text`](../atoms/text.md). It never paints — see the [guards](../../guards.md).

## Notes

- Two-way binding: `show` writes `*self.checked = !*self.checked` when the surface is clicked.
- The inner checkbox is mirrored display state (`.interactive(false)`), so it doesn't compete for the click.
- Set `id_source` to avoid surface-id collisions when stacking multiple cards.
