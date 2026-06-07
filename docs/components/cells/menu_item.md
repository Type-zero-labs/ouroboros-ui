# MenuItem

> **Layer:** cell · **Path:** `src/cells/menu_item.rs` · **Exports:** `menu_item::MenuItem`

A single menu row: optional leading icon, a label, and an optional right-aligned keyboard shortcut rendered as a [`Kbd`](../atoms/kbd.md). Modelled on the shadcn `DropdownMenu` item. Clicking yields a [`Response`]; a disabled item drops its interactive sense.

## Design

- **Purpose / when to use** — Rows inside dropdowns, context menus, and command lists where each entry pairs an action label with an optional accelerator.
- **Anatomy** — A [`Surface`](../atoms/surface.md) (transparent, padded) wrapping a horizontal layout: optional muted [`Icon`](../atoms/icon.md) + a [`Text`](../atoms/text.md) label + (right-to-left) an optional [`Kbd`](../atoms/kbd.md) shortcut.
- **Variants / states**

  | State | Effect |
  |-------|--------|
  | default (`enabled(true)`) | `Surface::interactive()` — hover feedback + click sense |
  | disabled (`enabled(false)`) | `Surface` is non-interactive (no hover/sense) |
  | with shortcut | trailing `Kbd` pinned right via `Layout::right_to_left(Align::Center)` |

- **Tokens / layout consumed** — `core::SPACE_1` (4px outer pad), `core::SPACE_2` (icon→label gap), `core::RADIUS_SM` (4px). See [tokens](../../tokens.md).
- **Accessibility** — Disabled rows simply lose interactivity; they are not greyed by the cell itself.

## API

| Method | Signature | Effect |
|--------|-----------|--------|
| `new` | `new(label: impl Into<String>) -> Self` | Construct with a label; `enabled` defaults `true`. |
| `icon` | `icon(self, glyph: &'static str) -> Self` | Leading muted icon (phosphor glyph). |
| `shortcut` | `shortcut(self, shortcut: impl Into<String>) -> Self` | Right-aligned `Kbd` shortcut text. |
| `enabled` | `enabled(self, enabled: bool) -> Self` | When `false`, the surface is not interactive. |
| `id_source` | `id_source(self, id: impl std::hash::Hash) -> Self` | Stable id for the underlying `Surface`. |
| `show` | `show(self, ui: &mut Ui) -> Response` | Render; returns the `Surface` response (`.clicked()`). |

## Usage

```rust
use ouroboros_ui::cells::MenuItem;
use ouroboros_ui::egui_phosphor::light;

MenuItem::new("Copy").icon(light::COPY).shortcut("Ctrl C").id_source("mi_c").show(ui);
```

```rust
// realistic — a small context menu column
MenuItem::new("Copy").icon(light::COPY).shortcut("Ctrl C").id_source("mi_c").show(ui);
MenuItem::new("Paste").icon(light::CLIPBOARD).shortcut("Ctrl V").id_source("mi_v").show(ui);
MenuItem::new("Delete").icon(light::TRASH).id_source("mi_d").show(ui);
MenuItem::new("Disabled").enabled(false).id_source("mi_x").show(ui);
```

## Composition

Composes the [`Surface`](../atoms/surface.md), [`Icon`](../atoms/icon.md), [`Text`](../atoms/text.md), and [`Kbd`](../atoms/kbd.md) atoms only. No painting — visuals come entirely from `Surface` + atoms. Enforced by [`tests/no_painter_in_molecules.rs`](../../guards.md).

## Notes

- The shortcut is plain text (e.g. `"Ctrl C"`), rendered by `Kbd`; it does not bind a real accelerator — wire the key handling separately.
- Give each row a distinct `id_source` to avoid surface id collisions.
