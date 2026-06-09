//! PropertyRow cell — an aligned inspector row (label column ↔ control). [Unity inspector]

use crate::atoms::Text;
use crate::tokens::{core, layout};
use egui::{vec2, Align, Layout, Response, Ui};

/// An inspector property row: a fixed-width label column then the control.
pub struct PropertyRow {
    label: String,
    label_width: f32,
}

impl PropertyRow {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            label_width: layout::PROPERTY_LABEL_WIDTH,
        }
    }
    pub fn label_width(mut self, width: f32) -> Self {
        self.label_width = width;
        self
    }

    pub fn show(self, ui: &mut Ui, control: impl FnOnce(&mut Ui) -> Response) -> Response {
        let label = self.label;
        let label_width = self.label_width;
        ui.horizontal(|ui| {
            ui.allocate_ui_with_layout(
                vec2(label_width, core::CONTROL_MD),
                Layout::left_to_right(Align::Center),
                |ui| {
                    Text::new(label).muted().show(ui);
                },
            );
            // Control anchored to the RIGHT (Unity-inspector style, like the data-table value
            // column): a fixed-width control sits at the right edge and the gap between label and
            // control flexes on resize. A filling control (text input) still fills the remainder.
            ui.with_layout(Layout::right_to_left(Align::Center), control)
                .inner
        })
        .inner
    }
}
