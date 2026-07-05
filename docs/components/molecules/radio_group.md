# RadioGroup

> **Layer:** molecule · **Path:** `src/molecules/radio_group.rs` · **Exports:** `radio_group::RadioGroup`

A single-select group of [`Radio`](../atoms/radio.md) atoms bound to a `&mut usize` index. Composes one labeled radio per option, vertically (default) or horizontally; clicking an option writes its index back into the binding.

## Design

- **Purpose / when to use** — Pick exactly one option from a short list with full per-option labels visible (vs. a compact [`ToggleGroup`](toggle_group.md)).
- **Anatomy** — A vertical or horizontal layout of [`Radio`](../atoms/radio.md) atoms (`Radio::new(selected == i).label(option)`), each spaced by `SPACE_1`.
- **States** — exactly one radio reflects `*selected == i`.
- **Tokens / layout consumed** — `core::SPACE_1` (inter-option gap). See [tokens](../../tokens.md).

## API

| Method | Effect |
|---|---|
| `RadioGroup::new(selected: &'a mut usize) -> Self` | Bind the selected index. |
| `.options<S: Into<String>>(options: impl IntoIterator<Item = S>) -> Self` | Set the option labels. |
| `.horizontal() -> Self` | Lay out in a row instead of a column. |
| `.show(self, ui: &mut Ui) -> Response` | Render; writes `*selected = i` on click. Returns the layout `Response`. |

## Usage

```rust
use ouroboros_ui::molecules::RadioGroup;

// minimal — vertical
let mut sel = 0usize;
RadioGroup::new(&mut sel)
    .options(["Small", "Medium", "Large"])
    .show(ui);
```

```rust
use ouroboros_ui::molecules::RadioGroup;

// horizontal
RadioGroup::new(&mut sel)
    .options(["Windowed", "Fullscreen"])
    .horizontal()
    .show(ui);
```

## Composition

Composes [`Radio`](../atoms/radio.md) atoms inside `ui.vertical` / `ui.horizontal`. It never paints — see the [guards](../../guards.md).

## Notes

- Two-way binding: `show` mutates `*selected` to the clicked index.
- Per-radio ids are derived via `.id_source(("radio_group", i))`, so a single group is collision-free; nest under [`FieldSet`](field.md#fieldset) for a labeled group.
- Returns the container `Response`, not a per-option signal — selection is communicated through the binding.
