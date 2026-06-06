//! Table organism — a header + data rows. [shadcn Table / Unity Multi-column]
//!
//! Built on [`egui::Grid`] (reliable column layout + zebra striping) with [`Text`] atom cells.
//! Cells can be plain strings or, for the engine's rich tables, carry a status dot.

use crate::atoms::Text;
use egui::{Grid, Id, Response, Ui};

/// A data table. Rows are strings; the header row is styled as muted labels.
pub struct Table {
    id: Id,
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    striped: bool,
}

impl Table {
    pub fn new() -> Self {
        Self {
            id: Id::new("table"),
            headers: Vec::new(),
            rows: Vec::new(),
            striped: true,
        }
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id = Id::new(id);
        self
    }
    pub fn headers<S: Into<String>>(mut self, headers: impl IntoIterator<Item = S>) -> Self {
        self.headers = headers.into_iter().map(Into::into).collect();
        self
    }
    pub fn row<S: Into<String>>(mut self, row: impl IntoIterator<Item = S>) -> Self {
        self.rows.push(row.into_iter().map(Into::into).collect());
        self
    }
    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let headers = self.headers;
        let rows = self.rows;
        Grid::new(self.id)
            .striped(self.striped)
            .num_columns(
                headers
                    .len()
                    .max(rows.first().map_or(0, |r| r.len()))
                    .max(1),
            )
            .show(ui, |ui| {
                if !headers.is_empty() {
                    for header in &headers {
                        Text::new(header).label().muted().show(ui);
                    }
                    ui.end_row();
                }
                for row in &rows {
                    for cell in row {
                        Text::new(cell).show(ui);
                    }
                    ui.end_row();
                }
            })
            .response
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}
