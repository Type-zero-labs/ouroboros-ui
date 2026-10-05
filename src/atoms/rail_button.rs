//! RailButton atom — one module icon on the vertical shell rail. [Figma left rail]
//!
//! A small square (`RAIL_BUTTON`, 24 px) with a 12 px icon, filled `card` at rest and
//! `rail_active` when it is the active module, with the hover veil on top. An optional numeric
//! badge (e.g. console warnings) sits on the top-right corner, and a tooltip with the name and
//! shortcut appears to the right on hover.

use crate::atoms::{Kbd, Text};
use crate::theme::typography;
use crate::tokens::{core, layout};
use crate::Theme;
use egui::{pos2, vec2, Align2, CornerRadius, Rect, Response, Sense, Ui};

/// A rail item. Builder; `show` returns the click [`Response`].
pub struct RailButton {
    icon: &'static str,
    active: bool,
    badge: Option<u32>,
    tooltip: Option<(String, Option<String>)>,
    size: f32,
    id: Option<egui::Id>,
}

impl RailButton {
    pub fn new(icon: &'static str) -> Self {
        Self {
            icon,
            active: false,
            badge: None,
            tooltip: None,
            size: layout::RAIL_BUTTON,
            id: None,
        }
    }
    /// The active module: filled with `rail_active`.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
    /// A count in the corner (hidden at 0 / `None`).
    pub fn badge(mut self, count: Option<u32>) -> Self {
        self.badge = count.filter(|c| *c > 0);
        self
    }
    /// Name (and optional shortcut) shown to the right on hover.
    pub fn tooltip(mut self, label: impl Into<String>, shortcut: Option<&str>) -> Self {
        self.tooltip = Some((label.into(), shortcut.map(str::to_owned)));
        self
    }
    /// Box size (default `RAIL_BUTTON`; the logo/footer slots use `RAIL_SLOT`).
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id = Some(egui::Id::new(id));
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let (rect, mut response) =
            ui.allocate_exact_size(vec2(self.size, self.size), Sense::click());
        if let Some(id) = self.id {
            response = ui.interact(rect, id, Sense::click());
        }
        let painter = ui.painter();
        let radius = CornerRadius::same(core::RADIUS_SM as u8);
        let fill = if self.active {
            theme.rail_active
        } else {
            theme.card
        };
        painter.rect_filled(rect, radius, fill);
        let t = core::hover_t(ui.ctx(), response.id, response.hovered());
        if t > 0.0 {
            painter.rect_filled(rect, radius, theme.hover_overlay.gamma_multiply(t));
        }
        let icon_color = if self.active {
            theme.foreground
        } else {
            theme.muted_foreground
        };
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            self.icon,
            typography::icon_font(layout::RAIL_ICON),
            icon_color,
        );
        if let Some(count) = self.badge {
            let label = if count > 99 {
                "99+".to_owned()
            } else {
                count.to_string()
            };
            let r = core::ICON_SM * 0.5;
            let c = pos2(rect.right() - r * 0.4, rect.top() + r * 0.4);
            let badge =
                Rect::from_center_size(c, vec2((label.len() as f32 * 6.0).max(r * 2.0), r * 2.0));
            painter.rect_filled(badge, CornerRadius::same(r as u8), theme.error);
            painter.text(
                badge.center(),
                Align2::CENTER_CENTER,
                label,
                typography::kbd().font_id(),
                theme.destructive_foreground,
            );
        }
        if let Some((label, _)) = &self.tooltip {
            let label = label.clone();
            let enabled = ui.is_enabled();
            response.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, &label)
            });
        }
        if let Some((label, shortcut)) = self.tooltip {
            response = response.on_hover_ui_at_pointer(move |ui| {
                ui.horizontal(|ui| {
                    Text::new(label).show(ui);
                    if let Some(s) = shortcut {
                        Kbd::new(s).show(ui);
                    }
                });
            });
        }
        response
    }
}
