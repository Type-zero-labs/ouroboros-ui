//! TableRow cell — a row of text cells at fixed column widths. [shadcn Table / Unity Multi-column]
//!
//! Building block for the Table organism: lay out `cells` across `widths`.

use crate::atoms::Text;
use crate::tokens::core;
use egui::{vec2, Response, Ui};

/// A table row. `header` styles cells as muted labels.
pub struct TableRow {
    cells: Vec<String>,
    header: bool,
}

impl TableRow {
    pub fn new<S: Into<String>>(cells: impl IntoIterator<Item = S>) -> Self {
        Self {
            cells: cells.into_iter().map(Into::into).collect(),
            header: false,
        }
    }
    pub fn header(mut self) -> Self {
        self.header = true;
        self
    }

    /// Lay the cells out across `widths` (falls back to an even split for missing widths).
    pub fn show(self, ui: &mut Ui, widths: &[f32]) -> Response {
        let header = self.header;
        let cells = self.cells;
        ui.horizontal(|ui| {
            for (i, cell) in cells.iter().enumerate() {
                let width = widths.get(i).copied().unwrap_or(core::SPACE_12 * 2.0);
                ui.allocate_ui(vec2(width, core::CONTROL_SM), |ui| {
                    if header {
                        Text::new(cell).label().muted().show(ui);
                    } else {
                        Text::new(cell).show(ui);
                    }
                });
            }
        })
        .response
    }
}
