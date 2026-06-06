//! Table organism — a header + data rows at fixed columns. [shadcn Table / Unity Multi-column]

use crate::atoms::Divider;
use crate::cells::TableRow;
use crate::tokens::core;
use egui::{Response, ScrollArea, Ui};

/// A data table. Composes [`TableRow`] cells under a header rule, scrollable.
pub struct Table {
    headers: Vec<String>,
    widths: Vec<f32>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            widths: Vec::new(),
            rows: Vec::new(),
        }
    }
    pub fn headers<S: Into<String>>(mut self, headers: impl IntoIterator<Item = S>) -> Self {
        self.headers = headers.into_iter().map(Into::into).collect();
        self
    }
    pub fn widths(mut self, widths: impl IntoIterator<Item = f32>) -> Self {
        self.widths = widths.into_iter().collect();
        self
    }
    pub fn row<S: Into<String>>(mut self, row: impl IntoIterator<Item = S>) -> Self {
        self.rows.push(row.into_iter().map(Into::into).collect());
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let widths = self.widths;
        let headers = self.headers;
        let rows = self.rows;
        ui.vertical(|ui| {
            if !headers.is_empty() {
                TableRow::new(headers).header().show(ui, &widths);
                ui.add_space(core::SPACE_1);
                Divider::horizontal().show(ui);
                ui.add_space(core::SPACE_1);
            }
            ScrollArea::vertical().show(ui, |ui| {
                for row in rows {
                    TableRow::new(row).show(ui, &widths);
                    ui.add_space(core::SPACE_1);
                }
            });
        })
        .response
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}
