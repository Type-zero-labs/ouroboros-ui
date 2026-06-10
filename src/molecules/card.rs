//! Card molecule — an elevated [`Surface`] with header / content / footer.
//!
//! Header holds title + description + an optional top-right **action** slot (shadcn CardAction).
//! `size` (default/sm — shadcn) scales the spacing.

use crate::atoms::{Divider, Heading, Surface, Text};
use crate::tokens::core;
use egui::{Align, Layout, Response, Ui};

type SlotFn<'a> = Box<dyn FnOnce(&mut Ui) + 'a>;

/// Card spacing scale. [shadcn size]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardSize {
    #[default]
    Default,
    Sm,
}

/// A content card. `show` runs `content` between an optional header and footer.
pub struct Card<'a> {
    title: Option<String>,
    description: Option<String>,
    action: Option<SlotFn<'a>>,
    footer: Option<SlotFn<'a>>,
    size: CardSize,
}

impl<'a> Card<'a> {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            action: None,
            footer: None,
            size: CardSize::default(),
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
    /// Top-right header slot (a button / menu / badge). [shadcn CardAction]
    pub fn action(mut self, action: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.action = Some(Box::new(action));
        self
    }
    pub fn footer(mut self, footer: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.footer = Some(Box::new(footer));
        self
    }
    pub fn size(mut self, size: CardSize) -> Self {
        self.size = size;
        self
    }
    pub fn sm(self) -> Self {
        self.size(CardSize::Sm)
    }

    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Response {
        let (pad, gap) = match self.size {
            CardSize::Default => (core::SPACE_4, core::SPACE_3),
            CardSize::Sm => (core::SPACE_3, core::SPACE_2),
        };
        Surface::new()
            .elevated()
            .pad(pad)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    let has_header =
                        self.title.is_some() || self.description.is_some() || self.action.is_some();
                    if has_header {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                if let Some(title) = self.title {
                                    Heading::new(title).heading().show(ui);
                                }
                                if let Some(description) = self.description {
                                    ui.add_space(core::SPACE_1);
                                    Text::new(description).caption().muted().wrap().show(ui);
                                }
                            });
                            if let Some(action) = self.action {
                                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                                    action(ui);
                                });
                            }
                        });
                        ui.add_space(gap);
                    }
                    content(ui);
                    if let Some(footer) = self.footer {
                        ui.add_space(gap);
                        Divider::horizontal().show(ui);
                        ui.add_space(gap);
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
