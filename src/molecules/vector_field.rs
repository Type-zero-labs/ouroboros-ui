//! VectorField molecule — N numeric components (Vec2/3/4) in a row. [Unity Vector Field]

use crate::atoms::{NumericField, Text};
use crate::tokens::core;
use egui::{vec2, Ui};

const AXES: [&str; 4] = ["X", "Y", "Z", "W"];

/// A vector of `f32` components edited side-by-side (each an [`NumericField`]).
pub struct VectorField<'a> {
    values: &'a mut [f32],
    speed: f32,
}

impl<'a> VectorField<'a> {
    pub fn new(values: &'a mut [f32]) -> Self {
        Self { values, speed: 0.1 }
    }
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub fn show(self, ui: &mut Ui) {
        let n = self.values.len().max(1);
        let speed = self.speed;
        // Split the row width evenly; reserve a label + gaps per component.
        let overhead = core::SPACE_6;
        let field_w = ((ui.available_width() / n as f32) - overhead).max(core::SPACE_12);
        ui.horizontal(|ui| {
            for (i, v) in self.values.iter_mut().enumerate() {
                Text::new(AXES.get(i).copied().unwrap_or("·"))
                    .caption()
                    .muted()
                    .show(ui);
                ui.add_space(core::SPACE_1);
                ui.allocate_ui(vec2(field_w, core::CONTROL_MD), |ui| {
                    NumericField::new(v).speed(speed).show(ui);
                });
                ui.add_space(core::SPACE_2);
            }
        });
    }
}
