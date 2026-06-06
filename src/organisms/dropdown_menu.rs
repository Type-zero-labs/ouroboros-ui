//! DropdownMenu organism — a popover of [`MenuItem`]s. [shadcn DropdownMenu / ContextMenu]

use crate::cells::MenuItem;
use egui::{Response, Ui};

/// A dropdown menu opened from a trigger. `show` returns the index of the item clicked, if any.
pub struct DropdownMenu {
    items: Vec<(Option<&'static str>, String)>,
}

impl DropdownMenu {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn item(mut self, icon: &'static str, label: impl Into<String>) -> Self {
        self.items.push((Some(icon), label.into()));
        self
    }
    pub fn text_item(mut self, label: impl Into<String>) -> Self {
        self.items.push((None, label.into()));
        self
    }

    pub fn show(self, trigger: &Response) -> Option<usize> {
        let items = self.items;
        egui::Popup::menu(trigger)
            .show(|ui: &mut Ui| {
                let mut clicked = None;
                for (i, (icon, label)) in items.into_iter().enumerate() {
                    let mut item = MenuItem::new(label).id_source(("dropdown", i));
                    if let Some(glyph) = icon {
                        item = item.icon(glyph);
                    }
                    if item.show(ui).clicked() {
                        clicked = Some(i);
                        ui.close();
                    }
                }
                clicked
            })
            .and_then(|r| r.inner)
    }
}

impl Default for DropdownMenu {
    fn default() -> Self {
        Self::new()
    }
}
