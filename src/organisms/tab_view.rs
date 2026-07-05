//! TabView organism — a tab bar + the selected panel. [shadcn Tabs]

use crate::atoms::Divider;
use crate::molecules::Tabs;
use crate::tokens::core;
use egui::{Response, Ui};

/// A tab bar bound to a `&mut usize` plus the active panel. `panel(ui, index)` renders the body.
pub struct TabView<'a> {
    selected: &'a mut usize,
    tabs: Vec<String>,
}

impl<'a> TabView<'a> {
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

    pub fn show(self, ui: &mut Ui, panel: impl FnOnce(&mut Ui, usize)) -> Response {
        let mut idx = *self.selected;
        let response = ui
            .vertical(|ui| {
                Tabs::new(&mut idx).tabs(self.tabs).show(ui);
                ui.add_space(core::SPACE_2);
                Divider::horizontal().show(ui);
                ui.add_space(core::SPACE_3);
                panel(ui, idx);
            })
            .response;
        *self.selected = idx;
        response
    }
}
