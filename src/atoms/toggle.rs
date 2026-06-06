//! Toggle atom — a two-state toggle *button* (distinct from [`Switch`](crate::atoms::Switch)).
//! [shadcn Toggle]
//!
//! Looks like a ghost button; fills with `accent` while on. Icon and/or label render as a
//! single valign-centered galley (like [`Button`](crate::atoms::Button)).

use crate::theme::typography;
use crate::tokens::core;
use crate::Theme;
use egui::{
    text::{LayoutJob, TextFormat},
    vec2, Align, Color32, CornerRadius, Id, Response, Sense, Ui,
};

/// A toggle button bound to a `&mut bool`. Builder; `show` returns the [`Response`].
pub struct Toggle<'a> {
    on: &'a mut bool,
    glyph: Option<&'static str>,
    label: Option<String>,
    enabled: bool,
    id_source: Option<Id>,
}

impl<'a> Toggle<'a> {
    pub fn new(on: &'a mut bool) -> Self {
        Self {
            on,
            glyph: None,
            label: None,
            enabled: true,
            id_source: None,
        }
    }

    pub fn icon(mut self, glyph: &'static str) -> Self {
        self.glyph = Some(glyph);
        self
    }
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
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
        let height = core::CONTROL_SM;
        let icon_size = core::ICON_MD;
        let fg = if self.enabled {
            theme.foreground
        } else {
            core::disabled_color(theme.foreground)
        };

        let fmt = |font: egui::FontId| TextFormat {
            font_id: font,
            color: fg,
            valign: Align::Center,
            ..Default::default()
        };
        let mut job = LayoutJob::default();
        job.wrap.max_width = f32::INFINITY;
        if let Some(glyph) = self.glyph {
            job.append(glyph, 0.0, fmt(typography::icon_font(icon_size)));
        }
        if let Some(label) = &self.label {
            let lead = if self.glyph.is_some() {
                core::SPACE_1
            } else {
                0.0
            };
            job.append(label, lead, fmt(typography::body().font_id()));
        }
        let galley = ui.painter().layout_job(job);
        let content = galley.size();
        let is_icon_only = self.label.is_none();
        let width = if is_icon_only {
            height
        } else {
            content.x + 2.0 * core::SPACE_2
        };

        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, mut response) = ui.allocate_exact_size(vec2(width, height), sense);
        if self.enabled && response.clicked() {
            *self.on = !*self.on;
            response.mark_changed();
        }
        let on = *self.on;
        let info_label = self.label.clone().unwrap_or_default();
        let info_enabled = self.enabled;
        response.widget_info(move || {
            egui::WidgetInfo::selected(
                egui::WidgetType::Button,
                info_enabled,
                on,
                info_label.clone(),
            )
        });

        let radius = CornerRadius::same(core::RADIUS_MD as u8);
        let painter = ui.painter();
        let overlay: Option<Color32> = if on {
            Some(theme.accent)
        } else if response.hovered() {
            Some(theme.hover_overlay)
        } else {
            None
        };
        if let Some(c) = overlay {
            painter.rect_filled(rect, radius, c);
        }
        painter.galley(rect.center() - content * 0.5, galley, fg);
        response
    }
}
