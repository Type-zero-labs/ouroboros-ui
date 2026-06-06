//! Select organism — a dropdown single-select. [shadcn Select / Unity Dropdown / O3DE Dropdown]
//!
//! A trigger button showing the current option + a [`Popup`](egui::Popup) of [`MenuItem`]s.

use crate::atoms::{Button, ButtonVariant};
use crate::cells::MenuItem;
use egui::{Response, Ui};
use egui_phosphor::light;

/// A select bound to a `&mut usize`. `show` returns the trigger [`Response`].
pub struct Select<'a> {
    selected: &'a mut usize,
    options: Vec<String>,
    placeholder: String,
}

impl<'a> Select<'a> {
    pub fn new(selected: &'a mut usize) -> Self {
        Self {
            selected,
            options: Vec::new(),
            placeholder: "Select…".to_owned(),
        }
    }
    pub fn options<S: Into<String>>(mut self, options: impl IntoIterator<Item = S>) -> Self {
        self.options = options.into_iter().map(Into::into).collect();
        self
    }
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let selected = self.selected;
        let options = self.options;
        let current = options
            .get(*selected)
            .cloned()
            .unwrap_or_else(|| self.placeholder.clone());
        let response = Button::new(current)
            .variant(ButtonVariant::Outline)
            .icon_right(light::CARET_DOWN)
            .id_source("select_trigger")
            .show(ui);
        egui::Popup::menu(&response).show(|ui: &mut Ui| {
            for (i, option) in options.iter().enumerate() {
                if MenuItem::new(option)
                    .id_source(("select", i))
                    .show(ui)
                    .clicked()
                {
                    *selected = i;
                    ui.close();
                }
            }
        });
        response
    }
}
