//! Table organism — a column-defined data table on `egui_extras::TableBuilder`.
//!
//! Element-Plus-flavored: columns describe layout (width/min/align), rows carry
//! [`TableCell`]s. `egui_extras` provides sizing, a sticky header, scrolling and striping;
//! zebra/selection/hover colors come from the theme via [`table_visuals`] (set on the `ui`,
//! never painted). Cells render through [`TableCell`]; the organism composes — it does not paint.
//!
//! ```ignore
//! Table::new()
//!     .columns([Column::new("Name"), Column::new("Type").end(), Column::new("Size").exact(80.0)])
//!     .rows(data.iter().map(|d| TableRow::new([
//!         TableCell::text(&d.name),
//!         TableCell::text(&d.kind).end(),
//!         TableCell::text(&d.size).end(),
//!     ])))
//!     .striped(true).border(true).max_height(240.0)
//!     .show(ui);
//! ```

use crate::atoms::{Spinner, Surface, Text};
use crate::cells::{CellAlign, TableCell, TableRow};
use crate::tokens::core;
use crate::{Size, Theme};
use egui::{Align, Id, Layout, Response, Sense, Ui};
use egui_extras::{Column as ExtraColumn, TableBuilder};

/// How a column is sized.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ColWidth {
    /// Size to content.
    Auto,
    /// Fixed width (px).
    Exact(f32),
    /// Initial width (px), resizable/sharable.
    Initial(f32),
    /// Share the leftover width.
    #[default]
    Remainder,
}

/// A table column descriptor (header label + layout).
pub struct Column {
    label: String,
    width: ColWidth,
    min_width: Option<f32>,
    align: CellAlign,
}

impl Column {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            width: ColWidth::default(),
            min_width: None,
            align: CellAlign::Start,
        }
    }
    pub fn width(mut self, width: ColWidth) -> Self {
        self.width = width;
        self
    }
    pub fn exact(self, px: f32) -> Self {
        self.width(ColWidth::Exact(px))
    }
    pub fn initial(self, px: f32) -> Self {
        self.width(ColWidth::Initial(px))
    }
    pub fn auto(self) -> Self {
        self.width(ColWidth::Auto)
    }
    pub fn remainder(self) -> Self {
        self.width(ColWidth::Remainder)
    }
    pub fn min_width(mut self, px: f32) -> Self {
        self.min_width = Some(px);
        self
    }
    /// Header alignment (cells carry their own alignment).
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
}

/// A column-defined data table. Builder; `show` returns the area [`Response`].
pub struct Table<'a> {
    columns: Vec<Column>,
    rows: Vec<TableRow<'a>>,
    size: Size,
    striped: bool,
    border: bool,
    height: Option<f32>,
    max_height: Option<f32>,
    selectable: bool,
    loading: bool,
    empty_text: String,
    id_salt: Option<Id>,
}

impl<'a> Table<'a> {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            size: Size::default(),
            striped: false,
            border: false,
            height: None,
            max_height: None,
            selectable: false,
            loading: false,
            empty_text: "No data".to_owned(),
            id_salt: None,
        }
    }

    pub fn columns(mut self, columns: impl IntoIterator<Item = Column>) -> Self {
        self.columns = columns.into_iter().collect();
        self
    }
    pub fn rows(mut self, rows: impl IntoIterator<Item = TableRow<'a>>) -> Self {
        self.rows = rows.into_iter().collect();
        self
    }
    pub fn row(mut self, row: TableRow<'a>) -> Self {
        self.rows.push(row);
        self
    }
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
    pub fn sm(self) -> Self {
        self.size(Size::Sm)
    }
    pub fn lg(self) -> Self {
        self.size(Size::Lg)
    }
    /// Alternating row backgrounds (zebra).
    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }
    /// Frame the table with an outer border.
    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }
    /// Fixed height: header sticks, body scrolls.
    pub fn height(mut self, px: f32) -> Self {
        self.height = Some(px);
        self
    }
    /// Fluid height capped at `px` (sticky header once it scrolls).
    pub fn max_height(mut self, px: f32) -> Self {
        self.max_height = Some(px);
        self
    }
    /// Clicking a row selects it (current-row highlight, persisted for the session).
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
    pub fn empty_text(mut self, text: impl Into<String>) -> Self {
        self.empty_text = text.into();
        self
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id_salt = Some(Id::new(id));
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        // Loading: a centered spinner instead of the grid.
        if self.loading {
            return ui
                .vertical_centered(|ui| {
                    ui.add_space(core::SPACE_6);
                    Spinner::new().lg().show(ui);
                    ui.add_space(core::SPACE_6);
                })
                .response;
        }
        // Empty: a muted placeholder.
        if self.rows.is_empty() {
            return ui
                .vertical_centered(|ui| {
                    ui.add_space(core::SPACE_6);
                    Text::new(self.empty_text).muted().show(ui);
                    ui.add_space(core::SPACE_6);
                })
                .response;
        }

        if self.border {
            Surface::new()
                .pad(core::SPACE_0)
                .radius(core::RADIUS_MD)
                .show(ui, |ui| self.build(ui))
                .response
        } else {
            self.build(ui)
        }
    }

    fn build(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let id = self.id_salt.unwrap_or_else(|| ui.id().with("table"));
        let row_h = self.size.height();

        ui.scope(|ui| {
            table_visuals(ui, &theme);

            let current: Option<usize> = if self.selectable {
                ui.data(|d| d.get_temp::<usize>(id))
            } else {
                None
            };
            let mut clicked: Option<usize> = None;

            let mut tb = TableBuilder::new(ui)
                .id_salt(id)
                .striped(self.striped)
                .cell_layout(Layout::left_to_right(Align::Center))
                .vscroll(self.height.is_some() || self.max_height.is_some());
            if self.selectable {
                tb = tb.sense(Sense::click());
            }
            if let Some(h) = self.height.or(self.max_height) {
                tb = tb.max_scroll_height(h);
            }
            for col in &self.columns {
                let mut c = match col.width {
                    ColWidth::Auto => ExtraColumn::auto(),
                    ColWidth::Exact(w) => ExtraColumn::exact(w),
                    ColWidth::Initial(w) => ExtraColumn::initial(w),
                    ColWidth::Remainder => ExtraColumn::remainder(),
                };
                if let Some(m) = col.min_width {
                    c = c.at_least(m);
                }
                tb = tb.column(c.clip(true));
            }

            let columns = &self.columns;
            tb.header(row_h, |mut header| {
                for col in columns {
                    header.col(|ui| {
                        TableCell::text(col.label.clone())
                            .header()
                            .align(col.align)
                            .show(ui);
                    });
                }
            })
            .body(|mut body| {
                for (i, trow) in self.rows.into_iter().enumerate() {
                    body.row(row_h, |mut row| {
                        if self.selectable && current == Some(i) {
                            row.set_selected(true);
                        }
                        for cell in trow.cells {
                            row.col(move |ui| {
                                cell.show(ui);
                            });
                        }
                        if self.selectable && trow.selectable && row.response().clicked() {
                            clicked = Some(i);
                        }
                    });
                }
            });

            if let Some(i) = clicked {
                ui.data_mut(|d| d.insert_temp(id, i));
            }
        })
        .response
    }
}

impl Default for Table<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// Derive table zebra/selection/hover colors from the theme onto `ui` (no painting) so
/// `egui_extras`' built-in striping/selection/hover read DS tokens.
fn table_visuals(ui: &mut Ui, theme: &Theme) {
    let v = ui.visuals_mut();
    v.faint_bg_color = theme.muted;
    v.selection.bg_fill = theme.accent;
    v.selection.stroke.color = theme.accent_foreground;
    v.widgets.hovered.weak_bg_fill = theme.muted;
    v.widgets.active.weak_bg_fill = theme.muted;
}
