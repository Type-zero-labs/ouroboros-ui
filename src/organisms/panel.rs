//! Panel organism — a canonical docked panel: background, an optional flush edge border, an
//! optional header (title + action) and footer, and a scrollable, token-padded body.
//!
//! Mounts inside a rect (e.g. a [`Splitter`](super::splitter::Splitter) band via a child `Ui`).
//! Unlike the elevated, rounded [`Card`](crate::molecules::Card), a Panel is **flush** (no radius,
//! no shadow) with a single hairline on its docking edge — the studio inspector/properties chrome,
//! so panels stop hand-rolling `egui::Frame` margins + manually painted borders/headers.
//!
//! ```ignore
//! Panel::new("world_inspector")
//!     .left_edge()
//!     .title("Inspector")
//!     .show(ui, |ui| {
//!         ResponsiveRow::new("Name").show(ui, |ui| Input::new(&mut name).show(ui));
//!     });
//! ```

use crate::atoms::{Divider, Heading, Surface, SurfaceFill};
use crate::tokens::{core, layout};
use egui::{Align, Id, Layout, Response, Ui, UiBuilder};

type SlotFn<'a> = Box<dyn FnOnce(&mut Ui) + 'a>;

/// Which edge of the panel carries the flush hairline border (the side it docks against). The
/// border is composed from a [`Divider`] atom carved off the panel rect — never hand-painted.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PanelEdge {
    #[default]
    None,
    Left,
    Right,
    Top,
    Bottom,
}

/// A docked panel. Builder; `show` paints the chrome and runs `content` in the padded body.
pub struct Panel<'a> {
    id: Id,
    title: Option<String>,
    action: Option<SlotFn<'a>>,
    footer: Option<SlotFn<'a>>,
    edge: PanelEdge,
    fill: SurfaceFill,
    scroll: bool,
    body_pad: f32,
}

impl<'a> Panel<'a> {
    pub fn new(id: impl std::hash::Hash) -> Self {
        Self {
            id: Id::new(id),
            title: None,
            action: None,
            footer: None,
            edge: PanelEdge::None,
            fill: SurfaceFill::Background,
            scroll: true,
            body_pad: layout::PANEL_PAD,
        }
    }

    /// Header title (a [`Heading`]); shown above a full-width divider.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    /// Top-right header slot (a button / menu / badge). [shadcn CardAction]
    pub fn action(mut self, action: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.action = Some(Box::new(action));
        self
    }
    /// Footer action bar, pinned to the panel's bottom above a full-width divider.
    pub fn footer(mut self, footer: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.footer = Some(Box::new(footer));
        self
    }
    pub fn edge(mut self, edge: PanelEdge) -> Self {
        self.edge = edge;
        self
    }
    /// Border on the left edge (a right-docked inspector).
    pub fn left_edge(self) -> Self {
        self.edge(PanelEdge::Left)
    }
    /// Border on the right edge (a left-docked panel).
    pub fn right_edge(self) -> Self {
        self.edge(PanelEdge::Right)
    }
    /// Background fill (default [`SurfaceFill::Background`]; `None` leaves the module's own bg).
    pub fn fill(mut self, fill: SurfaceFill) -> Self {
        self.fill = fill;
        self
    }
    /// Don't wrap the body in a `ScrollArea` (the content manages its own height).
    pub fn no_scroll(mut self) -> Self {
        self.scroll = false;
        self
    }
    /// Body inner padding (default [`layout::PANEL_PAD`]). Pass `0.0` for a **flush** body when the
    /// content manages its own insets (e.g. full-bleed accordion section headers).
    pub fn body_pad(mut self, px: f32) -> Self {
        self.body_pad = px;
        self
    }

    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Response {
        // Fill the whole mounted rect: the body's `set_min_size` pins the Surface frame to it.
        let full = ui.available_size();
        let fill = self.fill;
        Surface::new()
            .fill(fill)
            .border_none()
            .radius(0.0)
            .pad(0.0)
            .show(ui, |ui| {
                ui.set_min_size(full);
                self.body(ui, content);
            })
            .response
    }

    fn body(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
        // Carve the edge hairline off the panel rect, then stack header/body/footer in the rest.
        let rect = ui.max_rect();
        let w = core::BORDER_THIN;
        let (edge_rect, body_rect, edge_vertical) = match self.edge {
            PanelEdge::None => (None, rect, false),
            PanelEdge::Left => {
                let (e, b) = rect.split_left_right_at_x(rect.left() + w);
                (Some(e), b, true)
            }
            PanelEdge::Right => {
                let (b, e) = rect.split_left_right_at_x(rect.right() - w);
                (Some(e), b, true)
            }
            PanelEdge::Top => {
                let (e, b) = rect.split_top_bottom_at_y(rect.top() + w);
                (Some(e), b, false)
            }
            PanelEdge::Bottom => {
                let (b, e) = rect.split_top_bottom_at_y(rect.bottom() - w);
                (Some(e), b, false)
            }
        };

        if let Some(er) = edge_rect {
            let mut eui = ui.new_child(UiBuilder::new().max_rect(er));
            if edge_vertical {
                Divider::vertical().show(&mut eui);
            } else {
                Divider::horizontal().show(&mut eui);
            }
        }

        let Panel {
            id,
            title,
            action,
            footer,
            scroll,
            body_pad,
            ..
        } = self;

        // Reserve a fixed bottom band for the footer (a one-row action bar) so the scrolling body
        // fills the remainder and never overlaps it.
        let (content_rect, footer_rect) = if footer.is_some() {
            let fh = layout::PANEL_PAD * 2.0 + core::CONTROL_MD;
            let split_y = (body_rect.bottom() - fh).max(body_rect.top());
            let (c, f) = body_rect.split_top_bottom_at_y(split_y);
            (c, Some(f))
        } else {
            (body_rect, None)
        };

        // Header + scrollable body in the content rect. Inner surfaces are chrome-less
        // (`fill_none` + `border_none`): padding only, no nested box/border/radius.
        let mut bui = ui.new_child(
            UiBuilder::new()
                .max_rect(content_rect)
                .layout(Layout::top_down(Align::Min)),
        );
        if title.is_some() || action.is_some() {
            Surface::new()
                .fill_none()
                .border_none()
                .pad(layout::PANEL_PAD)
                .show(&mut bui, |ui| {
                    ui.horizontal(|ui| {
                        if let Some(title) = title {
                            Heading::new(title).heading().show(ui);
                        }
                        if let Some(action) = action {
                            ui.with_layout(Layout::right_to_left(Align::Center), action);
                        }
                    });
                });
            Divider::horizontal().show(&mut bui);
        }
        let inner = move |ui: &mut Ui| {
            Surface::new()
                .fill_none()
                .border_none()
                .pad(body_pad)
                .show(ui, |ui| {
                    // Port of the studio `panel_body`: pin the content width to the available
                    // width and keep horizontal auto-shrink ON so a fill control doesn't ratchet
                    // the width on resize (egui #1297); consistent row gap inside.
                    ui.set_min_width(ui.available_width());
                    ui.spacing_mut().item_spacing.y = layout::PANEL_GAP;
                    content(ui);
                });
        };
        if scroll {
            egui::ScrollArea::vertical()
                .id_salt(id)
                .auto_shrink([true, false])
                .show(&mut bui, inner);
        } else {
            inner(&mut bui);
        }

        // Footer band: a full-width divider on top, then the action bar.
        if let (Some(footer), Some(fr)) = (footer, footer_rect) {
            let mut fui = ui.new_child(
                UiBuilder::new()
                    .max_rect(fr)
                    .layout(Layout::top_down(Align::Min)),
            );
            Divider::horizontal().show(&mut fui);
            Surface::new()
                .fill_none()
                .border_none()
                .pad(layout::PANEL_PAD)
                .show(&mut fui, |ui| {
                    ui.with_layout(Layout::left_to_right(Align::Center), footer);
                });
        }
    }
}
