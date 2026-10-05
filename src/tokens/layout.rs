//! Layout tokens — panel dimensions and component width constraints.
//!
//! egui is immediate-mode (no CSS grid), so these are *primitives* a layout helper or
//! component reads — standard panel widths and per-control width floors/ceilings. Tune
//! the panel values to the real studio shell.

// ─────────────────────────────────────────────────────────────────────────────
// Panels — standard shell dimensions (px). Starting points; tune to the studio.
// ─────────────────────────────────────────────────────────────────────────────

/// Left navigation / tree sidebar.
pub const SIDEBAR_WIDTH: f32 = 224.0;
/// Right properties / inspector panel.
pub const INSPECTOR_WIDTH: f32 = 260.0;
/// Minimum a resizable panel may shrink to.
pub const PANEL_MIN: f32 = 200.0;
/// Maximum a resizable panel may grow to.
pub const PANEL_MAX: f32 = 420.0;
/// Top toolbar height.
pub const TOOLBAR_HEIGHT: f32 = 40.0;
/// Bottom status bar height.
pub const STATUSBAR_HEIGHT: f32 = 24.0;
/// Canonical inner padding of a [`Panel`](crate::organisms::Panel) body/header/footer (= `core::SPACE_4`):
/// the single source of truth for panel content inset, replacing per-panel `Frame::inner_margin`.
pub const PANEL_PAD: f32 = super::core::SPACE_2;
/// Canonical gap between rows inside a [`Panel`](crate::organisms::Panel) body (= `core::SPACE_2`).
pub const PANEL_GAP: f32 = super::core::SPACE_1;

/// Component breakpoint: a responsive [`Field`](crate::molecules::Field) goes side-by-side
/// (label↔control) at/above this available width, else stacks.
pub const FIELD_HORIZONTAL_MIN: f32 = 480.0;

/// Fixed label column for an inspector [`PropertyRow`](crate::cells::PropertyRow) (aligned rows).
pub const PROPERTY_LABEL_WIDTH: f32 = 96.0;

/// Available width below which a responsive inspector row
/// ([`ResponsiveRow`](crate::cells::ResponsiveRow)) stacks the label above the control instead of
/// keeping the aligned column. Lower than [`FIELD_HORIZONTAL_MIN`] because inspector side panels
/// (≈280–480px) are narrower than full-width form fields.
pub const INSPECTOR_ROW_STACK_MIN: f32 = 220.0;

/// Row height for table cells/headers ([`TableCell`](crate::cells::TableCell)).
pub const TABLE_ROW_HEIGHT: f32 = 26.0;

// ─────────────────────────────────────────────────────────────────────────────
// Control width constraints — intrinsic floors/ceilings for fill-width atoms.
// Declared once on the component (like a Figma component's constraints) so any
// panel inherits sane shrink/grow behavior without local annotation.
// ─────────────────────────────────────────────────────────────────────────────

/// Floor for text inputs/textareas (cursor + a few chars).
pub const INPUT_MIN_W: f32 = 96.0;
/// Floor for a numeric field — matches the per-component floor VectorField already uses.
pub const NUMERIC_MIN_W: f32 = 48.0;
/// Floor for a numeric field **with stepper buttons** (`−`/`+` flank the value): two
/// sm icon buttons + a readable number. Below this the value paints over the buttons.
pub const NUMERIC_STEPPER_MIN_W: f32 = 88.0;
/// Canonical width cap of a numeric/value field (= the studio's FIELD_NUM_W): numbers
/// stay moderate and column-aligned instead of ballooning to the panel width.
pub const FIELD_NUM_W: f32 = 88.0;
/// Fixed width of a **stepper** numeric field ([`NumericField::fixed_width`](crate::atoms::NumericField::fixed_width)):
/// a constant, comfortable width (two sm icon buttons + a readable number) that ignores
/// `available_width` so the value never slides behind the `−` when a panel is squeezed.
/// ≥ [`NUMERIC_STEPPER_MIN_W`].
pub const NUMERIC_STEPPER_W: f32 = 96.0;
/// Floor for a slider track.
pub const SLIDER_MIN_W: f32 = 120.0;
/// Floor for a progress track.
pub const PROGRESS_MIN_W: f32 = 64.0;

// ── Shell (Figma-like layout, Oct 2026) ─────────────────────────────────────

/// Vertical module rail width (the divider sits on its right edge).
pub const RAIL_WIDTH: f32 = 52.0;
/// Rail item box (icon button).
pub const RAIL_BUTTON: f32 = 24.0;
/// Rail item icon.
pub const RAIL_ICON: f32 = 12.0;
/// Logo / footer box on the rail.
pub const RAIL_SLOT: f32 = 32.0;
/// Gap between rail items.
pub const RAIL_GAP: f32 = 12.0;
/// Side panel default width and resize bounds.
pub const SIDE_PANEL_W: f32 = 224.0;
pub const SIDE_PANEL_MIN: f32 = 200.0;
pub const SIDE_PANEL_MAX: f32 = 360.0;
/// Side panel header: title row + search.
pub const SIDE_HEADER_H: f32 = 88.0;
/// Tree / list row height.
pub const ROW_HEIGHT: f32 = 28.0;
/// Floating canvas toolbar height.
pub const FLOATING_TOOLBAR_H: f32 = 36.0;
/// Bottom drawer (console) default height.
pub const DRAWER_H: f32 = 240.0;
