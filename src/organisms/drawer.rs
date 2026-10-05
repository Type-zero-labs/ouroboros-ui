//! Drawer organism — a resizable sheet over the bottom of a canvas. [Figma/VS Code panel]
//!
//! Holds the console: opened from the rail, it covers the bottom of the canvas instead of
//! taking a permanent band. The top edge is a grip (height clamped to 120 px…60 % of the
//! canvas); `Esc` closes it. The owner keeps `open` and `height` (persist them).

use crate::atoms::{Axis, SplitterHandle, Surface};
use crate::tokens::core;
use egui::{pos2, Area, Context, Id, Key, Order, Rect, Ui, UiBuilder};

/// The drawer. Builder; `show` draws it over the bottom of `canvas` when `*open`.
pub struct Drawer<'a> {
    id: Id,
    open: &'a mut bool,
    height: &'a mut f32,
}

impl<'a> Drawer<'a> {
    pub fn new(id: impl std::hash::Hash, open: &'a mut bool, height: &'a mut f32) -> Self {
        Self {
            id: Id::new(id),
            open,
            height,
        }
    }

    /// Draw (if open). Returns `true` when the height changed (persist it).
    pub fn show(self, ctx: &Context, canvas: Rect, body: impl FnOnce(&mut Ui)) -> bool {
        if !*self.open {
            return false;
        }
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            *self.open = false;
            return false;
        }
        let max_h = (canvas.height() * 0.6).max(120.0);
        *self.height = self.height.clamp(120.0, max_h);
        let rect = Rect::from_min_max(
            pos2(canvas.left(), canvas.bottom() - *self.height),
            canvas.max,
        );
        let mut resized = false;
        let height = self.height;
        Area::new(self.id)
            .order(Order::Foreground)
            .fixed_pos(rect.min)
            .show(ctx, |ui| {
                ui.set_min_size(rect.size());
                ui.set_max_size(rect.size());
                Surface::new()
                    .elevated()
                    .radius(0.0)
                    .pad(0.0)
                    .show(ui, |ui| {
                        ui.set_min_size(rect.size());
                        let grip = Rect::from_min_max(
                            rect.min,
                            pos2(rect.right(), rect.top() + core::SPACE_1),
                        );
                        let mut gui = ui.new_child(UiBuilder::new().max_rect(grip));
                        let g = SplitterHandle::new(Axis::Horizontal).show(&mut gui);
                        if g.dragged() {
                            *height = (*height - g.drag_delta().y).clamp(120.0, max_h);
                            resized = true;
                        }
                        let mut b = ui.new_child(
                            UiBuilder::new().max_rect(
                                Rect::from_min_max(pos2(rect.left(), grip.bottom()), rect.max)
                                    .shrink(core::SPACE_2),
                            ),
                        );
                        body(&mut b);
                    });
            });
        resized
    }
}
