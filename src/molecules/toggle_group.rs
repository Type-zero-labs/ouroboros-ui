//! ToggleGroup molecule — a segmented single-select control. [shadcn Toggle Group / Button Group]

use crate::atoms::{Button, ButtonVariant, Surface};
use crate::tokens::core;
use egui::{Response, Ui};

/// A segmented single-select bound to a `&mut usize`. The selected segment is a raised secondary
/// [`Button`] (same style as a real button), the rest ghost — inside a [`Surface`] container.
pub struct ToggleGroup<'a> {
    selected: &'a mut usize,
    options: Vec<String>,
}

impl<'a> ToggleGroup<'a> {
    pub fn new(selected: &'a mut usize) -> Self {
        Self {
            selected,
            options: Vec::new(),
        }
    }
    pub fn options<S: Into<String>>(mut self, options: impl IntoIterator<Item = S>) -> Self {
        self.options = options.into_iter().map(Into::into).collect();
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let selected = self.selected;
        let options = self.options;
        Surface::new()
            .pad(core::SPACE_1)
            .radius(core::RADIUS_MD)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (i, option) in options.iter().enumerate() {
                        let variant = if *selected == i {
                            ButtonVariant::Secondary
                        } else {
                            ButtonVariant::Ghost
                        };
                        if Button::new(option)
                            .variant(variant)
                            .sm()
                            .id_source(("toggle_group", i))
                            .show(ui)
                            .clicked()
                        {
                            *selected = i;
                        }
                    }
                });
            })
            .response
    }
}
