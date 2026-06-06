//! Sidebar organism — a vertical navigation list. [shadcn Sidebar / Navigation Menu]

use crate::cells::ListItem;
use egui::{Response, Ui};

/// A nav list bound to a `&mut usize`. `show` composes one [`ListItem`] per entry, scrollable.
pub struct Sidebar<'a> {
    selected: &'a mut usize,
    items: Vec<(Option<&'static str>, String)>,
}

impl<'a> Sidebar<'a> {
    pub fn new(selected: &'a mut usize) -> Self {
        Self {
            selected,
            items: Vec::new(),
        }
    }
    pub fn item(mut self, icon: &'static str, label: impl Into<String>) -> Self {
        self.items.push((Some(icon), label.into()));
        self
    }
    pub fn text_item(mut self, label: impl Into<String>) -> Self {
        self.items.push((None, label.into()));
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let selected = self.selected;
        let items = self.items;
        ui.vertical(|ui| {
            for (i, (icon, label)) in items.into_iter().enumerate() {
                let mut item = ListItem::new(label)
                    .selected(*selected == i)
                    .id_source(("sidebar", i));
                if let Some(glyph) = icon {
                    item = item.icon(glyph);
                }
                if item.show(ui).clicked() {
                    *selected = i;
                }
            }
        })
        .response
    }
}
