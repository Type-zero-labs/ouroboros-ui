# Popover

> **Layer:** organism · **Path:** `src/organisms/popover.rs` · **Exports:** `popover::Popover`

Free content anchored to a trigger widget, opened on click (shadcn Popover). A thin wrapper over [`egui::Popup::menu`](https://docs.rs/egui) whose frame inherits the themed menu visuals. This is the substrate for color pickers, selects, combobox and menus — anything that needs click-anchored floating content.

## Design

- **Purpose / when to use** — anchored panels (pickers, mini-forms, hover cards) hung off a button or any widget `Response`. Reach for [`DropdownMenu`](dropdown_menu.md) / [`Select`](select.md) when the content is a list of items; reach for `Popover` when the content is arbitrary.
- **Anatomy** — `egui::Popup::menu(trigger)` casing → your `content` closure (any widgets).
- **Variants / states**

  | State | How |
  |---|---|
  | closed | trigger not clicked |
  | open | opens on trigger click; dismiss via outside-click / Esc |

- **Tokens / layout consumed** — themed menu frame via `Popup` (no direct token use).
- **Layering** — uses **`egui::Popup`** (menu style) anchored to the trigger `Response`; the themed menu frame is the casing.
- **Accessibility** — `Popup` handles outside-click / Esc dismiss.

## API

| Method | Effect |
|---|---|
| `Popover::new() -> Self` | Construct. (Unit struct; no fields.) |
| `Popover::default()` | Same as `new()`. |
| `.show(trigger: &Response, content: impl FnOnce(&mut Ui))` | Open from `trigger` on click; render `content`. Returns `()`. |

`show` takes a `&Response` (the trigger) plus a content closure; it returns nothing — wire any result through your own captured state.

## Usage

```rust
use ouroboros_ui::organisms::Popover;
use ouroboros_ui::atoms::{Button, Text};

let resp = Button::new("Open").id_source("pop").show(ui);
Popover::new().show(&resp, |ui| {
    Text::new("Anchored content").show(ui);
});
```

```rust
// realistic (from storybook)
use ouroboros_ui::organisms::Popover;
use ouroboros_ui::atoms::{Button, Text};
use ouroboros_ui::tokens::core;

let resp = Button::new("Open popover").id_source("pop_btn").show(ui);
Popover::new().show(&resp, |ui| {
    Text::new("Popover content").body_strong().show(ui);
    ui.add_space(core::SPACE_1);
    Text::new("Anchored to the trigger.").muted().caption().show(ui);
});
```

## Composition

Overlay organism: **`egui::Popup::menu`** container + themed menu frame for the casing; the body is whatever atoms/molecules you compose inside `content`. It never paints — see [guards](../../guards.md).

## Notes

- The trigger is a separate widget you draw and pass by reference; `Popover` does not draw the trigger.
- `show` returns `()`; to capture a result (e.g. a picked value), mutate a variable closed over by `content`.
- `content` is `FnOnce`, run only while the popover is open.
