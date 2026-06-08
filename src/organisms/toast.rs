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
}

impl Toast {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            id: Id::new("toast"),
            message: message.into(),
            variant: AlertVariant::default(),
            dismissible: false,
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
