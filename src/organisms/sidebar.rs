//! Sidebar organism — a vertical navigation list. [shadcn Sidebar / Navigation Menu]

use crate::atoms::{Button, ButtonVariant};
use crate::cells::ListItem;
use egui::{Response, Ui};

/// A nav list bound to a `&mut usize`. `show` composes one [`ListItem`] per entry; `.icons_only()`
/// collapses it to an icon rail.
pub struct Sidebar<'a> {
    selected: &'a mut usize,
    items: Vec<(Option<&'static str>, String)>,
    icons_only: bool,
    icon_px: Option<f32>,
}

impl<'a> Sidebar<'a> {
    pub fn new(selected: &'a mut usize) -> Self {
        Self {
            selected,
            items: Vec::new(),
            icons_only: false,
            icon_px: None,
        }
    }
    pub fn item(mut self, icon: &'static str, label: impl Into<String>) -> Self {
        self.items.push((Some(icon), label.into()));
        self
    }
    pub fn text_item(mut self, label: impl Into<String>) -> Self {
        self.items.push((None, label.into()));
        self
    }
    /// Collapse to an icon-only rail (items without an icon fall back to their first glyph-less button).
    pub fn icons_only(mut self) -> Self {
        self.icons_only = true;
        self
    }
    /// Override the rail glyph box size (px) in [`Self::icons_only`] mode. Defaults to the
    /// button's icon size; pass e.g. `core::ICON_XL` (24px) for a larger rail. Additive —
    /// no effect on the default (non-icon-only) sidebar.
    pub fn icon_size(mut self, px: f32) -> Self {
        self.icon_px = Some(px);
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let selected = self.selected;
        let items = self.items;
        let icons_only = self.icons_only;
        let icon_px = self.icon_px;
        ui.vertical(|ui| {
            for (i, (icon, label)) in items.into_iter().enumerate() {
                let active = *selected == i;
                if icons_only {
                    // Active icon-rail item reads as a selected toggle: primary fill (TEAL_200)
                    // + dark on-primary glyph. Distinct from the muted Secondary it used to be.
                    let variant = if active {
                        ButtonVariant::Default
                    } else {
                        ButtonVariant::Ghost
                    };
                    let mut button = Button::new("")
                        .icon_only()
                        .variant(variant)
                        .id_source(("sidebar_icon", i));
                    if let Some(px) = icon_px {
                        button = button.icon_px(px);
                    }
                    if let Some(glyph) = icon {
                        button = button.icon_left(glyph);
                    }
                    if button.show(ui).clicked() {
                        *selected = i;
                    }
                } else {
                    let mut item = ListItem::new(label)
                        .selected(active)
                        .id_source(("sidebar", i));
                    if let Some(glyph) = icon {
                        item = item.icon(glyph);
                    }
                    if item.show(ui).clicked() {
                        *selected = i;
                    }
                }
            }
        })
        .response
    }
}
