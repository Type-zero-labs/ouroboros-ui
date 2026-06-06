//! Surface atom — a token-painted container that holds content.
//!
//! The one place a "box" is painted (fill / border / radius / shadow / padding), so molecules
//! can compose a surface instead of hand-rolling a frame. Atoms may paint; molecules may not.
//! Optional `interactive` (clickable) and `selected` (ring border) for card-style selectors.

use crate::tokens::core;
use crate::Theme;
use egui::{CornerRadius, Id, Sense, Stroke, StrokeKind, Ui};

/// Background fill of a [`Surface`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SurfaceFill {
    #[default]
    Card,
    Muted,
    Background,
    None,
}

/// Border of a [`Surface`] (overridden by `selected` → `ring`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SurfaceBorder {
    None,
    #[default]
    Default,
    Strong,
}

/// A container surface. Builder; `show` paints the box and runs `content` inside it.
pub struct Surface {
    fill: SurfaceFill,
    border: SurfaceBorder,
    radius: f32,
    elevated: bool,
    padding: f32,
    interactive: bool,
    selected: bool,
    id_source: Option<Id>,
}

impl Surface {
    pub fn new() -> Self {
        Self {
            fill: SurfaceFill::Card,
            border: SurfaceBorder::Default,
            radius: core::RADIUS_LG,
            elevated: false,
            padding: core::SPACE_4,
            interactive: false,
            selected: false,
            id_source: None,
        }
    }

    pub fn fill(mut self, fill: SurfaceFill) -> Self {
        self.fill = fill;
        self
    }
    pub fn muted(self) -> Self {
        self.fill(SurfaceFill::Muted)
    }
    pub fn background(self) -> Self {
        self.fill(SurfaceFill::Background)
    }
    pub fn fill_none(self) -> Self {
        self.fill(SurfaceFill::None)
    }

    pub fn border(mut self, border: SurfaceBorder) -> Self {
        self.border = border;
        self
    }
    pub fn border_none(self) -> Self {
        self.border(SurfaceBorder::None)
    }
    pub fn border_strong(self) -> Self {
        self.border(SurfaceBorder::Strong)
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
    pub fn elevated(mut self) -> Self {
        self.elevated = true;
        self
    }
    pub fn pad(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }
    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id_source = Some(Id::new(id));
        self
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> egui::InnerResponse<R> {
        let theme = Theme::get(ui);
        let radius = CornerRadius::same(self.radius as u8);

        let mut frame = egui::Frame::default()
            .corner_radius(radius)
            .inner_margin(self.padding);
        match self.fill {
            SurfaceFill::Card => frame = frame.fill(theme.card),
            SurfaceFill::Muted => frame = frame.fill(theme.muted),
            SurfaceFill::Background => frame = frame.fill(theme.background),
            SurfaceFill::None => {}
        }
        if self.selected {
            frame = frame.stroke(Stroke::new(core::BORDER_FOCUS, theme.ring));
        } else {
            match self.border {
                SurfaceBorder::None => {}
                SurfaceBorder::Default => {
                    frame = frame.stroke(Stroke::new(core::BORDER_THIN, theme.border))
                }
                SurfaceBorder::Strong => {
                    frame = frame.stroke(Stroke::new(core::BORDER_THIN, theme.border_strong))
                }
            }
        }
        if self.elevated {
            frame = frame.shadow(core::SHADOW_MD);
        }

        let inner = frame.show(ui, content);
        let mut response = inner.response;
        if self.interactive {
            let id = self.id_source.unwrap_or(response.id);
            response = ui.interact(response.rect, id, Sense::click());
            if response.hovered() {
                ui.painter().rect_stroke(
                    response.rect,
                    radius,
                    Stroke::new(core::BORDER_THIN, theme.border_strong),
                    StrokeKind::Inside,
                );
            }
        }
        egui::InnerResponse::new(inner.inner, response)
    }
}

impl Default for Surface {
    fn default() -> Self {
        Self::new()
    }
}
