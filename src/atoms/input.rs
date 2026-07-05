//! Input atom — a single-line text field over a `&mut String`.
//!
//! Token-painted box (fill `muted`, border `input`/`destructive`, focus ring) wrapping a
//! frameless [`egui::TextEdit`] (egui owns the editing; the casing is all token). States:
//! default / focus / disabled / error. *(Size/leading-icon belong to a Field molecule.)*

use crate::theme::typography;
use crate::tokens::core::{self, Size};
use crate::tokens::layout;
use crate::Theme;
use egui::{
    vec2, Align, Color32, CornerRadius, Id, Layout, Response, RichText, Sense, Stroke, StrokeKind,
    TextEdit, Ui, UiBuilder,
};

/// A single-line input bound to a `&mut String`. Builder; `show` returns the [`Response`]
/// (`changed` when the text was edited).
pub struct Input<'a> {
    buf: &'a mut String,
    placeholder: Option<String>,
    error: bool,
    enabled: bool,
    size: Size,
    id_source: Option<Id>,
}

impl<'a> Input<'a> {
    pub fn new(buf: &'a mut String) -> Self {
        Self {
            buf,
            placeholder: None,
            error: false,
            enabled: true,
            size: Size::default(),
            id_source: None,
        }
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
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
        let height = self.size.height();
        let pad_x = self.size.pad_x();
        // Fill the panel, but never shrink below the intrinsic floor (text needs room,
        // so there is deliberately no cap).
        let width = ui.available_width().max(layout::INPUT_MIN_W);

        let (rect, box_resp) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        let dim = |c: Color32| {
            if self.enabled {
                c
            } else {
                core::disabled_color(c)
            }
        };
        let radius = CornerRadius::same(core::RADIUS_MD as u8);
        let painter = ui.painter().clone();
        painter.rect_filled(rect, radius, dim(theme.muted));

        // Animated hover veil (same treatment as Button) — gated on enabled.
        let hovered = self.enabled && ui.rect_contains_pointer(rect);
        let ht = core::hover_t(ui.ctx(), box_resp.id, hovered);
        if ht > 0.0 {
            painter.rect_filled(rect, radius, theme.hover_overlay.gamma_multiply(ht));
        }

        let inner = rect.shrink2(vec2(pad_x, 0.0));
        let body = typography::body();
        let hint = RichText::new(self.placeholder.unwrap_or_default())
            .font(body.font_id())
            .extra_letter_spacing(body.tracking)
            .color(theme.muted_foreground);
        let mut cui = ui.new_child(
            UiBuilder::new()
                .max_rect(inner)
                .layout(Layout::left_to_right(Align::Center)),
        );
        let edit = TextEdit::singleline(self.buf)
            .frame(egui::Frame::NONE)
            .hint_text(hint)
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
