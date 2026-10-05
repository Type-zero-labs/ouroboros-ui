//! WindowResizeBorder atom — invisible grips on the edges and corners of an undecorated window.
//!
//! The shell draws without OS decorations (Figma-like), so it owns resizing: this atom senses a
//! drag starting on a 4 px band along each edge/corner, sets the resize cursor on hover, and
//! returns which edge was grabbed. The caller hands that to the windowing layer
//! (`winit::Window::drag_resize_window`). Paints nothing.

use crate::tokens::core;
use egui::{pos2, CursorIcon, Id, Rect, Sense, Ui};

/// Which edge or corner of the window was grabbed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizeEdge {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl ResizeEdge {
    fn cursor(self) -> CursorIcon {
        match self {
            ResizeEdge::North | ResizeEdge::South => CursorIcon::ResizeVertical,
            ResizeEdge::East | ResizeEdge::West => CursorIcon::ResizeHorizontal,
            ResizeEdge::NorthEast | ResizeEdge::SouthWest => CursorIcon::ResizeNeSw,
            ResizeEdge::NorthWest | ResizeEdge::SouthEast => CursorIcon::ResizeNwSe,
        }
    }
}

/// Resize grips around `rect` (normally the whole window). `show` returns the edge whose drag
/// started this frame.
pub struct WindowResizeBorder {
    thickness: f32,
}

impl Default for WindowResizeBorder {
    fn default() -> Self {
        Self {
            thickness: core::SPACE_1,
        }
    }
}

impl WindowResizeBorder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(self, ui: &mut Ui, rect: Rect) -> Option<ResizeEdge> {
        let t = self.thickness;
        let c = t * 3.0; // corners are easier to grab than edges
        let (l, r, top, b) = (rect.left(), rect.right(), rect.top(), rect.bottom());
        let zones = [
            (
                ResizeEdge::NorthWest,
                Rect::from_min_max(pos2(l, top), pos2(l + c, top + c)),
            ),
            (
                ResizeEdge::NorthEast,
                Rect::from_min_max(pos2(r - c, top), pos2(r, top + c)),
            ),
            (
                ResizeEdge::SouthWest,
                Rect::from_min_max(pos2(l, b - c), pos2(l + c, b)),
            ),
            (
                ResizeEdge::SouthEast,
                Rect::from_min_max(pos2(r - c, b - c), pos2(r, b)),
            ),
            (
                ResizeEdge::North,
                Rect::from_min_max(pos2(l + c, top), pos2(r - c, top + t)),
            ),
            (
                ResizeEdge::South,
                Rect::from_min_max(pos2(l + c, b - t), pos2(r - c, b)),
            ),
            (
                ResizeEdge::West,
                Rect::from_min_max(pos2(l, top + c), pos2(l + t, b - c)),
            ),
            (
                ResizeEdge::East,
                Rect::from_min_max(pos2(r - t, top + c), pos2(r, b - c)),
            ),
        ];
        let mut grabbed = None;
        for (edge, zone) in zones {
            let resp = ui.interact(zone, Id::new(("window_resize", edge as u8)), Sense::drag());
            if resp.hovered() || resp.dragged() {
                ui.ctx().set_cursor_icon(edge.cursor());
            }
            if resp.drag_started() {
                grabbed = Some(edge);
            }
        }
        grabbed
    }
}
