//! ListItem cell — a selectable list row (icon + title + subtitle). [shadcn Item]

use crate::atoms::{Icon, Surface, Text};
use crate::tokens::core;
use egui::{Id, Response, Ui};

/// A list row. Selected → `muted` fill; click → [`Response`].
pub struct ListItem {
    icon: Option<&'static str>,
    title: String,
    subtitle: Option<String>,
    selected: bool,
    id_source: Option<Id>,
}

impl ListItem {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            icon: None,
            title: title.into(),
            subtitle: None,
            selected: false,
            id_source: None,
        }
    }
    pub fn icon(mut self, glyph: &'static str) -> Self {
        self.icon = Some(glyph);
        self
    }
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id_source = Some(Id::new(id));
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let mut surface = if self.selected {
            Surface::new().muted().border_none()
        } else {
            Surface::new().fill_none().border_none()
        }
        .interactive()
        .pad(core::SPACE_2)
        .radius(core::RADIUS_SM);
        if let Some(id) = self.id_source {
            surface = surface.id_source(id);
        }
        let icon = self.icon;
        let title = self.title;
        let subtitle = self.subtitle;
        surface
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if let Some(glyph) = icon {
                        Icon::new(glyph).muted().show(ui);
                        ui.add_space(core::SPACE_2);
                    }
                    ui.vertical(|ui| {
                        Text::new(title).show(ui);
                        if let Some(subtitle) = subtitle {
                            Text::new(subtitle).caption().muted().show(ui);
                        }
                    });
                });
            })
            .response
    }
}
