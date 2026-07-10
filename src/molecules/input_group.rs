//! InputGroup molecule — an input/textarea with addons in one shared [`Surface`].
//!
//! Addons (icon / text / button — shadcn `InputGroupAddon`/`InputGroupText`/`InputGroupButton`)
//! sit in four slots: inline-start, inline-end, block-start, block-end. `.multiline(rows)`
//! switches the field to the [`Textarea`] atom. Composes atoms only (the `TextEdit` is the
//! editing substrate, not a paint call).

use crate::atoms::{Button, Icon, Surface, Text, Textarea};
use crate::theme::typography;
use crate::tokens::core;
use crate::Theme;
use egui::{vec2, Align, Id, Layout, Response, RichText, TextEdit, Ui};

/// Addon position in an [`InputGroup`]. [shadcn align: inline-start/end, block-start/end]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slot {
    LeadingInline,
    TrailingInline,
    BlockStart,
    BlockEnd,
}

enum AddonKind<'a> {
    Icon(&'static str),
    Text(String),
    Button {
        glyph: &'static str,
        action: Box<dyn FnMut() + 'a>,
    },
}

struct Addon<'a> {
    slot: Slot,
    kind: AddonKind<'a>,
}

/// An input with addons. `show` returns the field's [`Response`] (`changed` when edited).
pub struct InputGroup<'a> {
    buf: &'a mut String,
    placeholder: Option<String>,
    addons: Vec<Addon<'a>>,
    multiline: Option<usize>,
    id_source: Option<Id>,
}

impl<'a> InputGroup<'a> {
    pub fn new(buf: &'a mut String) -> Self {
        Self {
            buf,
            placeholder: None,
            addons: Vec::new(),
            multiline: None,
            id_source: None,
        }
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }
    /// Switch to a multi-line [`Textarea`] with `rows` rows (inline addons are ignored).
    pub fn multiline(mut self, rows: usize) -> Self {
        self.multiline = Some(rows.max(1));
        self
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id_source = Some(Id::new(id));
        self
    }

    pub fn icon(mut self, slot: Slot, glyph: &'static str) -> Self {
        self.addons.push(Addon {
            slot,
            kind: AddonKind::Icon(glyph),
        });
        self
    }
    pub fn text(mut self, slot: Slot, text: impl Into<String>) -> Self {
        self.addons.push(Addon {
            slot,
            kind: AddonKind::Text(text.into()),
        });
        self
    }
    pub fn button(mut self, slot: Slot, glyph: &'static str, action: impl FnMut() + 'a) -> Self {
        self.addons.push(Addon {
            slot,
            kind: AddonKind::Button {
                glyph,
                action: Box::new(action),
            },
        });
        self
    }
    // sugar
    pub fn leading_icon(self, glyph: &'static str) -> Self {
        self.icon(Slot::LeadingInline, glyph)
    }
    pub fn leading_text(self, text: impl Into<String>) -> Self {
        self.text(Slot::LeadingInline, text)
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let body = typography::body();
        let buf = self.buf;
        let mut addons = self.addons;
        let placeholder = self.placeholder.unwrap_or_default();
        let multiline = self.multiline;
        let id_source = self.id_source;

        Surface::new()
            .muted()
            .pad(core::SPACE_1)
            .radius(core::RADIUS_MD)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    slot_row(ui, &mut addons, Slot::BlockStart);
                    let response = if let Some(rows) = multiline {
                        let mut ta = Textarea::new(buf).rows(rows).placeholder(placeholder);
                        if let Some(id) = id_source {
                            ta = ta.id_source(id);
                        }
                        ta.show(ui)
                    } else {
                        // Fixed-height, vertically-centered inline row so the leading icon /
                        // placeholder / addons all sit on the same centerline.
                        let full = ui.available_width();
                        ui.allocate_ui_with_layout(
                            vec2(full, core::CONTROL_MD),
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                render_slot(ui, &mut addons, Slot::LeadingInline);
                                let reserve = addons
                                    .iter()
                                    .filter(|a| a.slot == Slot::TrailingInline)
                                    .count() as f32
                                    * (core::CONTROL_MD + core::SPACE_2);
                                let width = (ui.available_width() - reserve).max(0.0);
                                let hint = RichText::new(placeholder)
                                    .font(body.font_id())
                                    .extra_letter_spacing(body.tracking)
                                    .color(theme.muted_foreground);
                                let mut edit = TextEdit::singleline(buf)
                                    .frame(egui::Frame::NONE)
                                    .hint_text(hint)
                                    .font(body.font_id())
                                    .text_color(theme.foreground)
                                    .desired_width(width);
                                if let Some(id) = id_source {
                                    edit = edit.id(id);
                                }
                                let response = ui.add(edit);
                                render_slot(ui, &mut addons, Slot::TrailingInline);
                                response
                            },
                        )
                        .inner
                    };
                    slot_row(ui, &mut addons, Slot::BlockEnd);
                    response
                })
                .inner
            })
            .inner
    }
}

/// Render a block slot as its own horizontal row (only if it has addons).
fn slot_row(ui: &mut Ui, addons: &mut [Addon], slot: Slot) {
    if addons.iter().any(|a| a.slot == slot) {
        ui.horizontal(|ui| render_slot(ui, addons, slot));
        ui.add_space(core::SPACE_1);
    }
}

fn render_slot(ui: &mut Ui, addons: &mut [Addon], slot: Slot) {
    let mut first = true;
    for addon in addons.iter_mut().filter(|a| a.slot == slot) {
        if !first {
            ui.add_space(core::SPACE_2);
        }
        first = false;
        match &mut addon.kind {
            AddonKind::Icon(glyph) => {
                Icon::new(glyph).muted().show(ui);
            }
            AddonKind::Text(text) => {
                Text::new(text.clone()).muted().show(ui);
            }
            AddonKind::Button { glyph, action } => {
                if Button::new("")
                    .icon_left(glyph)
                    .icon_only()
                    .ghost()
                    .sm()
                    .show(ui)
                    .clicked()
                {
                    action();
                }
            }
        }
    }
}
