//! Core primitives — raw values with no semantic meaning.
//!
//! Color ramps, spacing scale, radius scale, shadow primitives, and type sizes.
//! Everything here is a `const`; the semantic layer maps meaning onto these.
//!
//! Decided interactively (value-by-value against shadcn). Sections fill in as the
//! token groups are confirmed.

use egui::{epaint::Shadow, Color32};

// ─────────────────────────────────────────────────────────────────────────────
// Neutral base ramp — Zinc (cool-neutral). Tailwind/shadcn `zinc` 50→950. The
// temperature of every gray surface/border/text token; the brand hue lives in the
// teal ramp below (primary), not here, so the zinc look is preserved.
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
// Brand ramp — Ouroboros turquoise (Tailwind `teal`). This is the **primary** hue:
// buttons, progress/slider fill, switch-on, focus ring, selection. Kept a touch
// light (300/400 in dark) per the brand. The neutral zinc above is untouched, so a
// pure-zinc theme stays available.
// ─────────────────────────────────────────────────────────────────────────────

pub const TEAL_200: Color32 = Color32::from_rgb(153, 246, 228);
pub const TEAL_300: Color32 = Color32::from_rgb(94, 234, 212);
pub const TEAL_400: Color32 = Color32::from_rgb(45, 212, 191);
pub const TEAL_500: Color32 = Color32::from_rgb(20, 184, 166);
pub const TEAL_600: Color32 = Color32::from_rgb(13, 148, 136);

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

/// Zero spacing — the semantic "no gap / no padding" sentinel (tight tables, full-bleed).
pub const SPACE_0: f32 = 0.0;
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

/// Zero radius — the semantic "square corners" sentinel (full-bleed rows, flush panels).
pub const RADIUS_NONE: f32 = 0.0;
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

/// Parameterized shadow builder — for elevations beyond the fixed `SM`/`MD`/`LG` triples
/// (custom blur/spread, directional offsets). `egui::epaint::Shadow` is foreign, so this is a
/// free `const fn` rather than an inherent `Shadow::new`. `const`, usable in token context.
pub const fn shadow(offset: [i8; 2], blur: u8, spread: u8, color: Color32) -> Shadow {
    Shadow {
        offset,
        blur,
        spread,
        color,
    }
}

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

/// Letter-spacing (px, extra per glyph). For legibility the scale is **inverse to size**:
/// big titles (display/h1/h2) stay at `NORMAL`; the smaller the text, the wider the tracking
/// (heading→SM, body→MD, label/code→LG, caption/kbd→WIDE).
pub const TRACKING_TIGHT: f32 = -0.25;
pub const TRACKING_NORMAL: f32 = 0.0;
pub const TRACKING_SM: f32 = 0.4;
pub const TRACKING_MD: f32 = 0.6;
pub const TRACKING_LG: f32 = 0.8;
pub const TRACKING_WIDE: f32 = 1.0;

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

/// Shared control size scale. One source of truth for every form control's footprint,
/// so density (compact toolbar vs. roomy panel) is expressible uniformly. Numeric here
/// (no semantic/theme dependency); the typography mapping lives in `theme::typography`
/// as [`Size::text_style`] to keep this layer a leaf.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    Sm,
    #[default]
    Md,
    Lg,
}

impl Size {
    /// Control height (px) — [`CONTROL_SM`]/[`CONTROL_MD`]/[`CONTROL_LG`].
    pub fn height(self) -> f32 {
        match self {
            Size::Sm => CONTROL_SM,
            Size::Md => CONTROL_MD,
            Size::Lg => CONTROL_LG,
        }
    }
    /// Icon box size (px) — [`ICON_SM`]/[`ICON_MD`]/[`ICON_LG`].
    pub fn icon_size(self) -> f32 {
        match self {
            Size::Sm => ICON_SM,
            Size::Md => ICON_MD,
            Size::Lg => ICON_LG,
        }
    }
    /// Horizontal padding (px) — tighter at `Sm`.
    pub fn pad_x(self) -> f32 {
        match self {
            Size::Sm => SPACE_3,
            Size::Md | Size::Lg => SPACE_4,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Motion — animation durations (seconds) + easing curves. egui drives hover/focus
// transitions by duration (`ctx.animate_*`); [`Easing`] shapes the progress.
// ─────────────────────────────────────────────────────────────────────────────

pub const DURATION_INSTANT: f32 = 0.0;
pub const DURATION_FAST: f32 = 0.10;
pub const DURATION_NORMAL: f32 = 0.18;
pub const DURATION_SLOW: f32 = 0.30;

/// Stagger/hold before an animation starts (seconds) — e.g. tooltip dwell, toast settle.
pub const DURATION_DELAY_SHORT: f32 = 0.15;
pub const DURATION_DELAY_LONG: f32 = 0.50;

/// Easing curve applied to a normalized progress `t` in `0..=1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Easing {
    Linear,
    /// Decelerate — natural for enter/hover. The default.
    #[default]
    EaseOut,
    /// Accelerate then decelerate — for moves/reorders.
    EaseInOut,
    /// Overshoot then settle (ease-out-back) — playful enters, springy toggles.
    Spring,
    /// Settle with a few decaying bounces (ease-out-bounce) — drops, attention pulls.
    Bounce,
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
            // ease-out-back: overshoots past 1 near the end, then settles.
            Easing::Spring => {
                const C1: f32 = 1.70158;
                const C3: f32 = C1 + 1.0;
                let u = t - 1.0;
                1.0 + C3 * u * u * u + C1 * u * u
            }
            // ease-out-bounce: decaying parabolic bounces (standard 4-segment form).
            Easing::Bounce => {
                const N1: f32 = 7.5625;
                const D1: f32 = 2.75;
                if t < 1.0 / D1 {
                    N1 * t * t
                } else if t < 2.0 / D1 {
                    let t = t - 1.5 / D1;
                    N1 * t * t + 0.75
                } else if t < 2.5 / D1 {
                    let t = t - 2.25 / D1;
                    N1 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / D1;
                    N1 * t * t + 0.984375
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

/// Blend a color to its disabled appearance — alpha × [`OPACITY_DISABLED`]. The single source
/// for the disabled veil; atoms gate it behind their own `if !enabled` so it's applied once.
pub fn disabled_color(c: Color32) -> Color32 {
    c.gamma_multiply(OPACITY_DISABLED)
}

/// Eased hover progress in `0..=1` for a widget. The single source for hover transitions:
/// animates `hovered` over [`DURATION_FAST`] and shapes it with [`Easing::EaseOut`], so every
/// atom fades its hover overlay/border identically. Pass a stable `id` (e.g. `response.id`);
/// returns `0.0` when not hovering and settled.
pub fn hover_t(ctx: &egui::Context, id: egui::Id, hovered: bool) -> f32 {
    let raw = ctx.animate_bool_with_time(id.with("hover"), hovered, DURATION_FAST);
    Easing::EaseOut.apply(raw)
}

/// Re-tint a color to a given alpha (`0..=255`), preserving its RGB. The single source for
/// translucent fills (selection marquee, soft overlays) built from a token color, so the
/// `graph` layer never reaches for a raw `Color32::from_rgba_*`. Lives here (a non-scanned
/// leaf) by design; callers pass a `Theme`/`core` color and an alpha.
pub fn tint(color: Color32, alpha: u8) -> Color32 {
    let [r, g, b, _] = color.to_array();
    Color32::from_rgba_unmultiplied(r, g, b, alpha)
}

// ─────────────────────────────────────────────────────────────────────────────
// Graph geometry — neutral primitives for the `graph` node-editor layer (dot grid,
// edges, handles). Sizes only; every color comes from `Theme` via `graph::GraphTokens`.
// Screen-space px unless a scale is applied; hit radii stay screen-constant on purpose
// (a thin edge/handle must remain grabbable when zoomed out).
// ─────────────────────────────────────────────────────────────────────────────

/// Radius (px) of a single background grid dot.
pub const GRID_DOT_RADIUS: f32 = 1.0;
/// Base spacing (px) between grid dots at zoom = 1 (scaled by zoom at paint time).
pub const GRID_SPACING: f32 = 28.0;
/// Default edge (wire) stroke width.
pub const EDGE_WIDTH: f32 = 2.0;
/// Pointer-to-edge distance (screen px) that still counts as a hover/click hit.
pub const EDGE_HIT_RADIUS: f32 = 6.0;
/// Radius (world px, pre-zoom) of a connection handle.
pub const HANDLE_RADIUS: f32 = 5.0;
/// Translucent fill alpha for the box-select marquee.
pub const MARQUEE_ALPHA: u8 = 38;
