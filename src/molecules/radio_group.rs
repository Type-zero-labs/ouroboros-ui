//! RadioGroup molecule — single-select group of [`Radio`] atoms.

use crate::atoms::Radio;
use crate::tokens::core;
use egui::{Response, Ui};

/// A single-select radio group bound to a `&mut usize`. `show` composes one [`Radio`] per option.
pub struct RadioGroup<'a> {
    selected: &'a mut usize,
    options: Vec<String>,
    horizontal: bool,
}

impl<'a> RadioGroup<'a> {
    pub fn new(selected: &'a mut usize) -> Self {
        Self {
            selected,
            options: Vec::new(),
            horizontal: false,
        }
    }

    pub fn options<S: Into<String>>(mut self, options: impl IntoIterator<Item = S>) -> Self {
        self.options = options.into_iter().map(Into::into).collect();
        self
    }
    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let selected = self.selected;
        let options = self.options;
        let body = |ui: &mut Ui| {
            for (i, option) in options.iter().enumerate() {
                if Radio::new(*selected == i)
                    .label(option)
                    .id_source(("radio_group", i))
                    .show(ui)
                    .clicked()
                {
                    *selected = i;
                }
                ui.add_space(core::SPACE_1);
            }
        };
        if self.horizontal {
            ui.horizontal(body).response
        } else {
            ui.vertical(body).response
        }
    }
}
