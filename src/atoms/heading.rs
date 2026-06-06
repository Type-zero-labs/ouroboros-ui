//! Heading atom — a title at a heading level, in the `foreground` token.
//!
//! Font/size/line-height come from a [`typography`](crate::theme::typography) type style.

use crate::theme::typography::{self, TypeStyle};
use crate::Theme;
use egui::{Label, Response, RichText, TextWrapMode, Ui};

/// Heading level, mapped onto a foundation type style.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HeadingLevel {
    /// Largest display title.
    Display,
    H1,
    #[default]
    H2,
    /// Smallest section heading.
    Heading,
}

impl HeadingLevel {
    fn style(self) -> TypeStyle {
        match self {
            HeadingLevel::Display => typography::display(),
            HeadingLevel::H1 => typography::h1(),
            HeadingLevel::H2 => typography::h2(),
            HeadingLevel::Heading => typography::heading(),
        }
    }
}

/// A heading at a [`HeadingLevel`]. Builder; `show` returns the [`Response`].
pub struct Heading {
    content: String,
    level: HeadingLevel,
}

impl Heading {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            level: HeadingLevel::default(),
        }
    }

    pub fn level(mut self, level: HeadingLevel) -> Self {
        self.level = level;
        self
    }
    pub fn display(self) -> Self {
        self.level(HeadingLevel::Display)
    }
    pub fn h1(self) -> Self {
        self.level(HeadingLevel::H1)
    }
    pub fn h2(self) -> Self {
        self.level(HeadingLevel::H2)
    }
    pub fn heading(self) -> Self {
        self.level(HeadingLevel::Heading)
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let style = self.level.style();
        let rich = RichText::new(self.content)
            .font(style.font_id())
            .line_height(Some(style.line_height))
            .color(theme.foreground);
        ui.add(
            Label::new(rich)
                .selectable(false)
                .wrap_mode(TextWrapMode::Extend),
        )
    }
}
