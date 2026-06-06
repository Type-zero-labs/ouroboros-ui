//! TableCell cell — one data/header cell of a table. [legacy ouroboros-ui table style]
//!
//! Left-aligned, vertically centered text in a fixed-width column, with an optional status
//! dot (a [`ColorSwatch`]). `header()` styles it as a muted column label.

use crate::atoms::{ColorSwatch, Text};
use crate::tokens::{core, layout};
use egui::{vec2, Align, Color32, Layout, Response, Ui};

/// One table cell. `show(ui, width)` lays it out in a `width × TABLE_ROW_HEIGHT` box.
pub struct TableCell {
    text: String,
    header: bool,
    status: Option<Color32>,
}

impl TableCell {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            header: false,
            status: None,
        }
    }
    /// Style as a muted column header.
    pub fn header(mut self) -> Self {
        self.header = true;
        self
    }
    /// Show a leading status dot in `color`.
    pub fn status(mut self, color: Color32) -> Self {
        self.status = Some(color);
        self
    }

    pub fn show(self, ui: &mut Ui, width: f32) -> Response {
        ui.allocate_ui_with_layout(
            vec2(width, layout::TABLE_ROW_HEIGHT),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.add_space(core::SPACE_2);
                if let Some(color) = self.status {
                    ColorSwatch::new(color)
                        .circle()
                        .size(core::SPACE_2)
                        .show(ui);
                    ui.add_space(core::SPACE_2);
                }
                if self.header {
                    Text::new(self.text).label().muted().show(ui);
                } else {
                    Text::new(self.text).show(ui);
                }
            },
        )
        .response
    }
}
