//! EmptyState molecule — centered "nothing here yet" placeholder with CTA.
//! [shadcn Empty]
//!
//! Criado na spec `scene-hub-shell`: telas de lista vazia do studio (cenas,
//! painéis, assets) precisam de um placeholder guiado com call-to-action.
//! `use` falha (só `Table::empty_text`, texto cru) e `extend` falha (nenhum
//! componente próximo) — nasce como molecule: grupo funcional de atoms
//! (Icon + Heading + Text + Buttons), sem pintar. Layout segue o empty state
//! do launcher do studio (coluna centrada, CTAs empilhados).

use crate::atoms::{Button, Heading, Icon, Text};
use crate::tokens::core;
use egui::{Align, Layout, Response, Ui};

/// Centered placeholder for empty content areas: icon + title + optional
/// description + up to two stacked CTAs (primary + secondary).
///
/// Renders as a horizontally-centered column over the available width,
/// pushed down by a third of the available height (optically centered).
pub struct EmptyState {
    icon: Option<&'static str>,
    title: String,
    description: Option<String>,
    primary: Option<String>,
    primary_icon: Option<&'static str>,
    secondary: Option<String>,
    secondary_icon: Option<&'static str>,
}

/// Output of [`EmptyState::show`]: the container response plus which CTA (if
/// any) was clicked this frame.
pub struct EmptyStateOutput {
    pub response: Response,
    pub primary_clicked: bool,
    pub secondary_clicked: bool,
}

impl EmptyState {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            icon: None,
            title: title.into(),
            description: None,
            primary: None,
            primary_icon: None,
            secondary: None,
            secondary_icon: None,
        }
    }

    /// Leading glyph rendered muted above the title (e.g. `light::STACK`).
    pub fn icon(mut self, glyph: &'static str) -> Self {
        self.icon = Some(glyph);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Primary call-to-action button (default variant).
    pub fn primary_action(mut self, label: impl Into<String>) -> Self {
        self.primary = Some(label.into());
        self
    }

    /// Leading glyph for the primary CTA (e.g. `light::PLUS`).
    pub fn primary_icon(mut self, glyph: &'static str) -> Self {
        self.primary_icon = Some(glyph);
        self
    }

    /// Secondary call-to-action button (secondary variant), below the primary.
    pub fn secondary_action(mut self, label: impl Into<String>) -> Self {
        self.secondary = Some(label.into());
        self
    }

    /// Leading glyph for the secondary CTA.
    pub fn secondary_icon(mut self, glyph: &'static str) -> Self {
        self.secondary_icon = Some(glyph);
        self
    }

    pub fn show(self, ui: &mut Ui) -> EmptyStateOutput {
        let Self {
            icon,
            title,
            description,
            primary,
            primary_icon,
            secondary,
            secondary_icon,
        } = self;
        let mut primary_clicked = false;
        let mut secondary_clicked = false;

        let response = ui
            .with_layout(Layout::top_down(Align::Center), |ui| {
                // Centro óptico: empurra o bloco pra ~1/3 da área disponível
                // (proporcional — funciona de painel estreito a tela cheia).
                ui.add_space(ui.available_height() / 3.0);
                if let Some(glyph) = icon {
                    Icon::new(glyph).xl().muted().show(ui);
                    ui.add_space(core::SPACE_3);
                }
                Heading::new(title).h2().show(ui);
                if let Some(description) = description {
                    ui.add_space(core::SPACE_2);
                    Text::new(description).muted().wrap().show(ui);
                }
                if let Some(label) = primary {
                    ui.add_space(core::SPACE_5);
                    let mut button = Button::new(label);
                    if let Some(glyph) = primary_icon {
                        button = button.icon_left(glyph);
                    }
                    if button.show(ui).clicked() {
                        primary_clicked = true;
                    }
                }
                if let Some(label) = secondary {
                    ui.add_space(core::SPACE_3);
                    let mut button = Button::new(label).secondary();
                    if let Some(glyph) = secondary_icon {
                        button = button.icon_left(glyph);
                    }
                    if button.show(ui).clicked() {
                        secondary_clicked = true;
                    }
                }
            })
            .response;

        EmptyStateOutput {
            response,
            primary_clicked,
            secondary_clicked,
        }
    }
}
