//! AppMenu organism — the app-wide menu opened from the rail logo. [Figma main menu]
//!
//! One popup with titled sections (File, Edit, Tools, Settings, Window…), rows built from the
//! [`MenuItem`] cell (icon, shortcut, check, disabled), separators and nested submenus. The
//! caller fills it in a closure and reacts to clicks **inline** — the menu closes itself when
//! a row is clicked.

use crate::atoms::{Divider, Text};
use crate::cells::MenuItem;
use crate::tokens::core;
use egui::{Response, Ui};

/// The menu being filled. Rows report `true` when clicked.
pub struct MenuUi<'u> {
    ui: &'u mut Ui,
    /// Ids derive from the path in the menu: a counter alone repeats at every level, and the
    /// same id in a submenu's layer trips egui ("widget changed layer_id").
    base: egui::Id,
    n: usize,
}

impl MenuUi<'_> {
    /// A muted section title (e.g. "File").
    pub fn section(&mut self, title: impl Into<String>) {
        if self.n > 0 {
            self.separator();
        }
        self.ui.add_space(core::SPACE_1);
        Text::new(title).caption().muted().show(self.ui);
    }
    /// A row; `true` when clicked (the menu then closes).
    pub fn add(&mut self, item: MenuItem) -> bool {
        self.n += 1;
        let clicked = item
            .id_source(self.base.with(("row", self.n)))
            .show(self.ui)
            .clicked();
        if clicked {
            self.ui.close();
        }
        clicked
    }
    pub fn separator(&mut self) {
        self.ui.add_space(core::SPACE_1);
        Divider::horizontal().show(self.ui);
        self.ui.add_space(core::SPACE_1);
    }
    /// A nested submenu, opened on hover (a [`MenuItem`] row with a caret).
    pub fn submenu(&mut self, label: impl Into<String>, add: impl FnOnce(&mut MenuUi)) {
        self.n += 1;
        let base = self.base.with(("sub", self.n));
        let row = MenuItem::new(label).submenu().id_source(base).show(self.ui);
        egui::containers::menu::SubMenu::new().show(self.ui, &row, |ui| {
            fix_width(ui);
            add(&mut MenuUi { ui, base, n: 0 });
        });
    }
    /// Raw access for custom rows (an input, a toggle group).
    pub fn ui(&mut self) -> &mut Ui {
        self.ui
    }
}

/// The app menu. `show` opens it under/next to `trigger` when the trigger is clicked.
pub struct AppMenu;

impl AppMenu {
    pub fn show(trigger: &Response, add: impl FnOnce(&mut MenuUi)) {
        let base = trigger.id.with("app_menu");
        egui::Popup::menu(trigger).show(|ui: &mut Ui| {
            fix_width(ui);
            add(&mut MenuUi { ui, base, n: 0 });
        });
    }
}

/// Menus have a fixed width: right-aligned shortcuts and dividers fill the available
/// width, which in a popup is the whole screen.
fn fix_width(ui: &mut Ui) {
    let w = core::SPACE_12 * 6.0;
    ui.set_min_width(w);
    ui.set_max_width(w);
}
