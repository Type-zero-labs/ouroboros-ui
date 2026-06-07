# Select

> **Layer:** organism · **Path:** `src/organisms/select.rs` · **Exports:** `select::Select`

A dropdown single-select (shadcn Select / Unity Dropdown / O3DE Dropdown): a trigger [`Button`](../atoms/button.md) showing the current option + a [`egui::Popup::menu`](https://docs.rs/egui) of [`MenuItem`](../cells/menu_item.md) cells. Bound to a `&mut usize` index — clicking an item writes the index and closes the popup. `show` returns the trigger `Response`.

## Design

- **Purpose / when to use** — pick one value from a fixed list (blend mode, quality level). For arbitrary anchored content use [`Popover`](popover.md); for command menus use [`DropdownMenu`](dropdown_menu.md).
- **Anatomy** — trigger: `Button` (`Outline` variant, right-side `CARET_DOWN` glyph, keyed `"select_trigger"`) showing the selected option or placeholder → `Popup::menu` on its response → one `MenuItem` per option (keyed `("select", i)`).
- **Variants / states**

  | State | How |
  |---|---|
  | no/invalid selection | trigger shows `placeholder` (default `"Select…"`) |
  | selected | trigger shows `options[*selected]` |
  | option clicked | writes `*selected = i`, calls `ui.close()` |
  | size | `Size::Sm` / `Md` (default) / `Lg` via `.size` / `.sm()` / `.lg()` |

- **Tokens / layout consumed** — trigger height follows the shared [`Size`](../../tokens.md) scale (hover animation lives in `Button`); themed menu frame via `Popup`.
- **Layering** — uses **`egui::Popup`** anchored to the trigger; themed menu frame is the casing.
- **Accessibility** — `Popup` dismiss on outside-click / Esc; selection closes.

## API

| Method | Effect |
|---|---|
| `Select::new(selected: &'a mut usize) -> Self` | Bind to a selection index. |
| `.options<S: Into<String>>(options: impl IntoIterator<Item = S>) -> Self` | Set the option labels. |
| `.placeholder(text: impl Into<String>) -> Self` | Text shown when the index is out of range (default `"Select…"`). |
| `.size(size: Size) -> Self` | Set trigger size. |
| `.sm() -> Self` / `.lg() -> Self` | Size shortcuts. |
| `.show(ui) -> Response` | Render trigger + popup; on click writes `*selected`. Returns the trigger `Response`. |

## Usage

```rust
use ouroboros_ui::organisms::Select;

let mut sel = 0usize;
Select::new(&mut sel)
    .options(["Opaque", "Cutout", "Transparent"])
    .placeholder("Blend mode…")
    .show(ui);
```

```rust
// realistic — persist selection across frames (from storybook)
use ouroboros_ui::organisms::Select;

let id = egui::Id::new("select_demo");
let mut sel = ui.data(|d| d.get_temp::<usize>(id).unwrap_or(0));
Select::new(&mut sel)
    .options(["Opaque", "Cutout", "Transparent", "Additive"])
    .placeholder("Blend mode…")
    .show(ui);
ui.data_mut(|d| d.insert_temp(id, sel));
```

## Composition

Overlay organism: a [`Button`](../atoms/button.md) atom trigger + the **`egui::Popup::menu`** container holding [`MenuItem`](../cells/menu_item.md) cells. It never paints — see [guards](../../guards.md).

## Notes

- **State ownership** — the consumer owns the `&mut usize`; persist it yourself (e.g. egui temp data) to survive frames, as the storybook does.
- An out-of-range index renders the placeholder rather than panicking.
- The trigger id is fixed (`"select_trigger"`); push a unique `ui.push_id(...)` scope when rendering several selects in one container (storybook does this for the size row).
