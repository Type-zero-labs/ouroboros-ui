//! Field molecule — wraps a control with a label and hint/error.
//!
//! Composes [`Text`] (label + `*` + hint/error) and the control closure, stacked vertically.

use crate::atoms::Text;
use crate::tokens::core;
use crate::Theme;
use egui::{Response, Ui};

/// A labeled form field. `show` runs `control` between the label and the hint/error.
pub struct Field {
    label: String,
    required: bool,
    hint: Option<String>,
    error: Option<String>,
}

impl Field {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            required: false,
            hint: None,
            error: None,
        }
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn show(self, ui: &mut Ui, control: impl FnOnce(&mut Ui) -> Response) -> Response {
        let theme = Theme::get(ui);
        ui.vertical(|ui| {
            if !self.label.is_empty() {
                ui.horizontal(|ui| {
                    Text::new(self.label).label().show(ui);
                    if self.required {
                        Text::new("*").label().color(theme.destructive).show(ui);
                    }
                });
                ui.add_space(core::SPACE_1);
            }
            let response = control(ui);
            if let Some(error) = self.error {
                ui.add_space(core::SPACE_1);
                Text::new(error).caption().color(theme.error).show(ui);
            } else if let Some(hint) = self.hint {
                ui.add_space(core::SPACE_1);
                Text::new(hint).caption().muted().show(ui);
            }
            response
        })
        .inner
    }
}
