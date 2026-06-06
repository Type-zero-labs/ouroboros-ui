//! Menubar organism — a row of menu triggers, each a dropdown. [shadcn Menubar]

use crate::atoms::Button;
use crate::cells::MenuItem;
use egui::Ui;

/// An application menu bar. `show` returns `(menu_index, item_index)` when an item is chosen.
pub struct Menubar {
    menus: Vec<(String, Vec<String>)>,
}

impl Menubar {
    pub fn new() -> Self {
        Self { menus: Vec::new() }
    }
    pub fn menu<S: Into<String>>(
        mut self,
        label: impl Into<String>,
        items: impl IntoIterator<Item = S>,
    ) -> Self {
        self.menus
            .push((label.into(), items.into_iter().map(Into::into).collect()));
        self
    }

    pub fn show(self, ui: &mut Ui) -> Option<(usize, usize)> {
        let menus = self.menus;
        ui.horizontal(|ui| {
            let mut chosen = None;
            for (mi, (label, items)) in menus.iter().enumerate() {
                let response = Button::new(label)
                    .ghost()
                    .sm()
                    .id_source(("menubar", mi))
                    .show(ui);
                egui::Popup::menu(&response).show(|ui: &mut Ui| {
                    for (ii, item) in items.iter().enumerate() {
                        if MenuItem::new(item)
                            .id_source(("menubar_item", mi, ii))
                            .show(ui)
                            .clicked()
                        {
                            chosen = Some((mi, ii));
                            ui.close();
                        }
                    }
                });
            }
            chosen
        })
        .inner
    }
}

impl Default for Menubar {
    fn default() -> Self {
        Self::new()
    }
}
