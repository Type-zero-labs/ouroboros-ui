//! Avatar atom — a circle with initials.
//!
//! Token circle (`muted`) + centered initials (a token type style). Image loading is a later
//! addition; this wave is initials-only.

use crate::theme::typography::{self, TypeStyle};
use crate::tokens::core;
use crate::Theme;
use egui::{vec2, Response, Sense, Ui};

/// Avatar size.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl AvatarSize {
    fn diameter(self) -> f32 {
        match self {
            AvatarSize::Sm => core::CONTROL_SM,
            AvatarSize::Md => core::CONTROL_MD,
            AvatarSize::Lg => core::CONTROL_LG,
        }
    }
    fn style(self) -> TypeStyle {
        match self {
            AvatarSize::Sm => typography::caption(),
            AvatarSize::Md => typography::label(),
            AvatarSize::Lg => typography::body_strong(),
        }
    }
}

/// A circular avatar with initials. Builder; `show` returns the [`Response`].
pub struct Avatar {
    initials: String,
    size: AvatarSize,
}

impl Avatar {
    pub fn new(initials: impl Into<String>) -> Self {
        Self {
            initials: initials.into(),
            size: AvatarSize::default(),
        }
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }
    pub fn sm(self) -> Self {
        self.size(AvatarSize::Sm)
    }
    pub fn lg(self) -> Self {
        self.size(AvatarSize::Lg)
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let d = self.size.diameter();
        let (rect, response) = ui.allocate_exact_size(vec2(d, d), Sense::hover());

        let painter = ui.painter().clone();
        painter.circle_filled(rect.center(), d / 2.0, theme.muted);

        let galley = painter.layout_no_wrap(
            self.initials.to_uppercase(),
            self.size.style().font_id(),
            theme.foreground,
        );
        painter.galley(
            rect.center() - galley.size() * 0.5,
            galley,
            theme.foreground,
        );

        response
    }
}
