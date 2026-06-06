//! Core primitives — raw values with no semantic meaning.
//!
//! Color ramps, spacing scale, radius scale, shadow primitives, and type sizes.
//! Everything here is a `const`; the semantic layer maps meaning onto these.
//!
//! Decided interactively (value-by-value against shadcn). Sections fill in as the
//! token groups are confirmed.

use egui::{epaint::Shadow, Color32};

// ─────────────────────────────────────────────────────────────────────────────
// Neutral base ramp — Zinc (cool-neutral). The temperature of every gray surface,
// border and text token. Tailwind/shadcn `zinc` 50→950. The semantic layer picks
// which steps mean background/card/border/foreground/etc.
// ─────────────────────────────────────────────────────────────────────────────

pub const ZINC_50: Color32 = Color32::from_rgb(250, 250, 250);
pub const ZINC_100: Color32 = Color32::from_rgb(244, 244, 245);
pub const ZINC_200: Color32 = Color32::from_rgb(228, 228, 231);
pub const ZINC_300: Color32 = Color32::from_rgb(212, 212, 216);
pub const ZINC_400: Color32 = Color32::from_rgb(161, 161, 170);
pub const ZINC_500: Color32 = Color32::from_rgb(113, 113, 122);
pub const ZINC_600: Color32 = Color32::from_rgb(82, 82, 91);
pub const ZINC_700: Color32 = Color32::from_rgb(63, 63, 70);
pub const ZINC_800: Color32 = Color32::from_rgb(39, 39, 42);
pub const ZINC_900: Color32 = Color32::from_rgb(24, 24, 27);
pub const ZINC_950: Color32 = Color32::from_rgb(9, 9, 11);

// ─────────────────────────────────────────────────────────────────────────────
// Status hues — Tailwind 500 (info uses blue-400 for text legibility on dark).
// The semantic layer composites the `*_bg` variants by applying low alpha to these.
// ─────────────────────────────────────────────────────────────────────────────

/// Success — green-500.
pub const GREEN_500: Color32 = Color32::from_rgb(34, 197, 94);
/// Error / destructive — red-500.
pub const RED_500: Color32 = Color32::from_rgb(239, 68, 68);
/// Warning — amber-500.
pub const AMBER_500: Color32 = Color32::from_rgb(245, 158, 11);
/// Info (text) — blue-400.
pub const BLUE_400: Color32 = Color32::from_rgb(96, 165, 250);
/// Info (fill base) — blue-500.
pub const BLUE_500: Color32 = Color32::from_rgb(59, 130, 246);

// ─────────────────────────────────────────────────────────────────────────────
// Spacing scale — 4px base, Tailwind numeric keys (key N = N×4px). Contiguous
// 1–6 then 8/10/12 for the larger gaps. Used for padding, gaps, margins.
// ─────────────────────────────────────────────────────────────────────────────

pub const SPACE_1: f32 = 4.0;
pub const SPACE_2: f32 = 8.0;
pub const SPACE_3: f32 = 12.0;
pub const SPACE_4: f32 = 16.0;
pub const SPACE_5: f32 = 20.0;
pub const SPACE_6: f32 = 24.0;
pub const SPACE_8: f32 = 32.0;
pub const SPACE_10: f32 = 40.0;
pub const SPACE_12: f32 = 48.0;

// ─────────────────────────────────────────────────────────────────────────────
// Corner radius — shadcn classic base (0.5rem). FULL is the pill/circle sentinel.
// ─────────────────────────────────────────────────────────────────────────────

pub const RADIUS_SM: f32 = 4.0;
pub const RADIUS_MD: f32 = 6.0;
pub const RADIUS_LG: f32 = 8.0;
pub const RADIUS_XL: f32 = 12.0;
pub const RADIUS_FULL: f32 = 9999.0;

// ─────────────────────────────────────────────────────────────────────────────
// Shadows — dark-tuned (high alpha to read on the zinc background). sm=chips,
// md=cards/popovers, lg=modals/overlays.
// ─────────────────────────────────────────────────────────────────────────────

/// Subtle lift — fields, chips.
pub const SHADOW_SM: Shadow = Shadow {
    offset: [0, 1],
    blur: 2,
    spread: 0,
    color: Color32::from_rgba_premultiplied(0, 0, 0, 61),
};
/// Cards, pills, popovers.
pub const SHADOW_MD: Shadow = Shadow {
    offset: [0, 2],
    blur: 4,
    spread: 0,
    color: Color32::from_rgba_premultiplied(0, 0, 0, 82),
};
/// Modals, overlays.
pub const SHADOW_LG: Shadow = Shadow {
    offset: [0, 8],
    blur: 24,
    spread: 0,
    color: Color32::from_rgba_premultiplied(0, 0, 0, 48),
};

// ─────────────────────────────────────────────────────────────────────────────
// Typography — raw primitives only. The theme/typography layer composes these into
// named styles (Display/H1/Body/Code…) over the registered Iosevka faces.
//
// Type size scale (px). Dense IDE calibration; body anchors at TEXT_BASE (14).
// ─────────────────────────────────────────────────────────────────────────────

pub const TEXT_XS: f32 = 12.0;
pub const TEXT_SM: f32 = 13.0;
pub const TEXT_BASE: f32 = 14.0;
pub const TEXT_LG: f32 = 16.0;
pub const TEXT_XL: f32 = 20.0;
pub const TEXT_2XL: f32 = 24.0;
pub const TEXT_3XL: f32 = 30.0;

/// Line-height multipliers (× font size). Tight = headings/display; normal = body;
/// relaxed = long-form blocks.
pub const LEADING_TIGHT: f32 = 1.2;
pub const LEADING_NORMAL: f32 = 1.45;
pub const LEADING_RELAXED: f32 = 1.6;

/// Letter-spacing (px, extra per glyph). Iosevka is monospace so default is 0;
/// large display sizes tighten slightly.
pub const TRACKING_TIGHT: f32 = -0.25;
pub const TRACKING_NORMAL: f32 = 0.0;

// ─────────────────────────────────────────────────────────────────────────────
// Sizing — control heights, icon sizes, stroke widths, minimum hit target (px).
// Dense IDE calibration.
// ─────────────────────────────────────────────────────────────────────────────

/// Control heights (button / input / select).
pub const CONTROL_SM: f32 = 26.0;
pub const CONTROL_MD: f32 = 32.0;
pub const CONTROL_LG: f32 = 38.0;

/// Icon box sizes.
pub const ICON_SM: f32 = 14.0;
pub const ICON_MD: f32 = 16.0;
pub const ICON_LG: f32 = 20.0;
pub const ICON_XL: f32 = 24.0;

/// Default border / divider stroke.
pub const BORDER_THIN: f32 = 1.0;
/// Focus-ring stroke.
pub const BORDER_FOCUS: f32 = 2.0;
/// Gap between a widget's edge and its focus ring.
pub const RING_OFFSET: f32 = 2.0;
/// Minimum interactive target.
pub const HIT_MIN: f32 = 32.0;

// ─────────────────────────────────────────────────────────────────────────────
// Motion — animation durations (seconds) + easing curves. egui drives hover/focus
// transitions by duration (`ctx.animate_*`); [`Easing`] shapes the progress.
// ─────────────────────────────────────────────────────────────────────────────

pub const DURATION_INSTANT: f32 = 0.0;
pub const DURATION_FAST: f32 = 0.10;
pub const DURATION_NORMAL: f32 = 0.18;
pub const DURATION_SLOW: f32 = 0.30;

/// Easing curve applied to a normalized progress `t` in `0..=1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Easing {
    Linear,
    /// Decelerate — natural for enter/hover. The default.
    #[default]
    EaseOut,
    /// Accelerate then decelerate — for moves/reorders.
    EaseInOut,
}

impl Easing {
    /// Map progress through the curve.
    pub fn apply(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Opacity & overlays — standard state alphas. `*_OVERLAY` are white veils applied
// over a surface for hover/press; `SCRIM` dims the backdrop behind modals.
// ─────────────────────────────────────────────────────────────────────────────

pub const OPACITY_DISABLED: f32 = 0.5;
pub const OPACITY_MUTED: f32 = 0.7;
pub const HOVER_OVERLAY: f32 = 0.06;
pub const PRESS_OVERLAY: f32 = 0.12;

/// Backdrop scrim behind modals — black at 60%.
pub const SCRIM: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 153);
