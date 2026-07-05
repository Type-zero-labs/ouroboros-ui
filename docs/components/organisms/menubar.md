# Menubar

> **Layer:** organism · **Path:** `src/organisms/menubar.rs` · **Exports:** `menubar::Menubar`

An application menu bar — a horizontal row of menu triggers, each a dropdown (shadcn Menubar). Composes [`Button`](../atoms/button.md) (ghost, sm) triggers with [`egui::Popup::menu`](https://docs.rs/egui) dropdowns of [`MenuItem`](../cells/menu_item.md) cells. `show` returns `(menu_index, item_index)` when an item is chosen.

## Design

- **Purpose / when to use** — top-of-window File / Edit / View bars. Use for command menus organized by top-level category.
- **Anatomy** — `ui.horizontal` row → per menu: a ghost-`sm` `Button` (keyed `("menubar", mi)`) → `Popup::menu` on its response → one `MenuItem` per entry (keyed `("menubar_item", mi, ii)`).
- **Variants / states**

  | State | How |
  |---|---|
  | menu trigger | ghost `sm` button |
  | item chosen | returns `Some((menu_idx, item_idx))`, calls `ui.close()` |
  | nothing chosen | returns `None` |

- **Tokens / layout consumed** — themed visuals via `Button`/`Popup`/`MenuItem`; horizontal layout spacing from egui defaults.
- **Layering** — each menu uses **`egui::Popup`** anchored to its trigger button; themed menu frame is the casing.
- **Accessibility** — `Popup` dismiss on outside-click / Esc; selection closes.

## API

| Method | Effect |
|---|---|
| `Menubar::new() -> Self` | Empty bar. |
| `Menubar::default()` | Same as `new()`. |
| `.menu<S: Into<String>>(label: impl Into<String>, items: impl IntoIterator<Item = S>) -> Self` | Append a menu with its item labels. |
| `.show(ui) -> Option<(usize, usize)>` | Render the bar; returns `(menu_idx, item_idx)` of the chosen item. |

## Usage

```rust
use ouroboros_ui::organisms::Menubar;

if let Some((m, i)) = Menubar::new()
    .menu("File", ["New", "Open", "Save"])
    .menu("Edit", ["Undo", "Redo"])
    .show(ui)
{
    // dispatch on (m, i)
}
```

```rust
// realistic — full bar, persist last choice (from storybook)
use ouroboros_ui::organisms::Menubar;

if let Some((m, i)) = Menubar::new()
    .menu("File", ["New", "Open", "Save", "Quit"])
    .menu("Edit", ["Undo", "Redo", "Preferences"])
    .menu("View", ["Zoom in", "Zoom out", "Reset"])
    .show(ui)
{
    ui.data_mut(|d| d.insert_temp(egui::Id::new("mb_last"), (m, i)));
}
```

## Composition

Composes [`Button`](../atoms/button.md) atoms (triggers), the **`egui::Popup`** container (dropdowns), and [`MenuItem`](../cells/menu_item.md) cells (entries). It never paints — see [guards](../../guards.md).

## Notes

- Menu and item indices follow insertion order; the return distinguishes them as `(menu, item)`.
- Triggers are keyed by menu index and items by `(menu, item)`, so duplicate labels are safe.
- Selection auto-closes via `ui.close()`; the consumer handles only the returned tuple.
