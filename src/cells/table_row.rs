//! TableRow cell — the row model for the [`Table`](crate::organisms::Table) organism.
//!
//! Holds a row's [`TableCell`]s plus row-level state (selected / selectable). It is a
//! *descriptor*, not a renderer: the Table organism lays the cells out across the column widths
//! (via `egui_extras`) and reads this state to drive selection. Never paints.

use crate::cells::TableCell;

/// A table row: its cells plus selection state.
pub struct TableRow {
    pub(crate) cells: Vec<TableCell>,
    pub(crate) selected: bool,
    pub(crate) selectable: bool,
}

impl TableRow {
    pub fn new(cells: impl IntoIterator<Item = TableCell>) -> Self {
        Self {
            cells: cells.into_iter().collect(),
            selected: false,
            selectable: true,
        }
    }

    /// Mark the row as selected (highlighted).
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    /// Whether the row may be selected (default `true`).
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }
}
