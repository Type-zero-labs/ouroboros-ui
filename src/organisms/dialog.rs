//! Dialog organism — a modal with title/description + body. [shadcn Dialog / Unity Overlay]
//!
//! Uses [`egui::Modal`] (scrim + centered) whose frame inherits the themed window visuals.
//! Content gets a generous inner padding ([`core::SPACE_6`]). [`Dialog::confirm`] is the
//! ready-made confirm/cancel variant (replaces the legacy `prompt`).

use crate::atoms::{Button, Heading, Text};
use crate::tokens::{core, layout};
use egui::{Context, Frame, Id, Ui};

/// The outcome of a [`Dialog::confirm`] modal for this frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DialogChoice {
    /// No button pressed this frame (modal still open).
    #[default]
    None,
    /// The confirm button was pressed.
    Confirm,
    /// The cancel button was pressed (or the modal was dismissed via backdrop/Esc).
    Cancel,
}

/// A modal dialog. Render only while open; `show` returns `true` when it should close
/// (backdrop click / Esc).
pub struct Dialog {
    id: Id,
    title: String,
    description: Option<String>,
    destructive: bool,
}

impl Dialog {
    pub fn new(title: impl Into<String>) -> Self {
        let title = title.into();
        Self {
            id: Id::new(format!("dialog::{title}")),
            title,
            description: None,
            destructive: false,
        }
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id = Id::new(id);
        self
    }
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    /// Style the [`confirm`](Self::confirm) action as destructive (red) — e.g. a delete prompt.
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }

    /// Generous padding wrapper shared by `show` and `confirm`.
    fn header(ui: &mut Ui, title: String, description: Option<String>) {
        Heading::new(title).h2().show(ui);
        if let Some(description) = description {
            ui.add_space(core::SPACE_1);
            Text::new(description).muted().show(ui);
        }
    }

    pub fn show(self, ctx: &Context, body: impl FnOnce(&mut Ui)) -> bool {
        let title = self.title;
        let description = self.description;
        egui::Modal::new(self.id)
            .show(ctx, |ui| {
                Frame::default().inner_margin(core::SPACE_6).show(ui, |ui| {
                    ui.set_max_width(layout::PANEL_MAX);
                    Self::header(ui, title, description);
                    ui.add_space(core::SPACE_4);
                    body(ui);
                });
            })
            .should_close()
    }

    /// Confirm/cancel variant. Renders the title/description plus a right-aligned
    /// `cancel` · `confirm` button row; returns the user's [`DialogChoice`] this frame.
    /// Backdrop click / Esc resolves to [`DialogChoice::Cancel`].
    pub fn confirm(
        self,
        ctx: &Context,
        confirm_label: impl Into<String>,
        cancel_label: impl Into<String>,
    ) -> DialogChoice {
        let title = self.title;
        let description = self.description;
        let destructive = self.destructive;
        let confirm_label = confirm_label.into();
        let cancel_label = cancel_label.into();
        let id = self.id;
        let modal = egui::Modal::new(id).show(ctx, |ui| {
            Frame::default()
                .inner_margin(core::SPACE_6)
                .show(ui, |ui| {
                    ui.set_max_width(layout::PANEL_MAX);
                    Self::header(ui, title, description);
                    ui.add_space(core::SPACE_5);
                    let mut choice = DialogChoice::None;
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let mut confirm = Button::new(confirm_label).id_source((id, "confirm"));
                        if destructive {
                            confirm = confirm.destructive();
                        }
                        if confirm.show(ui).clicked() {
                            choice = DialogChoice::Confirm;
                        }
                        ui.add_space(core::SPACE_2);
                        if Button::new(cancel_label)
                            .secondary()
                            .id_source((id, "cancel"))
                            .show(ui)
                            .clicked()
                        {
                            choice = DialogChoice::Cancel;
                        }
                    });
                    choice
                })
                .inner
        });
        if modal.should_close() {
            return DialogChoice::Cancel;
        }
        modal.inner
    }
}
