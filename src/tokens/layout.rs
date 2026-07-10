//! Layout tokens — panel dimensions and component width constraints.
//!
//! egui is immediate-mode (no CSS grid), so these are *primitives* a layout helper or
//! component reads — standard panel widths and per-control width floors/ceilings. Tune
//! the panel values to the real studio shell.

// ─────────────────────────────────────────────────────────────────────────────
// Panels — standard shell dimensions (px). Starting points; tune to the studio.
// ─────────────────────────────────────────────────────────────────────────────

/// Left navigation / tree sidebar.
pub const SIDEBAR_WIDTH: f32 = 240.0;
/// Right properties / inspector panel.
pub const INSPECTOR_WIDTH: f32 = 300.0;
/// Minimum a resizable panel may shrink to.
pub const PANEL_MIN: f32 = 180.0;
/// Maximum a resizable panel may grow to.
pub const PANEL_MAX: f32 = 480.0;
/// Top toolbar height.
pub const TOOLBAR_HEIGHT: f32 = 40.0;
/// Bottom status bar height.
pub const STATUSBAR_HEIGHT: f32 = 24.0;
/// Canonical inner padding of a [`Panel`](crate::organisms::Panel) body/header/footer (= `core::SPACE_4`):
/// the single source of truth for panel content inset, replacing per-panel `Frame::inner_margin`.
pub const PANEL_PAD: f32 = super::core::SPACE_4;
/// Canonical gap between rows inside a [`Panel`](crate::organisms::Panel) body (= `core::SPACE_2`).
pub const PANEL_GAP: f32 = super::core::SPACE_2;

/// Component breakpoint: a responsive [`Field`](crate::molecules::Field) goes side-by-side
/// (label↔control) at/above this available width, else stacks.
pub const FIELD_HORIZONTAL_MIN: f32 = 480.0;

/// Fixed label column for an inspector [`PropertyRow`](crate::cells::PropertyRow) (aligned rows).
pub const PROPERTY_LABEL_WIDTH: f32 = 120.0;

/// Available width below which a responsive inspector row
/// ([`ResponsiveRow`](crate::cells::ResponsiveRow)) stacks the label above the control instead of
/// keeping the aligned column. Lower than [`FIELD_HORIZONTAL_MIN`] because inspector side panels
/// (≈280–480px) are narrower than full-width form fields.
pub const INSPECTOR_ROW_STACK_MIN: f32 = 220.0;

/// Row height for table cells/headers ([`TableCell`](crate::cells::TableCell)).
pub const TABLE_ROW_HEIGHT: f32 = 28.0;

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
pub const FIELD_NUM_W: f32 = 120.0;
/// Fixed width of a **stepper** numeric field ([`NumericField::fixed_width`](crate::atoms::NumericField::fixed_width)):
/// a constant, comfortable width (two sm icon buttons + a readable number) that ignores
/// `available_width` so the value never slides behind the `−` when a panel is squeezed.
/// ≥ [`NUMERIC_STEPPER_MIN_W`].
pub const NUMERIC_STEPPER_W: f32 = 120.0;
/// Floor for a slider track.
pub const SLIDER_MIN_W: f32 = 120.0;
/// Floor for a progress track.
pub const PROGRESS_MIN_W: f32 = 64.0;
