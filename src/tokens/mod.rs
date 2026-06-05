//! Layered design tokens: `core` (primitives) → `semantic` (shadcn tokens) →
//! `component` (per-component overrides). See the crate-level docs for the model.

pub mod component;
pub mod core;
pub mod layout;
pub mod semantic;
