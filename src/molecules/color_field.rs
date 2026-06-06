//! ColorField molecule — a color swatch + hex readout. [Unity Color Field]
//!
//! v1 is display (swatch + hex); the eyedropper/picker popover lands with the overlay wave.

use crate::atoms::{ColorSwatch, Text};
use crate::tokens::core;
use egui::{Color32, Response, Ui};

/// A color field showing `color` as a swatch + hex. `show` returns the swatch [`Response`]
/// (clickable — wire a picker popover later).
pub struct ColorField {
    color: Color32,
}

impl ColorField {
    pub fn new(color: Color32) -> Self {
        Self { color }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let hex = format!(
            "#{:02X}{:02X}{:02X}",
            self.color.r(),
            self.color.g(),
            self.color.b()
        );
        ui.horizontal(|ui| {
            let response = ColorSwatch::new(self.color).show(ui);
            ui.add_space(core::SPACE_2);
            Text::new(hex).caption().muted().show(ui);
            response
        })
        .inner
    }
}
