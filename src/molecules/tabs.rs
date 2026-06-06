//! Tabs molecule — a single-select tab bar. [shadcn Tabs / Unity Tab]

use crate::atoms::{Button, ButtonVariant};
use egui::{Response, Ui};

/// A tab bar bound to a `&mut usize`. `show` composes one [`Button`] per tab.
pub struct Tabs<'a> {
    selected: &'a mut usize,
    tabs: Vec<String>,
}

impl<'a> Tabs<'a> {
    pub fn new(selected: &'a mut usize) -> Self {
        Self {
            selected,
            tabs: Vec::new(),
        }
    }
    pub fn tabs<S: Into<String>>(mut self, tabs: impl IntoIterator<Item = S>) -> Self {
        self.tabs = tabs.into_iter().map(Into::into).collect();
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let selected = self.selected;
        let tabs = self.tabs;
        ui.horizontal(|ui| {
            for (i, tab) in tabs.iter().enumerate() {
                let variant = if *selected == i {
                    ButtonVariant::Secondary
                } else {
                    ButtonVariant::Ghost
                };
                if Button::new(tab)
                    .variant(variant)
                    .sm()
                    .id_source(("tab", i))
                    .show(ui)
                    .clicked()
                {
                    *selected = i;
                }
            }
        })
        .response
    }
}
