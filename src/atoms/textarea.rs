//! Textarea atom — a multi-line text field over a `&mut String`.
//!
//! Sibling of [`Input`](crate::atoms::Input): token-painted box (fill `muted`, border
//! `input`/`destructive`, focus ring) wrapping a frameless multiline [`egui::TextEdit`].
//! Height derives from the requested row count.

use crate::theme::typography;
use crate::tokens::core;
use crate::Theme;
use egui::{
    vec2, Color32, CornerRadius, Id, Response, RichText, Sense, Stroke, StrokeKind, TextEdit, Ui,
    UiBuilder,
};

/// A multi-line input bound to a `&mut String`. Builder; `show` returns the [`Response`]
/// (`changed` when edited).
pub struct Textarea<'a> {
    buf: &'a mut String,
    placeholder: Option<String>,
    rows: usize,
    error: bool,
    enabled: bool,
    id_source: Option<Id>,
}

impl<'a> Textarea<'a> {
    pub fn new(buf: &'a mut String) -> Self {
        Self {
            buf,
            placeholder: None,
            rows: 3,
            error: false,
            enabled: true,
            id_source: None,
        }
    }

    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows.max(1);
        self
    }
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
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
        let body = typography::body();
        let pad = core::SPACE_2;
        let height = self.rows as f32 * body.line_height + 2.0 * pad;
        let width = ui.available_width();

        let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        let dim = |c: Color32| {
            if self.enabled {
                c
            } else {
                c.gamma_multiply(core::OPACITY_DISABLED)
            }
        };
        let radius = CornerRadius::same(core::RADIUS_MD as u8);
        let painter = ui.painter().clone();
        painter.rect_filled(rect, radius, dim(theme.muted));

        let inner = rect.shrink(pad);
        let hint = RichText::new(self.placeholder.unwrap_or_default())
            .font(body.font_id())
            .color(theme.muted_foreground);
        let mut cui = ui.new_child(UiBuilder::new().max_rect(inner));
        let edit = TextEdit::multiline(self.buf)
            .frame(egui::Frame::NONE)
            .hint_text(hint)
            .desired_rows(self.rows)
            .desired_width(inner.width())
            .font(body.font_id())
            .text_color(theme.foreground);
        let id = self.id_source.unwrap_or_else(|| cui.next_auto_id());
        let resp = cui.add_enabled(self.enabled, edit.id(id));

        let (border, w) = if self.error {
            (theme.destructive, core::BORDER_THIN)
        } else if resp.has_focus() {
            (theme.ring, core::BORDER_FOCUS)
        } else {
            (theme.input, core::BORDER_THIN)
        };
        painter.rect_stroke(
            rect,
            radius,
            Stroke::new(w, dim(border)),
            StrokeKind::Inside,
        );

        resp
    }
}
