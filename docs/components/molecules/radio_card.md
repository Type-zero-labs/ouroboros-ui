# RadioCard

> **Layer:** molecule · **Path:** `src/molecules/radio_card.rs` · **Exports:** `radio_card::RadioCard`

A selectable card for single-choice options: an interactive [`Surface`](../atoms/surface.md) wrapping a **display-only** [`Radio`](../atoms/radio.md) plus a label and optional description. Stateless like the `Radio` atom — it reports clicks via its `Response`; the consumer owns exclusivity across the set.

## Design

- **Purpose / when to use** — Present a small set of mutually exclusive options as legible cards (plan tiers, modes) instead of bare radio buttons.
- **Anatomy** — `Surface::new().interactive().selected(selected)` padded `SPACE_3` → horizontal row of a display [`Radio`](../atoms/radio.md) (`.interactive(false)`) + a vertical stack of a `body_strong` [`Text`](../atoms/text.md) label and an optional muted caption description.
- **States** — selected visual driven by `Surface::selected(selected)`; hover/press by `Surface::interactive()`. Selection itself is caller-managed.
- **Tokens / layout consumed** — `core::SPACE_3` (pad + radio→text gap). See [tokens](../../tokens.md).

## API

| Method | Effect |
|---|---|
| `RadioCard::new(selected: bool, label: impl Into<String>) -> Self` | Set the selected visual + label. Note `selected` is a plain `bool`, not a binding. |
| `.description(description: impl Into<String>) -> Self` | Optional muted caption under the label. |
| `.id_source(id: impl std::hash::Hash) -> Self` | Stable surface id (use one per card). |
| `.show(self, ui: &mut Ui) -> Response` | Render; returns the surface `Response` (check `.clicked()`). |

## Usage

```rust
use ouroboros_ui::molecules::RadioCard;

// realistic — consumer manages exclusivity over a list
let mut sel = 0usize;
for (i, (title, desc)) in [
    ("Starter", "Up to 10 projects"),
    ("Pro", "Unlimited projects"),
].iter().enumerate() {
    if RadioCard::new(sel == i, *title)
        .description(*desc)
        .id_source(("rc", i))
        .show(ui)
        .clicked()
    {
        sel = i;
    }
}
```

## Composition

Composes [`Surface`](../atoms/surface.md) + [`Radio`](../atoms/radio.md) + [`Text`](../atoms/text.md). It never paints — see the [guards](../../guards.md).

## Notes

- Unlike [`CheckboxCard`](checkbox_card.md), this takes `selected: bool` (not `&mut`) and does **not** mutate anything — react to `.clicked()` and update your own selection index.
- The inner radio is display-only (`.interactive(false)`); the whole card is the click target.
- Give each card a distinct `id_source` (e.g. `("rc", i)`) to avoid surface-id collisions.
