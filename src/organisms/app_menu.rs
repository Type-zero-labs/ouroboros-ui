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
            .id_source(("app_menu_row", self.n))
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
    /// A nested submenu opened on hover/click.
    pub fn submenu(&mut self, label: impl Into<String>, add: impl FnOnce(&mut MenuUi)) {
        self.n += 1;
        self.ui.menu_button(label.into(), |ui| {
            ui.set_min_width(core::SPACE_12 * 4.0);
            add(&mut MenuUi { ui, n: 0 });
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
        egui::Popup::menu(trigger).show(|ui: &mut Ui| {
            ui.set_min_width(core::SPACE_12 * 5.0);
            add(&mut MenuUi { ui, n: 0 });
        });
    }
}
