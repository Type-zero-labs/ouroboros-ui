//! Popover organism — content anchored to a trigger, on click. [shadcn Popover]
//!
//! Wraps [`egui::Popup`] (menu style); the frame inherits themed menu visuals. The substrate
//! for color pickers, selects, combobox and menus.

use egui::{Response, Ui};

/// A click-triggered popover anchored to a widget [`Response`].
pub struct Popover;

impl Popover {
    pub fn new() -> Self {
        Self
    }

    /// Show `content` in a popover that opens when `trigger` is clicked.
    pub fn show(self, trigger: &Response, content: impl FnOnce(&mut Ui)) {
        egui::Popup::menu(trigger).show(|ui| {
            content(ui);
        });
    }
}

impl Default for Popover {
    fn default() -> Self {
        Self::new()
    }
}
