# DropdownMenu

> **Layer:** organism · **Path:** `src/organisms/dropdown_menu.rs` · **Exports:** `dropdown_menu::DropdownMenu`

A popover list of [`MenuItem`](../cells/menu_item.md) cells opened from a trigger widget (shadcn DropdownMenu / ContextMenu). Built on [`egui::Popup::menu`](https://docs.rs/egui) anchored to a trigger `Response`. `show` returns the index of the clicked item, if any, and auto-closes on selection.

## Design

- **Purpose / when to use** — action menus hung off a button or context menus (Copy / Paste / Delete). Use when each entry is a one-shot command, not a persistent selection.
- **Anatomy** — `egui::Popup::menu(trigger)` casing → one [`MenuItem`](../cells/menu_item.md) per entry, each keyed `("dropdown", i)`, optional left glyph.
- **Variants / states**

  | State | How |
  |---|---|
  | item with icon | `.item(icon, label)` |
  | text-only item | `.text_item(label)` |
  | item clicked | returns `Some(index)`, calls `ui.close()` |
  | nothing clicked / closed | returns `None` |

- **Tokens / layout consumed** — themed menu visuals via `Popup` + `MenuItem` (no direct token use here).
- **Layering** — uses **`egui::Popup`** (menu style), anchored to the trigger response. Casing is the themed menu frame.
- **Accessibility** — `Popup` handles outside-click / Esc dismiss; selection closes it.

## API

| Method | Effect |
|---|---|
| `DropdownMenu::new() -> Self` | Empty menu. |
| `DropdownMenu::default()` | Same as `new()`. |
| `.item(icon: &'static str, label: impl Into<String>) -> Self` | Add an item with a leading glyph. |
| `.text_item(label: impl Into<String>) -> Self` | Add an item with no glyph. |
| `.show(trigger: &Response) -> Option<usize>` | Open from `trigger`; returns the clicked item index. |

`show` takes a `&Response` (the trigger widget), not a `&mut Ui`.

## Usage

```rust
use ouroboros_ui::organisms::DropdownMenu;
use ouroboros_ui::atoms::Button;
use ouroboros_ui::egui_phosphor::light;

let trigger = Button::new("Actions").icon_right(light::CARET_DOWN).id_source("dd").show(ui);
if let Some(i) = DropdownMenu::new()
    .item(light::COPY, "Copy")
    .item(light::CLIPBOARD, "Paste")
    .show(&trigger)
{
    // act on index `i`
}
```

```rust
// realistic — persist last choice (from storybook)
use ouroboros_ui::organisms::DropdownMenu;
use ouroboros_ui::atoms::Button;
use ouroboros_ui::egui_phosphor::light;

let resp = Button::new("Actions").icon_right(light::CARET_DOWN).id_source("dd_btn").show(ui);
if let Some(i) = DropdownMenu::new()
    .item(light::COPY, "Copy")
    .item(light::CLIPBOARD, "Paste")
    .item(light::TRASH, "Delete")
    .show(&resp)
{
    ui.data_mut(|d| d.insert_temp(egui::Id::new("dd_last"), i));
}
```

## Composition

Overlay organism: **`egui::Popup::menu`** container + themed menu frame for the casing; entries are [`MenuItem`](../cells/menu_item.md) cells. It never paints — see [guards](../../guards.md).

## Notes

- Item indices follow insertion order across `item` / `text_item`.
- Closes itself via `ui.close()` on click; the consumer only handles the returned index.
- The trigger is a separate widget you draw and pass in; the menu does not draw the trigger.
- Glyphs come from `ouroboros_ui::egui_phosphor::light::NAME`.
