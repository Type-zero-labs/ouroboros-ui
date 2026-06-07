# Table

> **Layer:** organism · **Path:** `src/organisms/table.rs` · **Exports:** `table::{ColWidth, Column, Table}`

A column-defined data table on [`egui_extras::TableBuilder`](https://docs.rs/egui_extras), Element-Plus-flavored. [`Column`](#column)s describe layout (width / min / header align); rows carry [`TableCell`](../cells/table_cell.md)s wrapped in [`TableRow`](../cells/table_row.md). `egui_extras` provides sizing, a sticky header, scrolling and striping; zebra / selection / hover colors come from the theme via `table_visuals` (set on the `ui`, never painted). Cells render through `TableCell`; the organism composes — it does not paint.

## Design

- **Purpose / when to use** — tabular data with aligned columns, sticky header, optional scrolling and row selection (asset lists, key/value property grids).
- **Anatomy** — optional `border` wraps everything in a card [`Surface`](../atoms/surface.md) (`pad 0`, `RADIUS_MD`). Inside: a `ui.scope` with `table_visuals` applied → `TableBuilder` with one `egui_extras::Column` per `Column` (all `.clip(true)`) → header row (`TableCell::text(label).header().align(col.align)`) → body rows (one `TableCell` per cell). Loading and empty states short-circuit to a centered `Spinner` / muted `Text`.
- **Variants / states**

  | State | How |
  |---|---|
  | striped | `.striped(true)` — zebra rows |
  | bordered | `.border(true)` — outer card surface |
  | sizes | `Size::Sm` / `Md` (default) / `Lg` → row height |
  | fixed height | `.height(px)` — header sticks, body scrolls |
  | fluid + cap | `.max_height(px)` — grows then scrolls |
  | selectable | `.selectable(true)` — click a row to select (persisted) |
  | loading | `.loading(true)` — centered [`Spinner`](../atoms/spinner.md) |
  | empty | no rows → muted `empty_text` (default `"No data"`) |

- **Tokens / layout consumed** — row height from the [`Size`](../../tokens.md) scale (`size.height()`); `core::SPACE_6` (loading/empty padding), `core::SPACE_0` + `core::RADIUS_MD` (border surface). Theme colors via [`table_visuals`](#table_visuals).
- **Accessibility** — selection via `Sense::click()` on rows when `selectable`; header is sticky on scroll.

## API

### `Table<'a>`

| Method | Effect |
|---|---|
| `Table::new() -> Self` | Empty table; defaults `empty_text = "No data"`, others off. |
| `Table::default()` | Same as `new()`. |
| `.columns(impl IntoIterator<Item = Column>) -> Self` | Set the columns. |
| `.rows(impl IntoIterator<Item = TableRow<'a>>) -> Self` | Set the rows. |
| `.row(TableRow<'a>) -> Self` | Append one row. |
| `.size(Size) -> Self` / `.sm()` / `.lg()` | Row height. |
| `.striped(bool) -> Self` | Zebra rows. |
| `.border(bool) -> Self` | Outer card border. |
| `.height(px) -> Self` | Fixed height; sticky header, body scrolls. |
| `.max_height(px) -> Self` | Fluid height capped at `px`. |
| `.selectable(bool) -> Self` | Click-to-select rows (persisted for the session). |
| `.loading(bool) -> Self` | Replace the grid with a centered spinner. |
| `.empty_text(impl Into<String>) -> Self` | Placeholder when there are no rows. |
| `.id_source(id: impl Hash) -> Self` | Stable id (drives selection + `TableBuilder` id salt). |
| `.show(ui) -> Response` | Render; returns the area `Response`. |

### Column

A header label + layout descriptor.

| Method | Effect |
|---|---|
| `Column::new(label: impl Into<String>) -> Self` | New column; default width `Remainder`, align `Start`. |
| `.width(ColWidth) -> Self` | Set sizing mode. |
| `.exact(px)` / `.initial(px)` / `.auto()` / `.remainder()` | Width shortcuts (`ColWidth` variants). |
| `.min_width(px) -> Self` | Floor the column width (`at_least`). |
| `.align(CellAlign) -> Self` / `.center()` / `.end()` | Header alignment (cells carry their own alignment). |

### ColWidth

`#[derive(Clone, Copy, Debug, Default, PartialEq)]` — how a column is sized.

| Variant | Maps to | Meaning |
|---|---|---|
| `Auto` | `ExtraColumn::auto()` | Size to content. |
| `Exact(f32)` | `ExtraColumn::exact(w)` | Fixed px width. |
| `Initial(f32)` | `ExtraColumn::initial(w)` | Initial px, resizable/sharable. |
| `Remainder` *(default)* | `ExtraColumn::remainder()` | Share leftover width. |

## Usage

```rust
use ouroboros_ui::organisms::{Table, Column};
use ouroboros_ui::cells::{TableRow, TableCell};

Table::new()
    .columns([Column::new("Key"), Column::new("Value").end()])
    .rows([TableRow::new([TableCell::text("width"), TableCell::text("1920").end()])])
    .border(true)
    .show(ui);
```

```rust
// realistic — striped, selectable, sticky header, status cells (from storybook)
use ouroboros_ui::organisms::{Table, Column};
use ouroboros_ui::cells::{TableRow, TableCell};

Table::new()
    .id_source("tbl_main")
    .columns([
        Column::new("Name"),
        Column::new("Type").exact(110.0),
        Column::new("Size").exact(90.0).end(),
        Column::new("Status").exact(110.0),
    ])
    .rows(data.iter().map(|(n, t, s, c)| TableRow::new([
        TableCell::text(*n),
        TableCell::text(*t).muted(),
        TableCell::text(*s).end(),
        TableCell::text("ref").status(*c),
    ])))
    .striped(true)
    .border(true)
    .selectable(true)
    .max_height(150.0)
    .show(ui);
```

## Composition

Composes [`TableCell`](../cells/table_cell.md) cells (header + body) wrapped in [`TableRow`](../cells/table_row.md), over `egui_extras::TableBuilder` for layout/scroll/striping. The optional border is a [`Surface`](../atoms/surface.md) atom; loading/empty use the [`Spinner`](../atoms/spinner.md) atom and [`Text`](../atoms/text.md) atom. Theme colors are pushed onto `ui.visuals_mut()` via `table_visuals` so `egui_extras`' built-in striping/selection/hover read DS tokens — no painting. See [guards](../../guards.md) and [theming](../../theming.md).

### `table_visuals`

Private helper that maps theme tokens onto `ui` (no painting): `faint_bg_color = theme.muted` (zebra), `selection.bg_fill = theme.accent`, `selection.stroke.color = theme.accent_foreground`, hovered/active `weak_bg_fill = theme.muted`.

## Notes

- **State ownership** — when `selectable`, the current row index persists in egui temp data keyed by the table id (`id_source`, or `ui.id().with("table")` if unset). Distinct tables need distinct `id_source` to avoid selection-state collisions; the same id also salts `TableBuilder`.
- Per-row opt-out: a row is only clickable/selectable if `TableRow.selectable` is true **and** `Table.selectable` is on.
- Column alignment is header-only; each `TableCell` carries its own cell alignment.
- All columns are clipped (`.clip(true)`); `vscroll` engages only when `height` or `max_height` is set.
- `loading` and empty (`rows.is_empty()`) short-circuit before any grid is built.
