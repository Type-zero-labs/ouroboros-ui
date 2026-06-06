//! TreeNode cell — an indented hierarchy row (caret + icon + label). [Unity/O3DE Tree View]

use crate::atoms::{Icon, Surface, Text};
use crate::tokens::core;
use egui::{Id, Response, Ui};
use egui_phosphor::light;

/// A tree row. `depth` indents; `expandable`/`expanded` draw a caret; selected → `muted` fill.
/// `show` returns the [`Response`] (the consumer toggles expand / selection).
pub struct TreeNode {
    label: String,
    depth: usize,
    icon: Option<&'static str>,
    expandable: bool,
    expanded: bool,
    selected: bool,
    id_source: Option<Id>,
}

impl TreeNode {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            depth: 0,
            icon: None,
            expandable: false,
            expanded: false,
            selected: false,
            id_source: None,
        }
    }
    pub fn depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }
    pub fn icon(mut self, glyph: &'static str) -> Self {
        self.icon = Some(glyph);
        self
    }
    pub fn expandable(mut self, expanded: bool) -> Self {
        self.expandable = true;
        self.expanded = expanded;
        self
    }
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id_source = Some(Id::new(id));
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let mut surface = if self.selected {
            Surface::new().muted().border_none()
        } else {
            Surface::new().fill_none().border_none()
        }
        .interactive()
        .pad(core::SPACE_1)
        .radius(core::RADIUS_SM);
        if let Some(id) = self.id_source {
            surface = surface.id_source(id);
        }
        let TreeNode {
            label,
            depth,
            icon,
            expandable,
            expanded,
            ..
        } = self;
        surface
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(depth as f32 * core::SPACE_4);
                    if expandable {
                        let caret = if expanded {
                            light::CARET_DOWN
                        } else {
                            light::CARET_RIGHT
                        };
                        Icon::new(caret).muted().sm().show(ui);
                    } else {
                        ui.add_space(core::ICON_SM);
                    }
                    ui.add_space(core::SPACE_1);
                    if let Some(glyph) = icon {
                        Icon::new(glyph).muted().show(ui);
                        ui.add_space(core::SPACE_1);
                    }
                    Text::new(label).show(ui);
                });
            })
            .response
    }
}
