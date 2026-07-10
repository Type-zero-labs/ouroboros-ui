//! TableCell cell — one cell of a table: a container that places content inside a column.
//!
//! A `cell` and a `header` are the same container; they differ **basically in the text weight**
//! (header = stronger) plus role. Padding + alignment are token-driven; composes the [`Text`]
//! atom (and optionally a [`ColorSwatch`] status dot) — never paints.

use crate::atoms::{ColorSwatch, Text};
use crate::tokens::core;
use egui::{Align, Color32, Direction, Layout, Response, Ui};

/// Horizontal alignment of a cell's content.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CellAlign {
    #[default]
    Start,
    Center,
    End,
}

/// One table cell. `show(ui)` fills the column cell it's given (by the Table organism).
pub struct TableCell {
    text: String,
    header: bool,
    align: CellAlign,
    status: Option<Color32>,
    muted: bool,
}

impl TableCell {
    /// A text cell.
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            header: false,
            align: CellAlign::Start,
            status: None,
            muted: false,
        }
    }

    /// Style as a header cell (stronger text weight).
    pub fn header(mut self) -> Self {
        self.header = true;
        self
    }
    pub fn align(mut self, align: CellAlign) -> Self {
        self.align = align;
        self
    }
    pub fn center(self) -> Self {
        self.align(CellAlign::Center)
    }
    pub fn end(self) -> Self {
        self.align(CellAlign::End)
    }
    /// A leading status dot in `color`.
    pub fn status(mut self, color: Color32) -> Self {
        self.status = Some(color);
        self
    }
    /// Render the text in the muted foreground.
    pub fn muted(mut self) -> Self {
        self.muted = true;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let layout = match self.align {
            CellAlign::Start => Layout::left_to_right(Align::Center),
            CellAlign::Center => Layout::centered_and_justified(Direction::LeftToRight),
            CellAlign::End => Layout::right_to_left(Align::Center),
        };
        ui.with_layout(layout, |ui| {
            ui.add_space(core::SPACE_2);
            if let Some(color) = self.status {
                ColorSwatch::new(color)
                    .circle()
                    .size(core::SPACE_2)
                    .show(ui);
                ui.add_space(core::SPACE_1);
            }
            let mut t = Text::new(self.text);
            if self.header {
                t = t.label_strong();
            } else if self.muted {
                t = t.muted();
            }
            t.show(ui);
        })
        .response
    }
}
