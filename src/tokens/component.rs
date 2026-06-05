//! Component tokens — thin per-component override structs.
//!
//! The third token layer. A component may expose a small struct of the exact values it
//! paints with, **derived from** the [`semantic`](super::semantic) layer (never from raw
//! [`core`](super::core)). This lets one component be retuned — a denser button, a louder
//! input focus — without touching global tokens, while keeping the
//! `core → semantic → component` dependency direction intact.
//!
//! Scaffolded here as a pattern; populated per-component as atoms arrive. [`ButtonTokens`]
//! is the worked example.

use crate::tokens::{core, semantic::Theme};
use egui::Color32;

/// Resolved paint values for a button variant — the worked example of the component
/// layer. Built from a [`Theme`]; a component never reads `core` directly.
#[derive(Clone, Debug)]
pub struct ButtonTokens {
    pub fill: Color32,
    pub foreground: Color32,
    pub border: Color32,
    pub radius: f32,
}

impl ButtonTokens {
    /// Primary variant — neutral near-white fill, dark text.
    pub fn primary(theme: &Theme) -> Self {
        Self {
            fill: theme.primary,
            foreground: theme.primary_foreground,
            border: theme.primary,
            radius: core::RADIUS_MD,
        }
    }

    /// Destructive variant — red fill.
    pub fn destructive(theme: &Theme) -> Self {
        Self {
            fill: theme.destructive,
            foreground: theme.destructive_foreground,
            border: theme.destructive,
            radius: core::RADIUS_MD,
        }
    }
}
