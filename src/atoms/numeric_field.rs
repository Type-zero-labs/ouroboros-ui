//! NumericField atom — a scrubbable numeric input. [Unity Numeric Field]
//!
//! Token box wrapping an egui [`DragValue`](egui::DragValue): drag to scrub, click to type.
//! The value is right-aligned. `.stepper()` flanks it with `−`/`+` icon buttons; `.suffix()`
//! appends a unit. The editing substrate is egui's; the casing is token.
//!
//! Width is intrinsically constrained: the field fills the available width clamped to
//! [`layout::NUMERIC_MIN_W`]`..=`[`layout::FIELD_NUM_W`], so numbers stay moderate and
//! column-aligned instead of ballooning to the panel. `.full_width()` drops the cap;
//! `.fixed_width()` pins a constant [`layout::NUMERIC_STEPPER_W`] (for a stepper in a
//! squeezed panel — the value never slides behind the `−`).

use crate::atoms::Button;
use crate::tokens::core::{self, Size};
use crate::tokens::layout;
use crate::Theme;
use egui::{
    vec2, Align, Color32, CornerRadius, DragValue, Layout, Response, Sense, Stroke, StrokeKind, Ui,
    UiBuilder,
};
use egui_phosphor::light;

/// A scrubbable numeric field bound to a `&mut f32`. Builder; `show` returns the [`Response`].
pub struct NumericField<'a> {
    value: &'a mut f32,
    min: f32,
    max: f32,
    speed: f32,
    step: Option<f32>,
    suffix: Option<String>,
    stepper: bool,
    full_width: bool,
    fixed_width: bool,
    enabled: bool,
    error: bool,
    size: Size,
    decimals: Option<usize>,
}

impl<'a> NumericField<'a> {
    pub fn new(value: &'a mut f32) -> Self {
        Self {
            value,
            min: f32::NEG_INFINITY,
            max: f32::INFINITY,
            speed: 0.1,
            step: None,
            suffix: None,
            stepper: false,
            full_width: false,
            fixed_width: false,
            enabled: true,
            error: false,
            size: Size::default(),
            decimals: None,
        }
    }

    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
    /// Increment used by the [`stepper`](Self::stepper) buttons (default `1.0`).
    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }
    /// Flank the value with `−`/`+` buttons.
    pub fn stepper(mut self) -> Self {
        self.stepper = true;
        self
    }
    /// Fill the available width — drops the [`layout::FIELD_NUM_W`] cap (the
    /// [`layout::NUMERIC_MIN_W`] floor still applies).
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }
    /// Use a **fixed** width ([`layout::NUMERIC_STEPPER_W`]) that ignores `available_width`.
    /// For [`stepper`](Self::stepper) fields in a squeezed panel: the box never shrinks, so the
    /// value never slides behind the `−`/`+` buttons. Takes precedence over [`full_width`](Self::full_width).
    pub fn fixed_width(mut self) -> Self {
        self.fixed_width = true;
        self
    }
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }
    /// Show and edit with a fixed number of decimal places. Default (`None`) keeps egui's
    /// adaptive formatting (integers stay integers) — the original behavior.
    pub fn decimals(mut self, decimals: usize) -> Self {
        self.decimals = Some(decimals);
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    pub fn disabled(self) -> Self {
        self.enabled(false)
    }
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
    pub fn sm(self) -> Self {
        self.size(Size::Sm)
    }
    pub fn lg(self) -> Self {
        self.size(Size::Lg)
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let height = self.size.height();
        // Intrinsic constraints: floor so the field never collapses; cap so a numeric
        // value stays moderate in wide panels (`.full_width()` keeps only the floor).
        // A stepper needs room for both flanking buttons besides the value.
        let floor = if self.stepper {
            layout::NUMERIC_STEPPER_MIN_W
        } else {
            layout::NUMERIC_MIN_W
        };
        let width = if self.fixed_width {
            // A constant width, independent of the panel: the stepper keeps its shape under
            // squeeze so the value never paints over the flanking buttons.
            layout::NUMERIC_STEPPER_W.max(floor)
        } else if self.full_width {
            ui.available_width().max(floor)
        } else {
            ui.available_width()
                .clamp(floor, layout::FIELD_NUM_W.max(floor))
        };
        let (rect, box_resp) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        let enabled = self.enabled;
        let error = self.error;
        let dim = |c: Color32| {
            if enabled {
                c
            } else {
                core::disabled_color(c)
            }
        };
        let radius = CornerRadius::same(core::RADIUS_MD as u8);
        let painter = ui.painter().clone();
        painter.rect_filled(rect, radius, dim(theme.muted));

        // Animated hover veil — gated on enabled.
        let hovered = enabled && ui.rect_contains_pointer(rect);
        let ht = core::hover_t(ui.ctx(), box_resp.id, hovered);
        if ht > 0.0 {
            painter.rect_filled(rect, radius, theme.hover_overlay.gamma_multiply(ht));
        }

        let inner = rect.shrink2(vec2(core::SPACE_2, 0.0));
        let step = self.step.unwrap_or(1.0);
        let suffix = self.suffix;
        let decimals = self.decimals;
        let (min, max, speed) = (self.min, self.max, self.speed);
        let value = self.value;
        macro_rules! drag {
            ($v:expr) => {{
                let mut dv = DragValue::new($v).speed(speed).range(min..=max);
                if let Some(suffix) = &suffix {
                    dv = dv.suffix(suffix.clone());
                }
                if let Some(d) = decimals {
                    dv = dv.fixed_decimals(d);
                }
                dv
            }};
        }

        let resp = if self.stepper {
            // Three bands carved off the inner rect — minus | value | plus — so the value is
            // centered across the FULL width between the two buttons (symmetric), not just the
            // remainder after the minus button (which lived in a separate container before).
            let btn_w = core::CONTROL_SM;
            let (minus_rect, rest) = inner.split_left_right_at_x(inner.left() + btn_w);
            let (value_rect, plus_rect) = rest.split_left_right_at_x(rest.right() - btn_w);

            let mut mui = ui.new_child(
                UiBuilder::new()
                    .max_rect(minus_rect)
                    .layout(Layout::left_to_right(Align::Center)),
            );
            let minus = Button::new("")
                .icon_left(light::MINUS)
                .icon_only()
                .ghost()
                .sm()
                .show(&mut mui);
            let minus_clicked = enabled && minus.clicked();
            if minus_clicked {
                *value = (*value - step).clamp(min, max);
            }

            let mut vui = ui.new_child(
                UiBuilder::new()
                    .max_rect(value_rect)
                    .layout(Layout::centered_and_justified(egui::Direction::LeftToRight)),
            );
            let mut r = vui.add_enabled(enabled, drag!(value));

            let mut pui = ui.new_child(
                UiBuilder::new()
                    .max_rect(plus_rect)
                    .layout(Layout::right_to_left(Align::Center)),
            );
            let plus = Button::new("")
                .icon_left(light::PLUS)
                .icon_only()
                .ghost()
                .sm()
                .show(&mut pui);
            let plus_clicked = enabled && plus.clicked();
            if plus_clicked {
                *value = (*value + step).clamp(min, max);
            }

            // The `−`/`+` buttons mutate the value directly; fold their clicks into the
            // returned response so callers gated on `.changed()` (e.g. dirty-tracking
            // editors) react to stepper edits, not just keyboard/drag edits.
            if minus_clicked || plus_clicked {
                r.mark_changed();
            }
            r
        } else {
            let mut cui = ui.new_child(
                UiBuilder::new()
                    .max_rect(inner)
                    .layout(Layout::right_to_left(Align::Center)),
            );
            cui.add_enabled(enabled, drag!(value))
        };

        let (border, w) = if error {
            (theme.destructive, core::BORDER_THIN)
        } else if resp.has_focus() {
            (theme.ring, core::BORDER_FOCUS)
        } else {
            (theme.input, core::BORDER_THIN)
        };
        painter.rect_stroke(
            rect,
            radius,
            Stroke::new(w, dim(border)),
            StrokeKind::Inside,
        );
        resp
    }
}
