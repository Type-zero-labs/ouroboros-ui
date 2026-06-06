//! Dialog organism — a modal with title/description + body. [shadcn Dialog / Unity Overlay]
//!
//! Uses [`egui::Modal`] (scrim + centered) whose frame inherits the themed window visuals.

use crate::atoms::{Heading, Text};
use crate::tokens::{core, layout};
use egui::{Context, Id, Ui};

/// A modal dialog. Render only while open; `show` returns `true` when it should close
/// (backdrop click / Esc).
pub struct Dialog {
    id: Id,
    title: String,
    description: Option<String>,
}

impl Dialog {
    pub fn new(title: impl Into<String>) -> Self {
        let title = title.into();
        Self {
            id: Id::new(format!("dialog::{title}")),
            title,
            description: None,
        }
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id = Id::new(id);
        self
    }
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn show(self, ctx: &Context, body: impl FnOnce(&mut Ui)) -> bool {
        let title = self.title;
        let description = self.description;
        egui::Modal::new(self.id)
            .show(ctx, |ui| {
                ui.set_max_width(layout::PANEL_MAX);
                Heading::new(title).h2().show(ui);
                if let Some(description) = description {
                    ui.add_space(core::SPACE_1);
                    Text::new(description).muted().show(ui);
                }
                ui.add_space(core::SPACE_4);
                body(ui);
            })
            .should_close()
    }
}
