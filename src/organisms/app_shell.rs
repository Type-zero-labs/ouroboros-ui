//! AppShell organism — a view scaffold with named slots. [Element Plus Container]
//!
//! Five optional slots: `header` and `footer` span the full width (fixed heights); the middle
//! band holds `aside_left` · `main` · `aside_right` as a horizontal [`Splitter`], so the asides
//! are **drag-resizable** by default (header/footer stay fixed). Laid out by rect-math inside
//! the `ui` it's given (not `egui::SidePanel`), so it composes anywhere and **nests** — an
//! `AppShell` can live inside another's slot. Default dimensions come from `tokens::layout`.
//! Composes closures + the [`Splitter`](crate::organisms::Splitter) organism; never paints.
//!
//! ```ignore
//! AppShell::new()
//!     .header(|ui| toolbar(ui))
//!     .aside_left(|ui| hierarchy(ui))
//!     .main(|ui| viewport(ui))
//!     .aside_right(|ui| inspector(ui))
//!     .footer(|ui| status_bar(ui))
//!     .show(ui);
//! ```

use crate::organisms::{PanelSpec, Splitter};
use crate::tokens::layout;
use egui::{pos2, vec2, Id, Rect, Response, Sense, Ui, UiBuilder};

type Slot<'a> = Option<Box<dyn FnMut(&mut Ui) + 'a>>;

/// A slotted view scaffold. Build with [`AppShell::new`], set the slots you need, then
/// [`AppShell::show`].
pub struct AppShell<'a> {
    header: Slot<'a>,
    aside_left: Slot<'a>,
    main: Slot<'a>,
    aside_right: Slot<'a>,
    footer: Slot<'a>,
    header_height: f32,
    footer_height: f32,
    aside_left_width: f32,
    aside_right_width: f32,
    id_source: Option<Id>,
}

impl<'a> AppShell<'a> {
    pub fn new() -> Self {
        Self {
            header: None,
            aside_left: None,
            main: None,
            aside_right: None,
            footer: None,
            header_height: layout::TOOLBAR_HEIGHT,
            footer_height: layout::STATUSBAR_HEIGHT,
            aside_left_width: layout::SIDEBAR_WIDTH,
            aside_right_width: layout::INSPECTOR_WIDTH,
            id_source: None,
        }
    }

    pub fn header(mut self, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.header = Some(Box::new(add));
        self
    }
    pub fn aside_left(mut self, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.aside_left = Some(Box::new(add));
        self
    }
    pub fn main(mut self, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.main = Some(Box::new(add));
        self
    }
    pub fn aside_right(mut self, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.aside_right = Some(Box::new(add));
        self
    }
    pub fn footer(mut self, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.footer = Some(Box::new(add));
        self
    }

    pub fn header_height(mut self, px: f32) -> Self {
        self.header_height = px;
        self
    }
    pub fn footer_height(mut self, px: f32) -> Self {
        self.footer_height = px;
        self
    }
    pub fn aside_left_width(mut self, px: f32) -> Self {
        self.aside_left_width = px;
        self
    }
    pub fn aside_right_width(mut self, px: f32) -> Self {
        self.aside_right_width = px;
        self
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id_source = Some(Id::new(id));
        self
    }

    pub fn show(mut self, ui: &mut Ui) -> Response {
        let outer = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(outer, Sense::hover());
        let id = self.id_source.unwrap_or(response.id);

        let header_h = if self.header.is_some() {
            self.header_height
        } else {
            0.0
        };
        let footer_h = if self.footer.is_some() {
            self.footer_height
        } else {
            0.0
        };

        // Vertical bands: header (top, full width) · middle · footer (bottom, full width).
        let middle_top = rect.top() + header_h;
        let middle_h = (rect.height() - header_h - footer_h).max(0.0);
        let middle_bottom = middle_top + middle_h;

        let render_band = |ui: &mut Ui, cell: Rect, slot: &mut Slot<'a>| {
            if cell.width() <= 0.0 || cell.height() <= 0.0 {
                return;
            }
            if let Some(add) = slot.as_mut() {
                let mut cui = ui.new_child(UiBuilder::new().max_rect(cell));
                cui.set_clip_rect(cell);
                add(&mut cui);
            }
        };

        if header_h > 0.0 {
            let cell = Rect::from_min_size(rect.left_top(), vec2(rect.width(), header_h));
            render_band(ui, cell, &mut self.header);
        }
        if footer_h > 0.0 {
            let cell = Rect::from_min_size(
                pos2(rect.left(), middle_bottom),
                vec2(rect.width(), footer_h),
            );
            render_band(ui, cell, &mut self.footer);
        }

        // ── Middle band: aside_left | main | aside_right as a horizontal Splitter ──
        let middle = Rect::from_min_size(
            pos2(rect.left(), middle_top),
            vec2(rect.width(), middle_h),
        );
        if middle.width() > 0.0 && middle.height() > 0.0 {
            let middle_w = middle.width();
            let frac = |w: f32| (w / middle_w).clamp(0.05, 0.45);

            let mut splitter = Splitter::horizontal().id_source((id, "appshell_mid"));
            if let Some(add) = self.aside_left.take() {
                splitter = splitter.panel(
                    PanelSpec::new()
                        .size(frac(self.aside_left_width))
                        .min(layout::PANEL_MIN)
                        .max(layout::PANEL_MAX),
                    add,
                );
            }
            if let Some(add) = self.main.take() {
                splitter = splitter.panel(PanelSpec::new(), add);
            }
            if let Some(add) = self.aside_right.take() {
                splitter = splitter.panel(
                    PanelSpec::new()
                        .size(frac(self.aside_right_width))
                        .min(layout::PANEL_MIN)
                        .max(layout::PANEL_MAX),
                    add,
                );
            }

            let mut cui = ui.new_child(UiBuilder::new().max_rect(middle));
            cui.set_clip_rect(middle);
            splitter.show(&mut cui);
        }

        response
    }
}

impl Default for AppShell<'_> {
    fn default() -> Self {
        Self::new()
    }
}
