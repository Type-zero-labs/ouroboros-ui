//! Card molecule — an elevated [`Surface`] with optional header/footer and content.

use crate::atoms::{Divider, Heading, Surface, Text};
use crate::tokens::core;
use egui::{Response, Ui};

type FooterFn<'a> = Box<dyn FnOnce(&mut Ui) + 'a>;

/// A content card. `show` runs `content` between an optional header and footer.
pub struct Card<'a> {
    title: Option<String>,
    description: Option<String>,
    footer: Option<FooterFn<'a>>,
}

impl<'a> Card<'a> {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            footer: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn footer(mut self, footer: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.footer = Some(Box::new(footer));
        self
    }

    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Response {
        Surface::new()
            .elevated()
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    let has_header = self.title.is_some() || self.description.is_some();
                    if let Some(title) = self.title {
                        Heading::new(title).heading().show(ui);
                    }
                    if let Some(description) = self.description {
                        ui.add_space(core::SPACE_1);
                        Text::new(description).caption().muted().show(ui);
                    }
                    if has_header {
                        ui.add_space(core::SPACE_3);
                    }
                    content(ui);
                    if let Some(footer) = self.footer {
                        ui.add_space(core::SPACE_3);
                        Divider::horizontal().show(ui);
                        ui.add_space(core::SPACE_3);
                        footer(ui);
                    }
                });
            })
            .response
    }
}

impl Default for Card<'_> {
    fn default() -> Self {
        Self::new()
    }
}
