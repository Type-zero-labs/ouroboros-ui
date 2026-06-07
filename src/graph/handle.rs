//! Handles (ports) — the connection anchors on a node's edges.
//!
//! Paint tier: draws the handle circle (token-colored) and owns the connect-drag detection.
//! Inputs anchor on a node's left edge, outputs on the right, distributed evenly down the side.
//! Variants beyond `Base` (labeled, button) land in a later task.

use egui::{Pos2, Rect};

use super::{PortId, PortSide};

/// Visual style of a handle.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HandleVariant {
    #[default]
    Base,
    /// A handle with a caption label beside it (inside the node).
    Labeled(&'static str),
}

/// Declares one port on a node: its id, which side it sits on, and whether it accepts drags.
#[derive(Clone, Copy, Debug)]
pub struct HandleSpec {
    pub id: PortId,
    pub side: PortSide,
    pub connectable: bool,
    pub variant: HandleVariant,
}

impl HandleSpec {
    /// An input port (left edge).
    pub fn input(id: u32) -> Self {
        Self {
            id: PortId(id),
            side: PortSide::In,
            connectable: true,
            variant: HandleVariant::Base,
        }
    }
    /// An output port (right edge).
    pub fn output(id: u32) -> Self {
        Self {
            id: PortId(id),
            side: PortSide::Out,
            connectable: true,
            variant: HandleVariant::Base,
        }
    }
    /// Mark the port non-connectable (decorative only).
    pub fn fixed(mut self) -> Self {
        self.connectable = false;
        self
    }
    /// Add a caption label beside the handle.
    pub fn label(mut self, text: &'static str) -> Self {
        self.variant = HandleVariant::Labeled(text);
        self
    }
}

/// Scene-space anchor for handle `index` of `count` on `side` of node `rect`.
pub(crate) fn anchor(rect: Rect, side: PortSide, index: usize, count: usize) -> Pos2 {
    let t = (index as f32 + 1.0) / (count as f32 + 1.0);
    let y = rect.top() + rect.height() * t;
    let x = match side {
        PortSide::In => rect.left(),
        PortSide::Out => rect.right(),
    };
    Pos2::new(x, y)
}
