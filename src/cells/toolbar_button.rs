//! ToolbarButton cell — an icon toggle with a tooltip. [Unity/O3DE Toolbar]

use crate::atoms::{Toggle, Tooltip};
use egui::{Id, Response, Ui};

/// A toolbar icon button bound to a `&mut bool` (active state), with an optional tooltip.
pub struct ToolbarButton<'a> {
    active: &'a mut bool,
    glyph: &'static str,
    tooltip: Option<String>,
    id_source: Option<Id>,
}

impl<'a> ToolbarButton<'a> {
    pub fn new(active: &'a mut bool, glyph: &'static str) -> Self {
        Self {
            active,
            glyph,
            tooltip: None,
            id_source: None,
        }
    }
    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id_source = Some(Id::new(id));
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let mut toggle = Toggle::new(self.active).icon(self.glyph);
        if let Some(id) = self.id_source {
            toggle = toggle.id_source(id);
        }
        let response = toggle.show(ui);
        match self.tooltip {
            Some(tooltip) => Tooltip::new(tooltip).show(response),
            None => response,
        }
    }
}
