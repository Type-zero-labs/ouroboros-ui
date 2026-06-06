//! Checkbox atom — a `&mut bool` toggle with an optional label.
//!
//! Box painted from tokens (border `input`; checked = `primary` fill + a centered check
//! glyph in `primary_foreground`); label is a [`Text`] atom. Focus ring, disabled dim — all
//! token-driven, light+dark.

use crate::atoms::Text;
use crate::theme::typography;
use crate::tokens::core;
use crate::Theme;
use egui::{
    pos2, vec2, Color32, CornerRadius, Id, Rect, Response, Sense, Stroke, StrokeKind, Ui, UiBuilder,
};
use egui_phosphor::light;

/// A checkbox bound to a `&mut bool`. Builder; `show` returns the [`Response`].
pub struct Checkbox<'a> {
    checked: &'a mut bool,
    label: Option<String>,
    enabled: bool,
    id_source: Option<Id>,
}

impl<'a> Checkbox<'a> {
    pub fn new(checked: &'a mut bool) -> Self {
        Self {
            checked,
            label: None,
            enabled: true,
            id_source: None,
        }
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
        let box_size = core::ICON_MD;
        let gap = core::SPACE_2;
        let style = typography::body();

        let label_size = self.label.as_ref().map(|l| {
            ui.painter()
                .layout_no_wrap(l.clone(), style.font_id(), theme.foreground)
                .size()
        });
        let label_w = label_size.map_or(0.0, |s| s.x);
        let label_h = label_size.map_or(0.0, |s| s.y);
        let height = box_size.max(label_h);
        let width = box_size
            + if self.label.is_some() {
                gap + label_w
            } else {
                0.0
            };

        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, mut response) = ui.allocate_exact_size(vec2(width, height), sense);
        if self.enabled && response.clicked() {
            *self.checked = !*self.checked;
            response.mark_changed();
        }
        let checked = *self.checked;

        let info_label = self.label.clone().unwrap_or_default();
        let info_enabled = self.enabled;
        response.widget_info(move || {
            egui::WidgetInfo::selected(
                egui::WidgetType::Checkbox,
                info_enabled,
                checked,
                info_label.clone(),
            )
        });

        let dim = |c: Color32| {
            if self.enabled {
                c
            } else {
                c.gamma_multiply(core::OPACITY_DISABLED)
            }
        };
        let box_rect = Rect::from_min_size(
            pos2(rect.left(), rect.center().y - box_size / 2.0),
            vec2(box_size, box_size),
        );
        let radius = CornerRadius::same(core::RADIUS_SM as u8);
        let painter = ui.painter().clone();

        if checked {
            painter.rect_filled(box_rect, radius, dim(theme.primary));
        }
        let border = if checked { theme.primary } else { theme.input };
        painter.rect_stroke(
            box_rect,
            radius,
            Stroke::new(core::BORDER_THIN, dim(border)),
            StrokeKind::Inside,
        );
        if checked {
            let fg = dim(theme.primary_foreground);
            let glyph = painter.layout_no_wrap(
                light::CHECK.to_owned(),
                typography::icon_font(box_size),
                fg,
            );
            painter.galley(box_rect.center() - glyph.size() * 0.5, glyph, fg);
        }
        if response.has_focus() {
            painter.rect_stroke(
                box_rect.expand(2.0),
                radius,
                Stroke::new(core::BORDER_FOCUS, theme.ring),
                StrokeKind::Outside,
            );
        }

        if let Some(label) = self.label {
            let label_rect = Rect::from_min_size(
                pos2(box_rect.right() + gap, rect.center().y - label_h / 2.0),
                vec2(label_w, label_h),
            );
            let mut cui = ui.new_child(UiBuilder::new().max_rect(label_rect));
            Text::new(label).color(dim(theme.foreground)).show(&mut cui);
        }

        response
    }
}
