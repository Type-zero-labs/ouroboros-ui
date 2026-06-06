//! Radio atom — a single radio button (`selected: bool`) with an optional label.
//!
//! A standalone atom: it reports clicks; the consumer (or a future RadioGroup molecule)
//! owns single-selection. Circle from tokens (border `input`; selected = inner `primary`
//! dot); label is a [`Text`] atom.

use crate::atoms::Text;
use crate::theme::typography;
use crate::tokens::core::{self, Size};
use crate::Theme;
use egui::{pos2, vec2, Color32, Id, Rect, Response, Sense, Stroke, Ui, UiBuilder};

/// A single radio button. Builder; `show` returns the [`Response`] (`clicked` flips selection
/// at the consumer).
pub struct Radio {
    selected: bool,
    label: Option<String>,
    enabled: bool,
    interactive: bool,
    size: Size,
    id_source: Option<Id>,
}

impl Radio {
    pub fn new(selected: bool) -> Self {
        Self {
            selected,
            label: None,
            enabled: true,
            interactive: true,
            size: Size::default(),
            id_source: None,
        }
    }

    /// Display-only (no click) — e.g. inside a clickable card.
    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
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
        let size = self.size.icon_size();
        let gap = core::SPACE_2;
        let style = typography::body();

        let label_size = self.label.as_ref().map(|l| {
            ui.painter()
                .layout_no_wrap(l.clone(), style.font_id(), theme.foreground)
                .size()
        });
        // `layout_no_wrap` measures without letter-spacing, but the label is rendered by the
        // Text atom *with* the role's tracking — reserve that extra width so it doesn't clip.
        let label_w = label_size.map_or(0.0, |s| s.x)
            + self
                .label
                .as_ref()
                .map_or(0.0, |l| style.tracking * l.chars().count() as f32);
        let label_h = label_size.map_or(0.0, |s| s.y);
        let height = size.max(label_h);
        let width = size
            + if self.label.is_some() {
                gap + label_w
            } else {
                0.0
            };

        let sense = if self.enabled && self.interactive {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, response) = ui.allocate_exact_size(vec2(width, height), sense);

        let info_label = self.label.clone().unwrap_or_default();
        let info_enabled = self.enabled;
        let selected = self.selected;
        response.widget_info(move || {
            egui::WidgetInfo::selected(
                egui::WidgetType::RadioButton,
                info_enabled,
                selected,
                info_label.clone(),
            )
        });

        let dim = |c: Color32| {
            if self.enabled {
                c
            } else {
                core::disabled_color(c)
            }
        };
        let center = pos2(rect.left() + size / 2.0, rect.center().y);
        let radius = size / 2.0;
        let painter = ui.painter().clone();

        let border = if self.selected {
            theme.primary
        } else {
            theme.input
        };
        painter.circle_stroke(center, radius, Stroke::new(core::BORDER_THIN, dim(border)));

        // Animated hover veil — gated on enabled + interactive.
        let hovered = self.enabled && self.interactive && response.hovered();
        let ht = core::hover_t(ui.ctx(), response.id, hovered);
        if ht > 0.0 {
            painter.circle_filled(center, radius, theme.hover_overlay.gamma_multiply(ht));
        }

        if self.selected {
            painter.circle_filled(center, radius * 0.5, dim(theme.primary));
        }
        if response.has_focus() {
            super::focus::focus_ring_circle(&painter, center, radius, theme.ring);
        }

        if let Some(label) = self.label {
            let label_rect = Rect::from_min_size(
                pos2(rect.left() + size + gap, rect.center().y - label_h / 2.0),
                vec2(label_w, label_h),
            );
            let mut cui = ui.new_child(UiBuilder::new().max_rect(label_rect));
            Text::new(label).color(dim(theme.foreground)).show(&mut cui);
        }

        response
    }
}
