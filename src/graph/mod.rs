//! # graph — node-editor layer (the DS's reactflow)
//!
//! A peer layer beside `atoms`/`cells`/`molecules`/`organisms`, blueprinted on
//! [reactflow.dev/ui](https://reactflow.dev/ui) but built entirely from the design system's
//! tokens and atoms. It is the **one place outside `atoms` that paints** — a node graph
//! genuinely needs grid dots, bezier wires, handle circles and a marquee, none of which the
//! atom vocabulary covers — so the `no_painter_in_molecules` guard deliberately does not scan
//! it.
//!
//! ## The paint-but-token invariant
//!
//! `graph` may call the painter, but **every value still flows through a token**: colors come
//! from [`Theme`](crate::Theme) (resolved once into [`GraphTokens`]), sizes from
//! [`core`](crate::tokens::core). No raw `Color32::from_*`, no bare stroke/radius literals. The
//! `no_raw_values` guard is extended to scan `src/graph`, giving this layer the same purity
//! contract the atoms have.
//!
//! ## Two internal tiers
//!
//! - **paint tier** (`grid`, `edge`, `handle`, `resizer`): touch the painter, only
//!   via tokens.
//! - **compose tier** (`node`, `controls`, `minimap`, `toolbar`, `search`): reuse `Surface` and
//!   the atoms; never paint inline.
//!
//! ## Data-model-agnostic contract
//!
//! The caller owns the node/edge data (identity, world position, the edge list); the library
//! owns only **view state** (pan/zoom/selection/drag/connect) and reports back **intents**
//! (`node_moved`, `connection`, `delete_edge`, …) for the caller to commit. The library never
//! sees the caller's domain types.

pub mod canvas;
pub mod controls;
pub mod edge;
pub mod grid;
pub mod handle;
pub mod minimap;
pub mod node;
pub mod resizer;
pub mod search;
pub mod state;
pub mod tokens;
pub mod toolbar;

pub use canvas::{GraphCtx, GraphResponse, GraphView};
pub use edge::{EdgeResult, EdgeStyle};
pub use handle::{HandleSpec, HandleVariant};
pub use node::{NodeFrame, NodeResult, NodeStatus};
pub use search::NodeSearch;
pub use state::GraphViewState;
pub use tokens::GraphTokens;

// ─────────────────────────────────────────────────────────────────────────────
// Shared identity vocabulary. Ids are defined by the *caller* (stable across frames):
// the library hashes nothing on its own — it just carries these around and reports them
// back in [`GraphResponse`]. Kept here (not in `handle`) because `state`/`canvas` need them
// before handles exist.
// ─────────────────────────────────────────────────────────────────────────────

/// Stable identifier of a node, assigned by the caller (e.g. a hash of its domain key).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeId(pub u64);

/// Identifier of a port within a node's port list, assigned by the caller.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PortId(pub u32);

/// Identifier of a node *kind* offered by [`search`](crate::graph) — caller-defined.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeKindId(pub u64);

/// Which side of a node a port lives on. Inputs anchor on the left/top, outputs on the
/// right/bottom; a connection always runs `Out → In`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PortSide {
    In,
    Out,
}

/// A fully-qualified port: a node, a port within it, and the side it sits on.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Port {
    pub node: NodeId,
    pub port: PortId,
    pub side: PortSide,
}

/// A requested edge between two ports. Emitted by the library on a successful connect-drag;
/// the caller commits it to its own edge list.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Connection {
    pub from: Port,
    pub to: Port,
}
