//! TableRow cell — a row of [`TableCell`]s across fixed column widths.
//! [legacy ouroboros-ui table style]
//!
//! `header()` styles the cells as labels; `zebra()`/header give the row a muted background.

use crate::atoms::Surface;
use crate::cells::TableCell;
use crate::tokens::core;
use egui::{Response, Ui};

/// A table row. `show(ui, &widths)` lays each cell at its column width.
pub struct TableRow {
    cells: Vec<String>,
    header: bool,
    zebra: bool,
}

impl TableRow {
    pub fn new<S: Into<String>>(cells: impl IntoIterator<Item = S>) -> Self {
        Self {
            cells: cells.into_iter().map(Into::into).collect(),
            header: false,
            zebra: false,
        }
    }
    pub fn header(mut self) -> Self {
        self.header = true;
        self
    }
    /// Give this row a muted background (alternating data rows).
    pub fn zebra(mut self, zebra: bool) -> Self {
        self.zebra = zebra;
        self
    }

    pub fn show(self, ui: &mut Ui, widths: &[f32]) -> Response {
        let cells = self.cells;
        let header = self.header;
        let body = |ui: &mut Ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                for (i, text) in cells.iter().enumerate() {
                    let width = widths.get(i).copied().unwrap_or(core::SPACE_12 * 2.0);
                    let mut cell = TableCell::new(text.clone());
                    if header {
                        cell = cell.header();
                    }
                    cell.show(ui, width);
                }
            });
        };
        if header || self.zebra {
            Surface::new()
                .muted()
                .border_none()
                .radius(0.0)
                .pad(0.0)
                .show(ui, body)
                .response
        } else {
            ui.scope(body).response
        }
    }
}
