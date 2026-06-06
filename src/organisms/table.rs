//! Table organism — framed data table. [legacy ouroboros-ui table style]
//!
//! A card [`Surface`] holding a muted header row (bottom-bordered) over zebra-striped data
//! rows, built from [`TableRow`]/[`TableCell`] cells. Columns are equal-width.

use crate::atoms::{Divider, Surface};
use crate::cells::TableRow;
use crate::tokens::core;
use egui::{Response, Ui};

/// A data table. `headers` + `row(..)` build it; the columns split the width equally.
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            rows: Vec::new(),
        }
    }
    pub fn headers<S: Into<String>>(mut self, headers: impl IntoIterator<Item = S>) -> Self {
        self.headers = headers.into_iter().map(Into::into).collect();
        self
    }
    pub fn row<S: Into<String>>(mut self, row: impl IntoIterator<Item = S>) -> Self {
        self.rows.push(row.into_iter().map(Into::into).collect());
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let headers = self.headers;
        let rows = self.rows;
        let ncols = headers
            .len()
            .max(rows.first().map_or(0, |r| r.len()))
            .max(1);
        // Card frame, full-bleed rows (no inner padding — the cells pad themselves).
        Surface::new()
            .pad(core::SPACE_0)
            .show(ui, |ui| {
                let avail = ui.available_width();
                let widths = vec![avail / ncols as f32; ncols];
                if !headers.is_empty() {
                    TableRow::new(headers).header().show(ui, &widths);
                    Divider::horizontal().show(ui);
                }
                for (r, row) in rows.into_iter().enumerate() {
                    TableRow::new(row).zebra(r % 2 == 1).show(ui, &widths);
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
