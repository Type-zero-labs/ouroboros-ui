//! Switch atom — a `&mut bool` toggle with an animated thumb.
//!
//! Pill track (`primary` on / `muted` off) + a thumb that slides with `animate_bool_with_time`.
//! All dimensions derive from tokens; focus ring and disabled dim included.

use crate::tokens::core;
use crate::Theme;
use egui::{pos2, vec2, Color32, CornerRadius, Id, Response, Sense, Stroke, StrokeKind, Ui};

/// A switch bound to a `&mut bool`. Builder; `show` returns the [`Response`].
pub struct Switch<'a> {
    on: &'a mut bool,
    enabled: bool,
    id_source: Option<Id>,
}

impl<'a> Switch<'a> {
    pub fn new(on: &'a mut bool) -> Self {
        Self {
            on,
            enabled: true,
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
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id_source = Some(Id::new(id));
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let track_h = core::ICON_LG;
        let track_w = track_h + core::SPACE_4;
        let thumb_d = core::ICON_MD;
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
                c.gamma_multiply(core::OPACITY_DISABLED)
            }
        };
        let pill = CornerRadius::same((track_h / 2.0) as u8);
        let painter = ui.painter().clone();

        let track = if on { theme.primary } else { theme.muted };
        painter.rect_filled(rect, pill, dim(track));

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
            painter.rect_stroke(
                rect.expand(core::RING_OFFSET),
                pill,
                Stroke::new(core::BORDER_FOCUS, theme.ring),
                StrokeKind::Outside,
            );
        }

        response
    }
}
