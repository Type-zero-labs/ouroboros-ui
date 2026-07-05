//! Breadcrumb molecule — a path trail. [shadcn Breadcrumb]

use crate::atoms::{Button, ButtonVariant, Icon, Text};
use egui::Ui;
use egui_phosphor::light;

/// A breadcrumb trail. Last item is plain text; earlier items are link buttons separated by carets.
pub struct Breadcrumb {
    items: Vec<String>,
}

impl Breadcrumb {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn items<S: Into<String>>(mut self, items: impl IntoIterator<Item = S>) -> Self {
        self.items = items.into_iter().map(Into::into).collect();
        self
    }

    /// Returns the index of the crumb clicked this frame, if any.
    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let items = self.items;
        let n = items.len();
        ui.horizontal(|ui| {
            let mut clicked = None;
            for (i, item) in items.iter().enumerate() {
                if i + 1 == n {
                    Text::new(item).body_strong().show(ui);
                } else {
                    if Button::new(item)
                        .variant(ButtonVariant::Link)
                        .sm()
                        .id_source(("crumb", i))
                        .show(ui)
                        .clicked()
                    {
                        clicked = Some(i);
                    }
                    Icon::new(light::CARET_RIGHT).muted().sm().show(ui);
                }
            }
            clicked
        })
        .inner
    }
}

impl Default for Breadcrumb {
    fn default() -> Self {
        Self::new()
    }
}
