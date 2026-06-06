//! Toolbar organism — a horizontal action bar. [Unity/O3DE Toolbar]

use crate::atoms::Surface;
use crate::tokens::core;
use egui::{Response, Ui};

/// A toolbar bar. `show` lays `content` out horizontally inside a muted [`Surface`].
pub struct Toolbar;

impl Toolbar {
    pub fn new() -> Self {
        Self
    }
    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Response {
        Surface::new()
            .muted()
            .border_none()
            .pad(core::SPACE_1)
            .radius(core::RADIUS_MD)
            .show(ui, |ui| {
                ui.horizontal(content);
            })
            .response
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}
