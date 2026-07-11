# Autocomplete

> **Layer:** organism · **Path:** `src/organisms/autocomplete.rs` · **Exports:** `autocomplete::Autocomplete`

A search field whose filtered matches render as a clickable result list below it ("add component" style — shadcn Combobox / Command). Bind a query `String` and pass the full candidate label list; `show` filters by case-insensitive substring and returns the **original index** of the row clicked this frame.

## Design

- **Purpose / when to use** — pick from a large list by typing (add-component pickers, asset search). For a small fixed list use [`Select`](select.md); for command menus use [`DropdownMenu`](dropdown_menu.md).
- **Anatomy** — [`SearchField`](../molecules/search_field.md) (bound to `query`) → [`Surface`](../atoms/surface.md) with one [`MenuItem`](../cells/menu_item.md) per match (each with a "click to add" tooltip) → [`Text`](../atoms/text.md) muted quando não há match.
- **Variants / states**

  | State | How |
  |---|---|
  | empty query | full list (capped) |
  | filtered | case-insensitive substring sobre os labels |
  | no match | muted "no results" text |
  | row clicked | `show` returns `Some(original_index)` |
  | cap | `MAX_RESULTS = 50` rows rendered |

- **Tokens / layout consumed** — spacing via `tokens::core`; list chrome via `Surface`; MVP renders inline (sem popup flutuante).

## API

```rust
use ouroboros_ui::organisms::Autocomplete;

let mut query = String::new();
if let Some(idx) = Autocomplete::new(&mut query, ["Poring", "Drops", "Lunatic"])
    .placeholder("Search creature…")
    .show(ui)
{
    // idx aponta pro item original (não pro filtrado)
}
```

| Builder | Effect |
|---|---|
| `new(&mut query, items)` | bind da query + labels candidatos |
| `.placeholder(text)` | placeholder do SearchField |
| `.show(ui) -> Option<usize>` | render + índice original clicado neste frame |

## Storybook

`cargo run --example storybook` → page **Autocomplete**.
