# TableCell

> **Layer:** cell · **Path:** `src/cells/table_cell.rs` · **Exports:** `table_cell::{CellAlign, TableCell}`

One cell of a table: a container that places its text content inside the column it is handed. A cell and a header are the same container — they differ chiefly in text weight (header = `label_strong`). Padding and alignment are token-driven; optionally a leading status dot ([`ColorSwatch`](../atoms/color_swatch.md)) precedes the content.

## Design

- **Purpose / when to use** — Building block for the [`Table`](../organisms/table.md) organism (and ad-hoc fixed-width row layouts).
- **Anatomy** — A `ui.with_layout(...)` block: leading `core::SPACE_2` pad, optional circular [`ColorSwatch`](../atoms/color_swatch.md) status dot + `core::SPACE_1` gap, then the content — a [`Text`](../atoms/text.md) atom (weight/muted per flags).
- **Variants / states**

  | Modifier | Effect |
  |----------|--------|
  | `text(s)` | text content |
  | `header()` | `Text::label_strong()` (stronger weight) |
  | `muted()` | `Text::muted()` foreground |
  | `status(color)` | leading circular color dot, `core::SPACE_2` diameter |
  | align: `align(CellAlign)` / `center()` / `end()` | content alignment within the cell |

- **Tokens / layout consumed** — `core::SPACE_2` (leading pad + status dot size), `core::SPACE_1` (dot→content gap). See [tokens](../../tokens.md).

### `CellAlign`

`#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]` — chooses the cell's egui `Layout`:

| Variant | Layout |
|---------|--------|
| `Start` (default) | `Layout::left_to_right(Align::Center)` |
| `Center` | `Layout::centered_and_justified(Direction::LeftToRight)` |
| `End` | `Layout::right_to_left(Align::Center)` |

## API

| Method | Signature | Effect |
|--------|-----------|--------|
| `text` | `text(text: impl Into<String>) -> Self` | Text cell. |
| `header` | `header(self) -> Self` | Render as a header (strong text weight). |
| `align` | `align(self, align: CellAlign) -> Self` | Set horizontal alignment. |
| `center` | `center(self) -> Self` | Shorthand for `align(CellAlign::Center)`. |
| `end` | `end(self) -> Self` | Shorthand for `align(CellAlign::End)`. |
| `status` | `status(self, color: Color32) -> Self` | Leading status dot in `color`. |
| `muted` | `muted(self) -> Self` | Muted text foreground. |
| `show` | `show(self, ui: &mut Ui) -> Response` | Fill the column cell it is given; returns the layout block `Response`. |

## Usage

```rust
use ouroboros_ui::cells::{TableCell, CellAlign};

TableCell::text("Name").header().show(ui);          // header cell
TableCell::text("2.1 MB").end().show(ui);           // right-aligned value
TableCell::text("ref").status(theme.success).show(ui); // leading status dot
```

```rust
// realistic — building rows for the Table organism (see TableRow)
use ouroboros_ui::cells::{TableCell, TableRow};

TableRow::new([
    TableCell::text(&d.name),
    TableCell::text(&d.size).end(),
    TableCell::text(&d.status).status(color),
]);
```

## Composition

Composes the [`Text`](../atoms/text.md) atom and (optionally) the [`ColorSwatch`](../atoms/color_swatch.md) atom. The cell never paints — alignment is an egui `Layout`, visuals come from the atoms. Enforced by [`tests/no_painter_in_molecules.rs`](../../guards.md).

## Notes

- The cell fills the width it is handed; the surrounding column width is set by the [`Table`](../organisms/table.md) organism (via `egui_extras`) — the cell itself does not size the column.
- For inline-editable cells (arbitrary widgets in a grid), use [`Table::layout`](../organisms/table.md) and draw atoms into the returned rects.
