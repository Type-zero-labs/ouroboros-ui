//! Toast organism — a transient notification anchored top-right. [shadcn Sonner / Unity notifications]
//!
//! Composes [`Alert`] inside a foreground [`egui::Area`]. The consumer owns visibility/timing.

use crate::atoms::Button;
use crate::molecules::{Alert, AlertVariant};
use crate::tokens::{core, layout};
use egui::{pos2, vec2, Align2, Area, Context, Id, Order, Rect, UiBuilder, Vec2};
use egui_phosphor::light;

/// A toast notification. `show` places it top-right and returns whether the close button
/// was clicked (only meaningful with [`Toast::dismissible`]).
pub struct Toast {
    id: Id,
    message: String,
    variant: AlertVariant,
    dismissible: bool,
    actions: Vec<String>,
    busy: bool,
    bottom: bool,
}

/// What the user did on a toast with actions ([`Toast::show_with_actions`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ToastResult {
    pub dismissed: bool,
    /// Index of the action button clicked.
    pub action: Option<usize>,
}

impl Toast {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            id: Id::new("toast"),
            message: message.into(),
            variant: AlertVariant::default(),
            dismissible: false,
            actions: Vec::new(),
            busy: false,
            bottom: false,
        }
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id = Id::new(id);
        self
    }
    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
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
    /// Add a close (✕) button in the top-right corner. `show` then returns `true` on click.
    pub fn dismissible(mut self) -> Self {
        self.dismissible = true;
        self
    }

    /// An action button under the message (e.g. "Reveal", "Stop").
    pub fn action(mut self, label: impl Into<String>) -> Self {
        self.actions.push(label.into());
        self
    }
    /// A spinner before the message (work in progress: export, preview build).
    pub fn busy(mut self, busy: bool) -> Self {
        self.busy = busy;
        self
    }
    /// Anchor bottom-right instead of top-right (the shell keeps the top for the canvas).
    pub fn bottom(mut self) -> Self {
        self.bottom = true;
        self
    }

    /// Like [`Toast::show`], with the action row; reports which action was clicked.
    pub fn show_with_actions(self, ctx: &Context) -> ToastResult {
        let actions = self.actions.clone();
        let busy = self.busy;
        let id = self.id;
        let mut result = ToastResult::default();
        let message = self.message.clone();
        let anchor = if self.bottom {
            (
                Align2::RIGHT_BOTTOM,
                Vec2::new(-core::SPACE_4, -core::SPACE_4),
            )
        } else {
            (Align2::RIGHT_TOP, Vec2::new(-core::SPACE_4, core::SPACE_4))
        };
        let variant = self.variant;
        let dismissible = self.dismissible;
        Area::new(id)
            .anchor(anchor.0, anchor.1)
            .order(Order::Foreground)
            .show(ctx, |ui| {
                ui.set_max_width(layout::INSPECTOR_WIDTH);
                crate::atoms::Surface::new()
                    .elevated()
                    .pad(core::SPACE_2)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            if busy {
                                crate::atoms::Spinner::new().show(ui);
                            }
                            let _ = variant;
                            crate::atoms::Text::new(message).wrap().show(ui);
                            if dismissible {
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Min),
                                    |ui| {
                                        if Button::new("")
                                            .icon_only()
                                            .ghost()
                                            .sm()
                                            .icon_left(light::X)
                                            .id_source((id, "toast_close"))
                                            .show(ui)
                                            .clicked()
                                        {
                                            result.dismissed = true;
                                        }
                                    },
                                );
                            }
                        });
                        if !actions.is_empty() {
                            ui.horizontal(|ui| {
                                for (i, label) in actions.iter().enumerate() {
                                    if Button::new(label.clone())
                                        .secondary()
                                        .sm()
                                        .id_source((id, "toast_action", i))
                                        .show(ui)
                                        .clicked()
                                    {
                                        result.action = Some(i);
                                    }
                                }
                            });
                        }
                    });
            });
        result
    }

    pub fn show(self, ctx: &Context) -> bool {
        let message = self.message;
        let variant = self.variant;
        let dismissible = self.dismissible;
        let id = self.id;
        Area::new(id)
            .anchor(Align2::RIGHT_TOP, Vec2::new(-core::SPACE_4, core::SPACE_4))
            .order(Order::Foreground)
            .show(ctx, |ui| {
                ui.set_max_width(layout::INSPECTOR_WIDTH);
                let alert = Alert::new(message).variant(variant).show(ui);
                let mut dismissed = false;
                if dismissible {
                    let s = core::ICON_LG;
                    let x_rect = Rect::from_min_size(
                        pos2(
                            alert.rect.right() - s - core::SPACE_2,
                            alert.rect.top() + core::SPACE_2,
                        ),
                        vec2(s, s),
                    );
                    let mut cui = ui.new_child(UiBuilder::new().max_rect(x_rect));
                    if Button::new("")
                        .icon_only()
                        .ghost()
                        .sm()
                        .icon_left(light::X)
                        .id_source((id, "toast_close"))
                        .show(&mut cui)
                        .clicked()
                    {
                        dismissed = true;
                    }
                }
                dismissed
            })
            .inner
    }
}
