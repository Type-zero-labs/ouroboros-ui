//! Icon atom — a Phosphor glyph at an icon-size token with a theme color.
//!
//! The glyph is a `&str` from [`egui_phosphor::light`](crate::egui_phosphor) (re-exported
//! at the crate root). Size comes from `core::ICON_*`; the font comes from
//! [`typography::icon_font`](crate::theme::typography::icon_font) — atoms never build a
//! `FontId` directly.

use crate::theme::typography;
use crate::tokens::core;
use crate::Theme;
use egui::{Color32, Label, Response, RichText, Ui};

/// A Phosphor glyph. Builder; `show` returns the [`Response`].
pub struct Icon {
    glyph: &'static str,
    size: f32,
    color: Option<Color32>,
    muted: bool,
}

impl Icon {
    /// `glyph` is a constant from `egui_phosphor::light` (e.g. `light::GEAR`).
    pub fn new(glyph: &'static str) -> Self {
        Self {
            glyph,
            size: core::ICON_MD,
            color: None,
            muted: false,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    pub fn sm(self) -> Self {
        self.size(core::ICON_SM)
    }
    pub fn md(self) -> Self {
        self.size(core::ICON_MD)
    }
    pub fn lg(self) -> Self {
        self.size(core::ICON_LG)
    }
    pub fn xl(self) -> Self {
        self.size(core::ICON_XL)
    }

    pub fn muted(mut self) -> Self {
        self.muted = true;
        self
    }
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let color = self.color.unwrap_or(if self.muted {
            theme.muted_foreground
        } else {
            theme.foreground
        });
        // Non-selectable so the glyph never steals a click from an interactive parent.
        ui.add(
            Label::new(
                RichText::new(self.glyph)
                    .font(typography::icon_font(self.size))
                    .color(color),
            )
            .selectable(false),
        )
    }
}
