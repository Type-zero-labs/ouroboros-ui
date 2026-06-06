//! InputGroup molecule — an input with leading/trailing icon addons in one [`Surface`].
//!
//! Composes a [`Surface`] (the shared box) + [`Icon`] addons + a frameless `TextEdit`
//! (the editing substrate). The single-atom [`Input`](crate::atoms::Input) is for a plain
//! field; the group shares one box across addons.

use crate::atoms::{Icon, Surface};
use crate::theme::typography;
use crate::tokens::core;
use crate::Theme;
use egui::{Id, Response, RichText, TextEdit, Ui};

/// An input with optional leading/trailing icons. `show` returns the `TextEdit` [`Response`].
pub struct InputGroup<'a> {
    buf: &'a mut String,
    placeholder: Option<String>,
    leading: Option<&'static str>,
    trailing: Option<&'static str>,
    id_source: Option<Id>,
}

impl<'a> InputGroup<'a> {
    pub fn new(buf: &'a mut String) -> Self {
        Self {
            buf,
            placeholder: None,
            leading: None,
            trailing: None,
            id_source: None,
        }
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }
    pub fn leading(mut self, glyph: &'static str) -> Self {
        self.leading = Some(glyph);
        self
    }
    pub fn trailing(mut self, glyph: &'static str) -> Self {
        self.trailing = Some(glyph);
        self
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id_source = Some(Id::new(id));
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let body = typography::body();
        let hint = RichText::new(self.placeholder.unwrap_or_default())
            .font(body.font_id())
            .color(theme.muted_foreground);
        let buf = self.buf;
        let leading = self.leading;
        let trailing = self.trailing;

        Surface::new()
            .muted()
            .pad(core::SPACE_2)
            .radius(core::RADIUS_MD)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if let Some(glyph) = leading {
                        Icon::new(glyph).muted().show(ui);
                        ui.add_space(core::SPACE_2);
                    }
                    let reserve = if trailing.is_some() {
                        core::ICON_MD + core::SPACE_2
                    } else {
                        0.0
                    };
                    let width = (ui.available_width() - reserve).max(0.0);
                    let mut edit = TextEdit::singleline(buf)
                        .frame(egui::Frame::NONE)
                        .hint_text(hint)
                        .font(body.font_id())
                        .text_color(theme.foreground)
                        .desired_width(width);
                    if let Some(id) = self.id_source {
                        edit = edit.id(id);
                    }
                    let response = ui.add(edit);
                    if let Some(glyph) = trailing {
                        ui.add_space(core::SPACE_2);
                        Icon::new(glyph).muted().show(ui);
                    }
                    response
                })
                .inner
            })
            .inner
    }
}
