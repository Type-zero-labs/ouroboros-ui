//! ResponsiveRow cell — an inspector row that stacks label↔control when narrow.
//! [Unity inspector + shadcn responsive]
//!
//! The responsive sibling of [`PropertyRow`](super::property_row::PropertyRow): wide, it keeps the
//! aligned label column with a right-anchored control; below [`layout::INSPECTOR_ROW_STACK_MIN`] it
//! stacks the label above a full-width control so a squeezed side panel never clips the pair.
//! Models the threshold switch on [`Field`](crate::molecules::Field)'s responsive orientation.

use crate::atoms::Text;
use crate::tokens::{core, layout};
use egui::{vec2, Align, Layout, Response, Ui};

/// An inspector property row that goes vertical (label above control) when the available width
/// drops below [`layout::INSPECTOR_ROW_STACK_MIN`]. Builder; `show` runs `control` in the value
/// slot.
pub struct ResponsiveRow {
    label: String,
    label_width: f32,
}

impl ResponsiveRow {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            label_width: layout::PROPERTY_LABEL_WIDTH,
        }
    }
    /// Width of the aligned label column (wide layout). Default [`layout::PROPERTY_LABEL_WIDTH`].
    pub fn label_width(mut self, width: f32) -> Self {
        self.label_width = width;
        self
    }

    pub fn show(self, ui: &mut Ui, control: impl FnOnce(&mut Ui) -> Response) -> Response {
        let label = self.label;
        let label_width = self.label_width;
        if ui.available_width() >= layout::INSPECTOR_ROW_STACK_MIN {
            // Wide: aligned label column then a right-anchored control (PropertyRow behaviour) —
            // the gap between them flexes on resize, a filling control still fills the remainder.
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    vec2(label_width, core::CONTROL_MD),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        Text::new(label).muted().show(ui);
                    },
                );
                ui.with_layout(Layout::right_to_left(Align::Center), control)
                    .inner
            })
            .inner
        } else {
            // Narrow: label stacked above a full-width control — never clips when the panel is
            // squeezed below the stack threshold (the ds-inspector <260px clip fix).
            ui.vertical(|ui| {
                Text::new(label).muted().show(ui);
                ui.add_space(core::SPACE_1);
                control(ui)
            })
            .inner
        }
    }
}
