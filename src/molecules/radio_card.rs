//! RadioCard molecule — a selectable [`Surface`] wrapping a display [`Radio`] + text.
//!
//! Stateless like the [`Radio`] atom: reports clicks; the consumer manages exclusivity.

use crate::atoms::{Radio, Surface, Text};
use crate::tokens::core;
use egui::{Id, Response, Ui};

/// A radio-as-card. `selected` drives the visual; `show` returns the [`Response`] (`clicked`).
pub struct RadioCard {
    selected: bool,
    label: String,
    description: Option<String>,
    id_source: Option<Id>,
}

impl RadioCard {
    pub fn new(selected: bool, label: impl Into<String>) -> Self {
        Self {
            selected,
            label: label.into(),
            description: None,
            id_source: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id_source = Some(Id::new(id));
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let selected = self.selected;
        let label = self.label;
        let description = self.description;
        let mut surface = Surface::new()
            .interactive()
            .selected(selected)
            .pad(core::SPACE_3);
        if let Some(id) = self.id_source {
            surface = surface.id_source(id);
        }
        surface
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    Radio::new(selected).interactive(false).show(ui);
                    ui.add_space(core::SPACE_3);
                    ui.vertical(|ui| {
                        Text::new(label).body_strong().show(ui);
                        if let Some(description) = description {
                            Text::new(description).caption().muted().wrap().show(ui);
                        }
                    });
                });
            })
            .response
    }
}
