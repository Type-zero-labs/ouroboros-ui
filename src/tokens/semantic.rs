//! Semantic tokens — the shadcn vocabulary mapped onto [`core`](super::core).
//!
//! The [`Theme`] struct holds the resolved semantic tokens (`background`/`foreground`
//! pairs, `primary`, `muted`, `accent`, `destructive`, `border`, `ring`, …) plus the
//! domain status semantics (`success`/`warning`/`error`/`info`/`neutral`). Every field
//! references a `core::*` primitive — no raw colors live here.
//!
//! Layered dark zinc: `background` 950 → `card`/`popover` 900 → `muted`/`border` 800 →
//! `border_strong` 700. `primary` is neutral near-white (no brand accent).

use crate::tokens::core;
use egui::Color32;

/// Alpha applied to a status hue to derive its soft `*_bg` surface (~15%).
const STATUS_BG_ALPHA: u8 = 38;

/// Tint a hue down to a translucent background fill.
fn tint(c: Color32, a: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), a)
}

/// Resolved semantic theme — every color token the design system exposes.
///
/// Built per [`Mode`](crate::Mode) via `Theme::resolve` (T05); the dark table is
/// [`Theme::dark`]. Fields map 1:1 onto [`core`](super::core) primitives.
#[derive(Clone, Debug)]
pub struct Theme {
    // ── Surfaces ─────────────────────────────────────────────
    /// Deepest layer — panels, window fill.
    pub background: Color32,
    /// Primary text on `background`.
    pub foreground: Color32,
    /// Raised surface — cards, elevated panels.
    pub card: Color32,
    /// Text on `card`.
    pub card_foreground: Color32,
    /// Floating surface — popovers, menus, tooltips.
    pub popover: Color32,
    /// Text on `popover`.
    pub popover_foreground: Color32,
    /// Muted surface — inputs, secondary fills, chips.
    pub muted: Color32,
    /// Muted text — labels, descriptions, placeholders.
    pub muted_foreground: Color32,
    /// Disabled text.
    pub disabled_foreground: Color32,

    // ── Interactive ──────────────────────────────────────────
    /// Primary action fill — neutral near-white.
    pub primary: Color32,
    /// Text/icon on `primary` (dark, for contrast).
    pub primary_foreground: Color32,
    /// Hover/active fill for `primary` actions.
    pub primary_hover: Color32,
    /// Secondary action fill.
    pub secondary: Color32,
    /// Text on `secondary`.
    pub secondary_foreground: Color32,
    /// Hover/active surface (shadcn `accent` — not a brand color).
    pub accent: Color32,
    /// Text on `accent`.
    pub accent_foreground: Color32,
    /// Destructive action fill — red.
    pub destructive: Color32,
    /// Text on `destructive`.
    pub destructive_foreground: Color32,

    // ── Borders & focus ──────────────────────────────────────
    /// Default border / divider.
    pub border: Color32,
    /// Emphasized border.
    pub border_strong: Color32,
    /// Input border.
    pub input: Color32,
    /// Focus ring.
    pub ring: Color32,
    /// Contrast veil for hover state (light veil on dark mode, dark veil on light mode).
    pub hover_overlay: Color32,
    /// Stronger contrast veil for the pressed state.
    pub press_overlay: Color32,
    /// Backdrop veil behind modals and loading overlays (black in both modes).
    pub scrim: Color32,

    // ── Status (solid + soft bg) ─────────────────────────────
    pub success: Color32,
    pub success_bg: Color32,
    pub warning: Color32,
    pub warning_bg: Color32,
    pub error: Color32,
    pub error_bg: Color32,
    pub info: Color32,
    pub info_bg: Color32,
    pub neutral: Color32,
    pub neutral_bg: Color32,
}

impl Theme {
    /// The dark (zinc) palette — the populated mode.
    pub fn dark() -> Self {
        Self {
            // Surfaces — layered 950 / 900 / 800.
            background: core::ZINC_950,
            foreground: core::ZINC_50,
            card: core::ZINC_900,
            card_foreground: core::ZINC_50,
            popover: core::ZINC_900,
            popover_foreground: core::ZINC_50,
            muted: core::ZINC_800,
            muted_foreground: core::ZINC_400,
            disabled_foreground: core::ZINC_600,

            // Interactive — Ouroboros turquoise primary (lighter; dark text reads on it).
            primary: core::TEAL_200,
            primary_foreground: core::ZINC_950,
            primary_hover: core::TEAL_300,
            secondary: core::ZINC_800,
            secondary_foreground: core::ZINC_50,
            accent: core::ZINC_800,
            accent_foreground: core::ZINC_50,
            destructive: core::RED_500,
            destructive_foreground: core::ZINC_50,

            // Borders & focus.
            border: core::ZINC_800,
            border_strong: core::ZINC_700,
            input: core::ZINC_800,
            ring: core::TEAL_300,
            hover_overlay: Color32::from_white_alpha((core::HOVER_OVERLAY * 255.0) as u8),
            press_overlay: Color32::from_white_alpha((core::PRESS_OVERLAY * 255.0) as u8),
            scrim: core::SCRIM,

            // Status.
            success: core::GREEN_500,
            success_bg: tint(core::GREEN_500, STATUS_BG_ALPHA),
            warning: core::AMBER_500,
            warning_bg: tint(core::AMBER_500, STATUS_BG_ALPHA),
            error: core::RED_500,
            error_bg: tint(core::RED_500, STATUS_BG_ALPHA),
            info: core::BLUE_400,
            info_bg: tint(core::BLUE_500, STATUS_BG_ALPHA),
            neutral: core::ZINC_500,
            neutral_bg: tint(core::ZINC_500, STATUS_BG_ALPHA),
        }
    }

    /// The light (zinc) palette — off-white surfaces, dark text & primary.
    pub fn light() -> Self {
        Self {
            // Surfaces — off-white (zinc-50) / zinc-100.
            background: core::ZINC_50,
            foreground: core::ZINC_950,
            card: core::ZINC_50,
            card_foreground: core::ZINC_950,
            popover: core::ZINC_50,
            popover_foreground: core::ZINC_950,
            muted: core::ZINC_100,
            muted_foreground: core::ZINC_500,
            disabled_foreground: core::ZINC_400,

            // Interactive — Ouroboros turquoise primary (lighter; dark text on teal).
            primary: core::TEAL_400,
            primary_foreground: core::ZINC_950,
            primary_hover: core::TEAL_500,
            secondary: core::ZINC_100,
            secondary_foreground: core::ZINC_900,
            accent: core::ZINC_100,
            accent_foreground: core::ZINC_900,
            destructive: core::RED_500,
            destructive_foreground: core::ZINC_50,

            // Borders & focus.
            border: core::ZINC_200,
            border_strong: core::ZINC_300,
            input: core::ZINC_200,
            ring: core::TEAL_400,
            hover_overlay: Color32::from_black_alpha((core::HOVER_OVERLAY * 255.0) as u8),
            press_overlay: Color32::from_black_alpha((core::PRESS_OVERLAY * 255.0) as u8),
            scrim: core::SCRIM,

            // Status (same hues; soft bg tints).
            success: core::GREEN_500,
            success_bg: tint(core::GREEN_500, STATUS_BG_ALPHA),
            warning: core::AMBER_500,
            warning_bg: tint(core::AMBER_500, STATUS_BG_ALPHA),
            error: core::RED_500,
            error_bg: tint(core::RED_500, STATUS_BG_ALPHA),
            info: core::BLUE_400,
            info_bg: tint(core::BLUE_500, STATUS_BG_ALPHA),
            neutral: core::ZINC_500,
            neutral_bg: tint(core::ZINC_500, STATUS_BG_ALPHA),
        }
    }

    /// Dark with the original **neutral zinc** primary (no brand hue) — preserves the
    /// pre-Ouroboros look for anyone who wants it.
    pub fn zinc_dark() -> Self {
        let mut t = Self::dark();
        t.primary = core::ZINC_50;
        t.primary_foreground = core::ZINC_900;
        t.primary_hover = core::ZINC_200;
        t.ring = core::ZINC_300;
        t
    }
    /// Light with the original neutral zinc primary.
    pub fn zinc_light() -> Self {
        let mut t = Self::light();
        t.primary = core::ZINC_900;
        t.primary_foreground = core::ZINC_50;
        t.primary_hover = core::ZINC_700;
        t.ring = core::ZINC_400;
        t
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
