//! FloatingToolbar organism — a pill of tools floating over a canvas. [Figma bottom toolbar]
//!
//! Context tools live on the canvas instead of in fixed bars: the World subtabs, "Add node",
//! Fit/Play. An elevated, rounded `card` strip `FLOATING_TOOLBAR_H` tall, anchored inside the
//! canvas rect (top-center by default). The caller fills it with toolbar buttons/toggles and
//! [`toolbar_separator`].

use crate::atoms::{Divider, Surface};
use crate::tokens::{core, layout};
use egui::{Align2, Area, Context, Id, Order, Rect, Ui, Vec2};

/// The floating toolbar. Builder; `show` draws it over `canvas`.
pub struct FloatingToolbar {
    id: Id,
    anchor: Align2,
    offset: Vec2,
}

impl FloatingToolbar {
    pub fn new(id: impl std::hash::Hash) -> Self {
        Self {
            id: Id::new(id),
            anchor: Align2::CENTER_TOP,
            offset: Vec2::new(0.0, core::SPACE_3),
        }
    }
    /// Where inside the canvas (default top-center, 12 px in).
    pub fn anchor(mut self, anchor: Align2, offset: Vec2) -> Self {
        self.anchor = anchor;
        self.offset = offset;
        self
    }

    pub fn show<R>(self, ctx: &Context, canvas: Rect, add: impl FnOnce(&mut Ui) -> R) -> R {
        let pos = self.anchor.pos_in_rect(&canvas.shrink2(self.offset.abs()));
        Area::new(self.id)
            .order(Order::Foreground)
            .pivot(self.anchor)
            .fixed_pos(pos)
            .constrain_to(canvas)
            .show(ctx, |ui| {
                Surface::new()
                    .elevated()
                    .radius(core::RADIUS_LG)
                    .pad(core::SPACE_1)
                    .show(ui, |ui| {
                        ui.set_height(layout::FLOATING_TOOLBAR_H - core::SPACE_2);
                        ui.horizontal_centered(|ui| {
                            ui.spacing_mut().item_spacing.x = core::SPACE_1;
                            add(ui)
                        })
                        .inner
                    })
                    .inner
            })
            .inner
    }
}

/// A thin vertical rule between toolbar groups.
pub fn toolbar_separator(ui: &mut Ui) {
    Divider::vertical().show(ui);
}
