//! CheckboxCard molecule — a selectable [`Surface`] wrapping a display [`Checkbox`] + text.
//!
//! The whole card is the click target (the inner checkbox is display-only, so there's no
//! double-toggle).

use crate::atoms::{Checkbox, Surface, Text};
use crate::tokens::core;
use egui::{Id, Response, Ui};

/// A checkbox-as-card bound to a `&mut bool`.
pub struct CheckboxCard<'a> {
    checked: &'a mut bool,
    label: String,
    description: Option<String>,
    id_source: Option<Id>,
}

impl<'a> CheckboxCard<'a> {
    pub fn new(checked: &'a mut bool, label: impl Into<String>) -> Self {
        Self {
            checked,
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
        let checked = *self.checked;
        let label = self.label;
        let description = self.description;
        let mut surface = Surface::new()
            .interactive()
            .selected(checked)
            .pad(core::SPACE_3);
        if let Some(id) = self.id_source {
            surface = surface.id_source(id);
        }
        let response = surface
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let mut shown = checked;
                    Checkbox::new(&mut shown).interactive(false).show(ui);
                    ui.add_space(core::SPACE_3);
                    ui.vertical(|ui| {
                        Text::new(label).body_strong().show(ui);
                        if let Some(description) = description {
                            Text::new(description).caption().muted().show(ui);
                        }
                    });
                });
            })
            .response;
        if response.clicked() {
            *self.checked = !*self.checked;
        }
        response
    }
}
