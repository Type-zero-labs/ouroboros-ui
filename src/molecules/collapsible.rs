//! Collapsible molecule — a caret-headed section that hides/reveals content.
//! [shadcn Collapsible / Unity Foldout]

use crate::atoms::{Icon, Text};
use crate::tokens::core;
use egui::{Id, Response, Sense, Ui};
use egui_phosphor::light;

/// A collapsible section. Open state persists in egui memory. `show` runs `content` when open.
pub struct Collapsible {
    title: String,
    default_open: bool,
    id_source: Option<Id>,
}

impl Collapsible {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            default_open: false,
            id_source: None,
        }
    }
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id_source = Some(Id::new(id));
        self
    }

    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Response {
        let id = self
            .id_source
            .unwrap_or_else(|| Id::new(format!("collapsible::{}", self.title)));
        let mut open = ui.data(|d| d.get_temp::<bool>(id).unwrap_or(self.default_open));
        let caret = if open {
            light::CARET_DOWN
        } else {
            light::CARET_RIGHT
        };
        let title = self.title;
        let row = ui
            .horizontal(|ui| {
                Icon::new(caret).muted().show(ui);
                ui.add_space(core::SPACE_1);
                Text::new(title).body_strong().show(ui);
            })
            .response;
        let row = ui.interact(row.rect, id.with("header"), Sense::click());
        if row.clicked() {
            open = !open;
        }
        ui.data_mut(|d| d.insert_temp(id, open));
        if open {
            ui.add_space(core::SPACE_2);
            content(ui);
        }
        row
    }
}
