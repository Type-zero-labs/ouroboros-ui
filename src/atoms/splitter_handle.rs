//! SplitterHandle atom — the draggable divider band between two Splitter panels.
//!
//! Fills its area as a drag hit-target and paints a centered hairline (token `border`) that
//! fades to `ring` on hover/drag, setting the resize cursor. The owning Splitter organism
//! reads the returned [`Response`] (drag delta, double-click). Atoms may paint; the organism
//! only composes this.

use crate::atoms::Axis;
use crate::tokens::core;
use crate::Theme;
use egui::{CursorIcon, Response, Sense, Stroke, Ui};

/// A splitter divider. `line` is the orientation of the visible rule — `Vertical` for a
/// left/right (horizontal) split, `Horizontal` for a top/bottom (vertical) split.
pub struct SplitterHandle {
    line: Axis,
    active: bool,
}

impl SplitterHandle {
    pub fn new(line: Axis) -> Self {
        Self {
            line,
            active: false,
        }
    }
    /// Force the highlighted state — e.g. while dragging, or when a neighbor is collapsed.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let rect = ui.max_rect();
        let resp = ui.allocate_rect(rect, Sense::click_and_drag());

        let interacting = resp.hovered() || resp.dragged();
        if interacting {
            ui.ctx().set_cursor_icon(match self.line {
                Axis::Vertical => CursorIcon::ResizeHorizontal,
                Axis::Horizontal => CursorIcon::ResizeVertical,
            });
        }
        let t = core::hover_t(ui.ctx(), resp.id, self.active || interacting);

        let painter = ui.painter();
        match self.line {
            Axis::Vertical => {
                let x = rect.center().x;
                painter.vline(
                    x,
                    rect.y_range(),
                    Stroke::new(core::BORDER_THIN, theme.border),
                );
                if t > 0.0 {
                    painter.vline(
                        x,
                        rect.y_range(),
                        Stroke::new(core::BORDER_FOCUS, theme.ring.gamma_multiply(t)),
                    );
                }
            }
            Axis::Horizontal => {
                let y = rect.center().y;
                painter.hline(
                    rect.x_range(),
                    y,
                    Stroke::new(core::BORDER_THIN, theme.border),
                );
                if t > 0.0 {
                    painter.hline(
                        rect.x_range(),
                        y,
                        Stroke::new(core::BORDER_FOCUS, theme.ring.gamma_multiply(t)),
                    );
                }
            }
        }
        resp
    }
}
