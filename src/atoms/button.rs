//! Button atom — labeled, optionally-iconed click control with shadcn variants.
//!
//! Icon + label render as a **single galley** (each section `valign`-centered) so the icon
//! and text share one baseline and align optically — composing two separate labels centers
//! them by cell, which looks off. Fonts come from `typography`, fill/foreground/border from
//! [`ButtonTokens`], and hover/press/disabled/focus from motion/opacity/border tokens, so
//! light+dark and every state stay token-driven. (Atoms may paint; only molecules/organisms
//! must compose.)

use crate::theme::typography;
use crate::tokens::component::ButtonTokens;
use crate::tokens::core::{self, Size};
use crate::Theme;
use egui::{vec2, Color32, CornerRadius, Id, Response, Sense, Shape, Stroke, StrokeKind, Ui, Vec2};

/// shadcn button variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

/// A click button. Builder; `show` returns the [`Response`] (`clicked`, `hovered`, …).
pub struct Button {
    label: String,
    variant: ButtonVariant,
    size: Size,
    icon_only: bool,
    loading: bool,
    icon_left: Option<&'static str>,
    icon_right: Option<&'static str>,
    enabled: bool,
    id_source: Option<Id>,
    icon_px: Option<f32>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::default(),
            size: Size::default(),
            icon_only: false,
            loading: false,
            icon_left: None,
            icon_right: None,
            enabled: true,
            id_source: None,
            icon_px: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn secondary(self) -> Self {
        self.variant(ButtonVariant::Secondary)
    }
    pub fn destructive(self) -> Self {
        self.variant(ButtonVariant::Destructive)
    }
    pub fn outline(self) -> Self {
        self.variant(ButtonVariant::Outline)
    }
    pub fn ghost(self) -> Self {
        self.variant(ButtonVariant::Ghost)
    }
    pub fn link(self) -> Self {
        self.variant(ButtonVariant::Link)
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
    /// Render as a square, icon-only button (label dropped). Composes with [`Self::size`].
    pub fn icon_only(mut self) -> Self {
        self.icon_only = true;
        self
    }
    /// Override the glyph box size (px), independent of [`Self::size`]. Defaults to the
    /// size's icon box (`Size::icon_size`). Use e.g. `core::ICON_XL` for a 24px rail icon.
    pub fn icon_px(mut self, px: f32) -> Self {
        self.icon_px = Some(px);
        self
    }

    /// Show an indeterminate spinner in place of the content and ignore clicks. Width is
    /// preserved so a button does not resize when toggled into loading.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn icon_left(mut self, glyph: &'static str) -> Self {
        self.icon_left = Some(glyph);
        self
    }
    pub fn icon_right(mut self, glyph: &'static str) -> Self {
        self.icon_right = Some(glyph);
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
        let bt = match self.variant {
            ButtonVariant::Default => ButtonTokens::primary(&theme),
            ButtonVariant::Secondary => ButtonTokens::secondary(&theme),
            ButtonVariant::Destructive => ButtonTokens::destructive(&theme),
            ButtonVariant::Outline => ButtonTokens::outline(&theme),
            ButtonVariant::Ghost => ButtonTokens::ghost(&theme),
            ButtonVariant::Link => ButtonTokens::link(&theme),
        };

        let height = self.size.height();
        let icon_size = self.icon_px.unwrap_or(self.size.icon_size());
        let gap = core::SPACE_2;
        let is_icon_only = self.icon_only;
        let has_text = !is_icon_only && !self.label.is_empty();

        // Foreground baked into the content galley (dimmed when disabled).
        let fg = if self.enabled {
            bt.foreground
        } else {
            core::disabled_color(bt.foreground)
        };

        // Content as a *single* galley: icon(s) + label on one baseline, each section
        // vertically centered (valign) so mixed icon/text fonts align optically —
        // centering two separate labels by their cells does not.
        let fmt = |font: egui::FontId, underline: bool, tracking: f32| egui::text::TextFormat {
            font_id: font,
            color: fg,
            valign: egui::Align::Center,
            extra_letter_spacing: tracking,
            underline: if underline {
                Stroke::new(core::BORDER_THIN, fg)
            } else {
                Stroke::NONE
            },
            ..Default::default()
        };
        let label_style = self.size.text_style();
        let icon_font_id = typography::icon_font(icon_size);
        let mut job = egui::text::LayoutJob::default();
        job.wrap.max_width = f32::INFINITY;
        let mut placed = false;
        if let Some(g) = self.icon_left {
            job.append(
                g,
                0.0,
                fmt(icon_font_id.clone(), false, core::TRACKING_NORMAL),
            );
            placed = true;
        }
        if has_text {
            let lead = if placed { gap } else { 0.0 };
            job.append(
                &self.label,
                lead,
                fmt(label_style.font_id(), bt.underline, label_style.tracking),
            );
            placed = true;
        }
        if let Some(g) = self.icon_right {
            let lead = if placed { gap } else { 0.0 };
            job.append(g, lead, fmt(icon_font_id, false, core::TRACKING_NORMAL));
        }
        let galley = ui.painter().layout_job(job);
        let content_size = galley.size();

        let width = if is_icon_only {
            height
        } else {
            content_size.x + 2.0 * self.size.pad_x()
        };
        let interactive = self.enabled && !self.loading;
        let sense = if interactive {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, response) = ui.allocate_exact_size(vec2(width, height), sense);
        let anim_id = self.id_source.unwrap_or(response.id);

        // Accessibility: expose a Button node with the label.
        let info_label = self.label.clone();
        let info_enabled = self.enabled;
        response.widget_info(move || {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, info_enabled, info_label.clone())
        });

        // State.
        let hovered = interactive && response.hovered();
        let down = interactive && response.is_pointer_button_down_on();
        let hover_t = core::hover_t(ui.ctx(), anim_id, hovered);

        let dim = |c: Color32| {
            if self.enabled {
                c
            } else {
                core::disabled_color(c)
            }
        };
        let radius = CornerRadius::same(bt.radius as u8);
        let painter = ui.painter().clone();

        painter.rect_filled(rect, radius, dim(bt.fill));
        if hover_t > 0.0 {
            painter.rect_filled(rect, radius, theme.hover_overlay.gamma_multiply(hover_t));
        }
        if down {
            painter.rect_filled(rect, radius, theme.press_overlay);
        }
        if bt.border.a() > 0 {
            painter.rect_stroke(
                rect,
                radius,
                Stroke::new(core::BORDER_THIN, dim(bt.border)),
                StrokeKind::Inside,
            );
        }
        if response.has_focus() {
            super::focus::focus_ring_rect(&painter, rect, radius, theme.ring);
        }

        if self.loading {
            // Indeterminate arc (same form as the Spinner atom), centered; dimmed fg.
            let spin = core::disabled_color(fg);
            let center = rect.center();
            let r = icon_size / 2.0 - core::BORDER_FOCUS;
            let start = ui.input(|i| i.time) as f32 * std::f32::consts::TAU;
            let sweep = 0.75 * std::f32::consts::TAU;
            let n = 32;
            let pts: Vec<_> = (0..=n)
                .map(|i| center + Vec2::angled(start + sweep * (i as f32 / n as f32)) * r)
                .collect();
            painter.add(Shape::line(pts, Stroke::new(core::BORDER_FOCUS, spin)));
            ui.ctx().request_repaint();
        } else {
            // Content galley, centered in the button.
            let pos = rect.center() - content_size * 0.5;
            painter.galley(pos, galley, fg);
        }

        response
    }
}
