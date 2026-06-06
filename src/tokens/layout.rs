//! Layout & layering tokens — panel dimensions, a content grid, responsive
//! breakpoints, and stacking [`Layer`]s.
//!
//! egui is immediate-mode (no CSS grid), so these are *primitives* a layout helper or
//! component reads — standard panel widths, a 12-column content grid, window-width
//! breakpoints, and z-order roles mapped onto [`egui::Order`]. Tune the panel/grid
//! values to the real studio shell.

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

// ─────────────────────────────────────────────────────────────────────────────
// Content grid — a 12-column grid for laying out forms / cards inside a panel.
// ─────────────────────────────────────────────────────────────────────────────

/// Column count of the content grid.
pub const GRID_COLUMNS: usize = 12;
/// Gap between columns/rows (= `core::SPACE_4`).
pub const GRID_GUTTER: f32 = super::core::SPACE_4;
/// Max readable content width before centering.
pub const CONTAINER_MAX: f32 = 1200.0;

// ─────────────────────────────────────────────────────────────────────────────
// Breakpoints — window-width thresholds (px) for adapting layout density.
// ─────────────────────────────────────────────────────────────────────────────

/// Below this: compact (single column, collapsed panels).
pub const BREAKPOINT_COMPACT: f32 = 720.0;
/// Below this: normal (one side panel).
pub const BREAKPOINT_NORMAL: f32 = 1024.0;
/// At/above this: wide (both side panels, roomy).
pub const BREAKPOINT_WIDE: f32 = 1440.0;

/// Component breakpoint: a responsive [`Field`](crate::molecules::Field) goes side-by-side
/// (label↔control) at/above this available width, else stacks.
pub const FIELD_HORIZONTAL_MIN: f32 = 480.0;

/// Fixed label column for an inspector [`PropertyRow`](crate::cells::PropertyRow) (aligned rows).
pub const PROPERTY_LABEL_WIDTH: f32 = 120.0;

/// Responsive size class derived from the available width.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeClass {
    Compact,
    Normal,
    Wide,
}

impl SizeClass {
    /// Classify an available width against the breakpoints.
    pub fn from_width(width: f32) -> Self {
        if width < BREAKPOINT_NORMAL {
            SizeClass::Compact
        } else if width < BREAKPOINT_WIDE {
            SizeClass::Normal
        } else {
            SizeClass::Wide
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Layering — z-order roles, mapped onto `egui::Order`. Ordered base → tooltip.
// ─────────────────────────────────────────────────────────────────────────────

/// Stacking role for floating surfaces. Lower variants sit beneath higher ones.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Layer {
    Base,
    Dropdown,
    Popover,
    Modal,
    Toast,
    Tooltip,
}

impl Layer {
    /// The egui paint order this role maps to. (egui's order set is coarse; finer
    /// ordering within a layer is by creation/`priority`.)
    pub fn order(self) -> egui::Order {
        match self {
            Layer::Base => egui::Order::Middle,
            Layer::Dropdown | Layer::Popover | Layer::Modal | Layer::Toast => {
                egui::Order::Foreground
            }
            Layer::Tooltip => egui::Order::Tooltip,
        }
    }

    /// Relative priority within a shared egui order (higher = on top).
    pub fn priority(self) -> i32 {
        self as i32
    }
}
