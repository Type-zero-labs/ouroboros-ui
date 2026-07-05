//! Alert molecule — a status callout. [shadcn Alert / Unity Help Box]

use crate::atoms::{Button, Icon, Surface, Text};
use crate::tokens::core;
use crate::Theme;
use egui::{Align, Layout, Response, Ui};
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

/// A status callout. Composes [`Surface`] + status [`Icon`] + [`Text`] + an optional
/// trailing [`Button`] action.
pub struct Alert {
    message: String,
    title: Option<String>,
    variant: AlertVariant,
    action: Option<String>,
}

/// Output of [`Alert::show_with_action`]: the callout response plus whether the trailing
/// action button was clicked this frame.
pub struct AlertOutput {
    pub response: Response,
    pub action_clicked: bool,
}

impl Alert {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            title: None,
            variant: AlertVariant::default(),
            action: None,
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

    /// Add a trailing action button (right-aligned in the callout). Pair with
    /// [`show_with_action`](Self::show_with_action) to observe clicks.
    pub fn action(mut self, label: impl Into<String>) -> Self {
        self.action = Some(label.into());
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        self.render(ui).response
    }

    /// Like [`show`](Self::show), but reports whether the trailing [`action`](Self::action)
    /// button was clicked. Without an action set, `action_clicked` is always `false`.
    pub fn show_with_action(self, ui: &mut Ui) -> AlertOutput {
        self.render(ui)
    }

    fn render(self, ui: &mut Ui) -> AlertOutput {
        let theme = Theme::get(ui);
        let (glyph, color) = match self.variant {
            AlertVariant::Info => (light::INFO, theme.info),
            AlertVariant::Success => (light::CHECK_CIRCLE, theme.success),
            AlertVariant::Warning => (light::WARNING, theme.warning),
            AlertVariant::Error => (light::WARNING_CIRCLE, theme.error),
        };
        let title = self.title;
        let message = self.message;
        let action = self.action;
        let mut action_clicked = false;
        let response = Surface::new()
            .pad(core::SPACE_3)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    Icon::new(glyph).color(color).show(ui);
                    ui.add_space(core::SPACE_2);
                    ui.vertical(|ui| {
                        if let Some(title) = title {
                            Text::new(title).body_strong().color(color).wrap().show(ui);
                            ui.add_space(core::SPACE_1);
                        }
                        // Wrap on the available width: in a narrow panel the callout grows
                        // taller instead of running past its cell (and getting clipped).
                        Text::new(message).muted().wrap().show(ui);
                    });
                    // Trailing action, anchored to the right edge of the callout.
                    if let Some(action) = action {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if Button::new(action).ghost().sm().show(ui).clicked() {
                                action_clicked = true;
                            }
                        });
                    }
                });
            })
            .response;
        AlertOutput {
            response,
            action_clicked,
        }
    }
}
