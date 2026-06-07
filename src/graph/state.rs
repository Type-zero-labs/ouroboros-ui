//! View state — the only thing the library owns across frames.
//!
//! Stored in egui memory keyed by the [`GraphView`](super::GraphView)'s id. The caller owns
//! the node/edge *data*; this struct owns the *view* (where the camera is, what's selected,
//! what's mid-drag). It must be `Clone + Default` to live in egui's temp store.

use std::collections::HashSet;

use egui::{Pos2, Rect, Vec2};

use super::{NodeId, Port};

/// A node-move drag in progress: which node grabbed it and the accumulated world delta. The
/// accumulator pattern (recompute from origin each frame) avoids drift on slow drags.
#[derive(Clone, Copy, Debug)]
pub struct NodeDrag {
    pub node: NodeId,
    pub accum_world: Vec2,
}

/// A connect drag in progress: dragged out from `from` (anchored at `from_world`), the wire
/// trails to the cursor (world).
#[derive(Clone, Copy, Debug)]
pub struct ConnectDrag {
    pub from: Port,
    pub from_world: Pos2,
    pub cursor_world: Pos2,
}

/// A box-select drag in progress, in world coordinates (so it tracks under pan/zoom).
#[derive(Clone, Copy, Debug)]
pub struct MarqueeDrag {
    pub start_world: Pos2,
    pub cursor_world: Pos2,
    /// Shift held at drag start → additive selection.
    pub additive: bool,
}

/// Everything the graph view remembers between frames.
///
/// `scene_rect` is the [`egui::Scene`] view window in world coordinates — the library hands a
/// `&mut` to it each frame and Scene mutates it on pan/zoom. `Rect::ZERO` is the "uninitialised"
/// sentinel: Scene auto-fits the content on the first frame.
#[derive(Clone, Debug)]
pub struct GraphViewState {
    pub scene_rect: Rect,
    pub selection: HashSet<NodeId>,
    pub edge_selection: Option<(Port, Port)>,
    pub hovered_node: Option<NodeId>,
    pub drag: Option<NodeDrag>,
    pub connect: Option<ConnectDrag>,
    pub marquee: Option<MarqueeDrag>,
}

impl Default for GraphViewState {
    fn default() -> Self {
        Self {
            scene_rect: Rect::ZERO,
            selection: HashSet::new(),
            edge_selection: None,
            hovered_node: None,
            drag: None,
            connect: None,
            marquee: None,
        }
    }
}
