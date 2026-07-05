//! MenuItem cell — a menu row (icon + label + shortcut). [shadcn DropdownMenu item]

use crate::atoms::{Icon, Kbd, Surface, Text};
use crate::tokens::core;
use egui::{Align, Id, Layout, Response, Ui};
use egui_phosphor::light;

/// A menu row. Composes icon + label + optional [`Kbd`] shortcut; click → [`Response`].
pub struct MenuItem {
    icon: Option<&'static str>,
    label: String,
    shortcut: Option<String>,
    enabled: bool,
    checked: Option<bool>,
    id_source: Option<Id>,
}

impl MenuItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            icon: None,
            label: label.into(),
            shortcut: None,
            enabled: true,
            checked: None,
            id_source: None,
        }
    }
    pub fn icon(mut self, glyph: &'static str) -> Self {
        self.icon = Some(glyph);
        self
    }
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    /// Checkable item (a View-menu toggle): `true` shows a check mark; `false` reserves
    /// the mark's width so checked/unchecked siblings stay aligned. [shadcn CheckboxItem]
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id_source = Some(Id::new(id));
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let mut surface = Surface::new()
            .fill_none()
            .border_none()
            .pad(core::SPACE_1)
            .radius(core::RADIUS_SM);
        if self.enabled {
            surface = surface.interactive();
        }
        if let Some(id) = self.id_source {
            surface = surface.id_source(id);
        }
        let icon = self.icon;
        let label = self.label;
        let shortcut = self.shortcut;
        let checked = self.checked;
        surface
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    match checked {
                        Some(true) => {
                            Icon::new(light::CHECK).show(ui);
                            ui.add_space(core::SPACE_2);
                        }
                        Some(false) => {
                            // Reserve the mark's slot so siblings line up.
                            ui.add_space(core::ICON_MD + core::SPACE_2);
                        }
                        None => {}
                    }
                    if let Some(glyph) = icon {
                        Icon::new(glyph).muted().show(ui);
                        ui.add_space(core::SPACE_2);
                    }
                    Text::new(label).show(ui);
                    if let Some(shortcut) = shortcut {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            Kbd::new(shortcut).show(ui);
                        });
                    }
                });
            })
            .response
    }
}
