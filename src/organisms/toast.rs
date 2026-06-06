//! Toast organism — a transient notification anchored top-right. [shadcn Sonner / Unity notifications]
//!
//! Composes [`Alert`] inside a foreground [`egui::Area`]. The consumer owns visibility/timing.

use crate::molecules::{Alert, AlertVariant};
use crate::tokens::{core, layout};
use egui::{Align2, Area, Context, Id, Order, Vec2};

/// A toast notification. `show` places it top-right.
pub struct Toast {
    id: Id,
    message: String,
    variant: AlertVariant,
}

impl Toast {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            id: Id::new("toast"),
            message: message.into(),
            variant: AlertVariant::default(),
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

    pub fn show(self, ctx: &Context) {
        let message = self.message;
        let variant = self.variant;
        Area::new(self.id)
            .anchor(Align2::RIGHT_TOP, Vec2::new(-core::SPACE_4, core::SPACE_4))
            .order(Order::Foreground)
            .show(ctx, |ui| {
                ui.set_max_width(layout::INSPECTOR_WIDTH);
                Alert::new(message).variant(variant).show(ui);
            });
    }
}
