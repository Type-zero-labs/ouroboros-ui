//! Text atom — a string rendered at a typography role with a theme color.
//!
//! Every visual comes from a token: the font/size/line-height from a
//! [`typography`](crate::theme::typography) type style, the color from the [`Theme`].

use crate::theme::typography::{self, TypeStyle};
use crate::Theme;
use egui::{Color32, Label, Response, RichText, TextWrapMode, Ui};

/// Typography role of a [`Text`], mapped onto a foundation type style.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextRole {
    #[default]
    Body,
    BodyStrong,
    Label,
    Caption,
    Code,
    Kbd,
}

impl TextRole {
    fn style(self) -> TypeStyle {
        match self {
            TextRole::Body => typography::body(),
            TextRole::BodyStrong => typography::body_strong(),
            TextRole::Label => typography::label(),
            TextRole::Caption => typography::caption(),
            TextRole::Code => typography::code(),
            TextRole::Kbd => typography::kbd(),
        }
    }
}

/// A run of text at a [`TextRole`]. Builder; `show` returns the [`Response`].
pub struct Text {
    content: String,
    role: TextRole,
    color: Option<Color32>,
    muted: bool,
    wrap: bool,
    underline: bool,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            role: TextRole::default(),
            color: None,
            muted: false,
            wrap: false,
            underline: false,
        }
    }

    pub fn role(mut self, role: TextRole) -> Self {
        self.role = role;
        self
    }
    pub fn body_strong(self) -> Self {
        self.role(TextRole::BodyStrong)
    }
    pub fn label(self) -> Self {
        self.role(TextRole::Label)
    }
    pub fn caption(self) -> Self {
        self.role(TextRole::Caption)
    }
    pub fn code(self) -> Self {
        self.role(TextRole::Code)
    }
    pub fn kbd(self) -> Self {
        self.role(TextRole::Kbd)
    }

    /// Render in the muted foreground token.
    pub fn muted(mut self) -> Self {
        self.muted = true;
        self
    }
    /// Override the color with an explicit token (e.g. `theme.success`).
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
    /// Wrap on the available width (default: extend / no wrap).
    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
    }
    /// Underline the text (e.g. a link).
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let color = self.color.unwrap_or(if self.muted {
            theme.muted_foreground
        } else {
            theme.foreground
        });
        let style = self.role.style();
        let mut rich = RichText::new(self.content)
            .font(style.font_id())
            .color(color);
        // Leading only matters for multi-line text; applying it to a single line inflates
        // the row and pushes the glyph off-center inside parents (e.g. a button).
        if self.wrap {
            rich = rich.line_height(Some(style.line_height));
        }
        if self.underline {
            rich = rich.underline();
        }
        let wrap_mode = if self.wrap {
            TextWrapMode::Wrap
        } else {
            TextWrapMode::Extend
        };
        // Non-selectable: UI text must not capture the pointer (it would steal clicks
        // from an interactive parent like a button and show a text cursor).
        ui.add(Label::new(rich).selectable(false).wrap_mode(wrap_mode))
    }
}
