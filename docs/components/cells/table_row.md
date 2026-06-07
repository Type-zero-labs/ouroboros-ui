# TableRow

> **Layer:** cell · **Path:** `src/cells/table_row.rs` · **Exports:** `table_row::TableRow`

The row model for the [`Table`](../organisms/table.md) organism: a `Vec` of [`TableCell`](table_cell.md)s plus row-level state (selected / selectable / key). It is a **descriptor, not a renderer** — `TableRow` has no `show`. The Table organism lays the cells out across the column widths (via `egui_extras`) and reads this state to drive selection. Carries lifetime `'a` from its cells (a `custom` cell may borrow).

## Design

- **Purpose / when to use** — Always paired with [`Table`](../organisms/table.md): construct one `TableRow` per data row and pass the collection to `Table::rows(...)`.
- **Anatomy** — Fields (crate-visible, consumed by the organism): `cells: Vec<TableCell<'a>>`, `selected: bool`, `selectable: bool` (default `true`), `key: Option<u64>`.
- **Variants / states**

  | Modifier | Effect |
  |----------|--------|
  | `selected(true)` | row highlighted by the Table organism |
  | `selectable(false)` | row cannot be selected (default is selectable) |
  | `key(u64)` | stable identity for selection / tree operations |

- **Tokens / layout consumed** — None directly; the organism applies row height (`layout::TABLE_ROW_HEIGHT`, 28px, or a size variant) and selection styling. See [layout tokens](../../layout.md).

## API

| Method | Signature | Effect |
|--------|-----------|--------|
| `new` | `new(cells: impl IntoIterator<Item = TableCell<'a>>) -> Self` | Build a row from its cells; `selected=false`, `selectable=true`, `key=None`. |
| `selected` | `selected(self, selected: bool) -> Self` | Mark the row selected (highlighted by the organism). |
| `selectable` | `selectable(self, selectable: bool) -> Self` | Whether the row may be selected (default `true`). |
| `key` | `key(self, key: u64) -> Self` | Stable identity for selection / tree operations. |

There is **no `show`** — `TableRow` is consumed by [`Table::rows(...)`](../organisms/table.md), not rendered standalone.

## Usage

```rust
use ouroboros_ui::cells::{TableRow, TableCell};

TableRow::new([
    TableCell::text("width"),
    TableCell::text("1920").end(),
]);
```

```rust
// realistic — feeding the Table organism with selection
use ouroboros_ui::cells::{TableRow, TableCell};
use ouroboros_ui::organisms::{Table, Column};

let rows = data.iter().enumerate().map(|(i, d)| {
    TableRow::new([
        TableCell::text(&d.name),
        TableCell::text(&d.size).end(),
        TableCell::text(&d.status).status(d.color),
    ])
    .key(i as u64)
    .selected(selected == Some(i as u64))
});

Table::new()
    .columns([Column::auto(), Column::auto().end(), Column::remainder()])
    .rows(rows)
    .show(ui);
```

## Composition

Composes [`TableCell`](table_cell.md)s only and holds plain state. It paints nothing and renders nothing on its own — rendering is the [`Table`](../organisms/table.md) organism's job. Consistent with the cells rule enforced by [`tests/no_painter_in_molecules.rs`](../../guards.md).

## Notes

- Pure data: the cells are stored, not drawn, until the organism iterates them across columns.
- Provide a stable `key` when the table supports selection or tree state so identity survives reordering.
- The `'a` lifetime flows from `TableCell<'a>` (a `custom` cell may borrow); keep borrowed data alive until the table is shown.
