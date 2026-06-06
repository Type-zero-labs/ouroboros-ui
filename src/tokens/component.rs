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

/// Absence of a fill/border (transparent — a structural value, not a design color).
const NONE: Color32 = Color32::TRANSPARENT;

/// Resolved paint values for a button variant — the worked example of the component layer.
/// Built from a [`Theme`]; a component never reads `core` colors directly. Transparent
/// `fill`/`border` mean "none". `underline` flags the Link variant's text.
#[derive(Clone, Copy, Debug)]
pub struct ButtonTokens {
    pub fill: Color32,
    pub foreground: Color32,
    pub border: Color32,
    pub radius: f32,
    pub underline: bool,
}

impl ButtonTokens {
    fn base(fill: Color32, foreground: Color32, border: Color32) -> Self {
        Self {
            fill,
            foreground,
            border,
            radius: core::RADIUS_MD,
            underline: false,
        }
    }

    /// Default — neutral near-white fill, dark text.
    pub fn primary(theme: &Theme) -> Self {
        Self::base(theme.primary, theme.primary_foreground, NONE)
    }
    /// Secondary — muted surface fill.
    pub fn secondary(theme: &Theme) -> Self {
        Self::base(theme.secondary, theme.secondary_foreground, NONE)
    }
    /// Destructive — red fill.
    pub fn destructive(theme: &Theme) -> Self {
        Self::base(theme.destructive, theme.destructive_foreground, NONE)
    }
    /// Outline — transparent fill, bordered.
    pub fn outline(theme: &Theme) -> Self {
        Self::base(NONE, theme.foreground, theme.border_strong)
    }
    /// Ghost — transparent until hover; no border.
    pub fn ghost(theme: &Theme) -> Self {
        Self::base(NONE, theme.foreground, NONE)
    }
    /// Link — underlined primary text, no fill/border.
    pub fn link(theme: &Theme) -> Self {
        let mut t = Self::base(NONE, theme.primary, NONE);
        t.underline = true;
        t
    }
}
