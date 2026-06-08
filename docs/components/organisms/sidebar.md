# Sidebar

> **Layer:** organism · **Path:** `src/organisms/sidebar.rs` · **Exports:** `sidebar::Sidebar`

A vertical navigation list (shadcn Sidebar / Navigation Menu) bound to a `&mut usize` selection. By default `show` composes one [`ListItem`](../cells/list_item.md) cell per entry; `.icons_only()` collapses it to an icon rail of icon-only [`Button`](../atoms/button.md)s. `show` returns the vertical `Response`; clicking an entry writes the selection in place.

## Design

- **Purpose / when to use** — primary nav for a view (Home / Assets / Settings), as the left band of a screen's root [`Splitter`](splitter.md) (fixed-px rail, or a resizable panel). Use the icon rail when horizontal space is tight.
- **Anatomy** — `ui.vertical` → per entry, either:
  - **list mode** — a `ListItem` (`.selected(active)`, keyed `("sidebar", i)`, optional leading glyph), or
  - **icons-only mode** — an icon-only `Button` (`Secondary` when active else `Ghost`, keyed `("sidebar_icon", i)`, optional `icon_left`).
- **Variants / states**

  | State | How |
  |---|---|
  | item with icon | `.item(icon, label)` |
  | text-only item | `.text_item(label)` |
  | selected | `*selected == i` → `ListItem.selected(true)` / `Button` `Secondary` |
  | icon rail | `.icons_only()` |

- **Tokens / layout consumed** — themed visuals through `ListItem` / `Button`; vertical layout spacing from egui defaults.
- **Accessibility** — selection is click-driven; active row/button is visually distinguished.

## API

| Method | Effect |
|---|---|
| `Sidebar::new(selected: &'a mut usize) -> Self` | Bind to a selection index. |
| `.item(icon: &'static str, label: impl Into<String>) -> Self` | Add an entry with a leading glyph. |
| `.text_item(label: impl Into<String>) -> Self` | Add an entry with no glyph. |
| `.icons_only() -> Self` | Collapse to an icon-only rail. |
| `.show(ui) -> Response` | Render the list; clicking writes `*selected`. Returns the vertical `Response`. |

## Usage

```rust
use ouroboros_ui::organisms::Sidebar;
use ouroboros_ui::egui_phosphor::light;

let mut sel = 0usize;
Sidebar::new(&mut sel)
    .item(light::HOUSE, "Home")
    .item(light::CUBE, "Assets")
    .item(light::GEAR, "Settings")
    .show(ui);
```

```rust
// realistic — list + icon rail sharing one selection (from storybook)
use ouroboros_ui::organisms::Sidebar;
use ouroboros_ui::egui_phosphor::light;

let mut sel = 0usize;
Sidebar::new(&mut sel)
    .item(light::HOUSE, "Home")
    .item(light::CUBE, "Assets")
    .show(ui);

Sidebar::new(&mut sel)              // shares `sel`
    .item(light::HOUSE, "Home")
    .item(light::CUBE, "Assets")
    .icons_only()
    .show(ui);
```

## Composition

Composes [`ListItem`](../cells/list_item.md) cells (list mode) or icon-only [`Button`](../atoms/button.md) atoms (rail mode). It never paints — see [guards](../../guards.md).

## Notes

- **State ownership** — the consumer owns the `&mut usize`; persist it across frames yourself.
- In icons-only mode an entry added via `text_item` (no glyph) renders a glyph-less icon button — supply icons for the rail.
- Two sidebars sharing the same `&mut usize` stay in sync (storybook pairs a list and a rail this way).
