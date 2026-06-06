//! Button atom — labeled, optionally-iconed click control with shadcn variants.
//!
//! Icon + label render as a **single galley** (each section `valign`-centered) so the icon
//! and text share one baseline and align optically — composing two separate labels centers
//! them by cell, which looks off. Fonts come from `typography`, fill/foreground/border from
//! [`ButtonTokens`], and hover/press/disabled/focus from motion/opacity/border tokens, so
//! light+dark and every state stay token-driven. (Atoms may paint; only molecules/organisms
//! must compose.)

use crate::theme::typography::{self, TypeStyle};
use crate::tokens::component::ButtonTokens;
use crate::tokens::core;
use crate::Theme;
use egui::{vec2, Color32, CornerRadius, Id, Response, Sense, Stroke, StrokeKind, Ui};

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

/// Button size. `Icon` is a square, icon-only button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
    Icon,
}

impl ButtonSize {
    fn height(self) -> f32 {
        match self {
            ButtonSize::Sm => core::CONTROL_SM,
            ButtonSize::Md | ButtonSize::Icon => core::CONTROL_MD,
            ButtonSize::Lg => core::CONTROL_LG,
        }
    }
    fn icon_size(self) -> f32 {
        match self {
            ButtonSize::Sm => core::ICON_SM,
            ButtonSize::Lg => core::ICON_LG,
            _ => core::ICON_MD,
        }
    }
    fn pad_x(self) -> f32 {
        match self {
            ButtonSize::Sm => core::SPACE_3,
            _ => core::SPACE_4,
        }
    }
    fn text_style(self) -> TypeStyle {
        match self {
            ButtonSize::Lg => typography::body_strong(),
            _ => typography::label(),
        }
    }
}

/// A click button. Builder; `show` returns the [`Response`] (`clicked`, `hovered`, …).
pub struct Button {
    label: String,
    variant: ButtonVariant,
    size: ButtonSize,
    icon_left: Option<&'static str>,
    icon_right: Option<&'static str>,
    enabled: bool,
    id_source: Option<Id>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::default(),
            size: ButtonSize::default(),
            icon_left: None,
            icon_right: None,
            enabled: true,
            id_source: None,
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

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }
    pub fn sm(self) -> Self {
        self.size(ButtonSize::Sm)
    }
    pub fn lg(self) -> Self {
        self.size(ButtonSize::Lg)
    }
    pub fn icon_only(self) -> Self {
        self.size(ButtonSize::Icon)
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
        let icon_size = self.size.icon_size();
        let gap = core::SPACE_2;
        let is_icon_only = matches!(self.size, ButtonSize::Icon);
        let has_text = !is_icon_only && !self.label.is_empty();

        // Foreground baked into the content galley (dimmed when disabled).
        let fg = if self.enabled {
            bt.foreground
        } else {
            bt.foreground.gamma_multiply(core::OPACITY_DISABLED)
        };

        // Content as a *single* galley: icon(s) + label on one baseline, each section
        // vertically centered (valign) so mixed icon/text fonts align optically —
        // centering two separate labels by their cells does not.
        let fmt = |font: egui::FontId, underline: bool| egui::text::TextFormat {
            font_id: font,
            color: fg,
            valign: egui::Align::Center,
            underline: if underline {
                Stroke::new(core::BORDER_THIN, fg)
            } else {
                Stroke::NONE
            },
            ..Default::default()
        };
        let icon_font_id = typography::icon_font(icon_size);
        let mut job = egui::text::LayoutJob::default();
        job.wrap.max_width = f32::INFINITY;
        let mut placed = false;
        if let Some(g) = self.icon_left {
            job.append(g, 0.0, fmt(icon_font_id.clone(), false));
            placed = true;
        }
        if has_text {
            let lead = if placed { gap } else { 0.0 };
            job.append(
                &self.label,
                lead,
                fmt(self.size.text_style().font_id(), bt.underline),
            );
            placed = true;
        }
        if let Some(g) = self.icon_right {
            let lead = if placed { gap } else { 0.0 };
            job.append(g, lead, fmt(icon_font_id, false));
        }
        let galley = ui.painter().layout_job(job);
        let content_size = galley.size();

        let width = if is_icon_only {
            height
        } else {
            content_size.x + 2.0 * self.size.pad_x()
        };
        let sense = if self.enabled {
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
        let hovered = self.enabled && response.hovered();
        let down = self.enabled && response.is_pointer_button_down_on();
        let raw =
            ui.ctx()
                .animate_bool_with_time(anim_id.with("hover"), hovered, core::DURATION_FAST);
        let hover_t = core::Easing::EaseOut.apply(raw);

        let dim = |c: Color32| {
            if self.enabled {
                c
            } else {
                c.gamma_multiply(core::OPACITY_DISABLED)
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
            painter.rect_stroke(
                rect.expand(2.0),
                radius,
                Stroke::new(core::BORDER_FOCUS, theme.ring),
                StrokeKind::Outside,
            );
        }

        // Content galley, centered in the button.
        let pos = rect.center() - content_size * 0.5;
        painter.galley(pos, galley, fg);

        response
    }
}
