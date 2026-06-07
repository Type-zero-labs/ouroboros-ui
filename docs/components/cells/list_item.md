# ListItem

> **Layer:** cell · **Path:** `src/cells/list_item.rs` · **Exports:** `list_item::ListItem`

A selectable list row built from an optional leading icon, a title, and an optional subtitle. Modelled on the shadcn `Item`. Selection is a stateless input (`selected: bool`) and a click yields a plain [`Response`] — the consumer owns the selected index and reacts to `.clicked()`.

## Design

- **Purpose / when to use** — Vertically stacked, single-line-or-two rows in a list/panel where one item can be highlighted (file lists, asset pickers, project entries).
- **Anatomy** — A [`Surface`](../atoms/surface.md) (interactive, padded) wrapping a horizontal layout: optional [`Icon`](../atoms/icon.md) (muted) + a vertical stack of [`Text`](../atoms/text.md) (title) and an optional muted caption [`Text`](../atoms/text.md) (subtitle).
- **Variants / states**

  | State | Effect |
  |-------|--------|
  | default | `Surface::fill_none().border_none()` (transparent) |
  | selected (`selected(true)`) | `Surface::muted()` fill |
  | hover/click | `Surface::interactive()` provides hover feedback + sense; the returned `Response` carries `.clicked()` |

- **Tokens / layout consumed** — `core::SPACE_2` (8px outer pad + icon→text gap), `core::RADIUS_SM` (4px). See [tokens](../../tokens.md).
- **Accessibility** — Inherits the `Surface`'s interactive sense; selection state is purely visual (`muted` fill), so pair with a real selection model in the consumer.

## API

| Method | Signature | Effect |
|--------|-----------|--------|
| `new` | `new(title: impl Into<String>) -> Self` | Construct with a title; icon/subtitle/selected default off. |
| `icon` | `icon(self, glyph: &'static str) -> Self` | Leading muted icon (a phosphor glyph). |
| `subtitle` | `subtitle(self, subtitle: impl Into<String>) -> Self` | Second line, rendered caption + muted. |
| `selected` | `selected(self, selected: bool) -> Self` | Toggle the `muted` selected fill. |
| `id_source` | `id_source(self, id: impl std::hash::Hash) -> Self` | Stable id for the underlying `Surface` (needed when rows share otherwise-equal layout). |
| `show` | `show(self, ui: &mut Ui) -> Response` | Render; returns the `Surface` response (`.clicked()`, `.hovered()`). |

## Usage

```rust
use ouroboros_ui::cells::ListItem;
use ouroboros_ui::egui_phosphor::light;

ListItem::new("Cube").icon(light::CUBE).subtitle("Mesh").show(ui);
```

```rust
// realistic — a selectable list owning the selected index
let mut sel: usize = /* persisted state */ 0;
for (i, (icon, title, sub)) in [
    (light::CUBE, "Cube", "Mesh"),
    (light::STAR, "Light", "Point"),
    (light::GEAR, "Settings", "Project"),
].iter().enumerate() {
    if ListItem::new(*title)
        .icon(icon)
        .subtitle(*sub)
        .selected(sel == i)
        .id_source(("li", i))
        .show(ui)
        .clicked()
    {
        sel = i;
    }
}
```

## Composition

Composes the [`Surface`](../atoms/surface.md), [`Icon`](../atoms/icon.md), and [`Text`](../atoms/text.md) atoms only. It performs no painting — all visuals come from `Surface` (fill/border/interaction) and the atoms. Enforced by [`tests/no_painter_in_molecules.rs`](../../guards.md).

## Notes

- Selection is **input, not state**: `ListItem` never remembers it. Drive it from a `usize`/`HashSet` in the parent and feed `.selected(...)`.
- Give each row a distinct `id_source` (e.g. `("li", i)`) so the interactive surfaces don't collide.
