//! Switch atom — a `&mut bool` toggle with an animated thumb.
//!
//! Pill track (`primary` on / `muted` off) + a thumb that slides with `animate_bool_with_time`.
//! All dimensions derive from tokens; focus ring and disabled dim included.

use crate::tokens::core::{self, Size};
use crate::Theme;
use egui::{pos2, vec2, Color32, CornerRadius, Id, Response, Sense, Ui};

/// A switch bound to a `&mut bool`. Builder; `show` returns the [`Response`].
pub struct Switch<'a> {
    on: &'a mut bool,
    enabled: bool,
    size: Size,
    id_source: Option<Id>,
}

impl<'a> Switch<'a> {
    pub fn new(on: &'a mut bool) -> Self {
        Self {
            on,
            enabled: true,
            size: Size::default(),
            id_source: None,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    pub fn disabled(self) -> Self {
        self.enabled(false)
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
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id_source = Some(Id::new(id));
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        // Track/thumb scale with Size; Md preserves the original dimensions.
        let (track_h, thumb_d) = match self.size {
            Size::Sm => (core::ICON_MD, core::ICON_SM),
            Size::Md => (core::ICON_LG, core::ICON_MD),
            Size::Lg => (core::ICON_XL, core::ICON_LG),
        };
        let track_w = track_h + core::SPACE_4;
        let inset = (track_h - thumb_d) / 2.0;

        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, mut response) = ui.allocate_exact_size(vec2(track_w, track_h), sense);
        if self.enabled && response.clicked() {
            *self.on = !*self.on;
            response.mark_changed();
        }
        let on = *self.on;

        let info_enabled = self.enabled;
        response.widget_info(move || {
            egui::WidgetInfo::selected(egui::WidgetType::Checkbox, info_enabled, on, "")
        });

        let id = self.id_source.unwrap_or(response.id);
        let t = ui
            .ctx()
            .animate_bool_with_time(id.with("switch"), on, core::DURATION_FAST);

        let dim = |c: Color32| {
            if self.enabled {
                c
            } else {
                core::disabled_color(c)
            }
        };
        let pill = CornerRadius::same((track_h / 2.0) as u8);
        let painter = ui.painter().clone();

        // Off-track uses `border_strong` (not `muted`) so the thumb stays legible in dark mode.
        let track = if on {
            theme.primary
        } else {
            theme.border_strong
        };
        painter.rect_filled(rect, pill, dim(track));

        // Animated hover veil — gated on enabled.
        let hovered = self.enabled && response.hovered();
        let ht = core::hover_t(ui.ctx(), id, hovered);
        if ht > 0.0 {
            painter.rect_filled(rect, pill, theme.hover_overlay.gamma_multiply(ht));
        }

        let thumb_r = thumb_d / 2.0;
        let left_x = rect.left() + inset + thumb_r;
        let right_x = rect.right() - inset - thumb_r;
        let thumb_x = left_x + t * (right_x - left_x);
        painter.circle_filled(
            pos2(thumb_x, rect.center().y),
            thumb_r,
            dim(theme.background),
        );

        if response.has_focus() {
            super::focus::focus_ring_rect(&painter, rect, pill, theme.ring);
        }

        response
    }
}
