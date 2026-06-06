//! Alert molecule — a status callout. [shadcn Alert / Unity Help Box]

use crate::atoms::{Icon, Surface, Text};
use crate::tokens::core;
use crate::Theme;
use egui::{Response, Ui};
use egui_phosphor::light;

/// Alert status. Drives the icon glyph + accent color.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// A status callout. Composes [`Surface`] + status [`Icon`] + [`Text`].
pub struct Alert {
    message: String,
    title: Option<String>,
    variant: AlertVariant,
}

impl Alert {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            title: None,
            variant: AlertVariant::default(),
        }
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn info(self) -> Self {
        self.variant(AlertVariant::Info)
    }
    pub fn success(self) -> Self {
        self.variant(AlertVariant::Success)
    }
    pub fn warning(self) -> Self {
        self.variant(AlertVariant::Warning)
    }
    pub fn error(self) -> Self {
        self.variant(AlertVariant::Error)
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let (glyph, color) = match self.variant {
            AlertVariant::Info => (light::INFO, theme.info),
            AlertVariant::Success => (light::CHECK_CIRCLE, theme.success),
            AlertVariant::Warning => (light::WARNING, theme.warning),
            AlertVariant::Error => (light::WARNING_CIRCLE, theme.error),
        };
        let title = self.title;
        let message = self.message;
        Surface::new()
            .pad(core::SPACE_3)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    Icon::new(glyph).color(color).show(ui);
                    ui.add_space(core::SPACE_2);
                    ui.vertical(|ui| {
                        if let Some(title) = title {
                            Text::new(title).body_strong().color(color).show(ui);
                            ui.add_space(core::SPACE_1);
                        }
                        Text::new(message).muted().show(ui);
                    });
                });
            })
            .response
    }
}
