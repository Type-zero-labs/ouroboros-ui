//! Rail organism — the fixed vertical module bar of the shell. [Figma left rail]
//!
//! `RAIL_WIDTH` wide, full height: a logo slot on top (the caller draws it — it opens the app
//! menu), module items below with `RAIL_GAP` between them, and footer items pinned to the
//! bottom (console, status). A hairline on the right edge separates it from the side panel.
//! Clicking the **active** item again is reported as `reclicked` — the shell uses it to
//! collapse/reopen the side panel. The empty rail background is returned too, so the shell can
//! drag the (undecorated) window from it.

use crate::atoms::{Divider, RailButton};
use crate::tokens::{core, layout};
use egui::{pos2, vec2, Align, Layout, Rect, Response, Sense, Ui, UiBuilder};

/// One item on the rail.
pub struct RailItem {
    pub icon: &'static str,
    pub label: String,
    pub shortcut: Option<String>,
    pub badge: Option<u32>,
}

impl RailItem {
    pub fn new(icon: &'static str, label: impl Into<String>) -> Self {
        Self {
            icon,
            label: label.into(),
            shortcut: None,
            badge: None,
        }
    }
    pub fn shortcut(mut self, s: impl Into<String>) -> Self {
        self.shortcut = Some(s.into());
        self
    }
    pub fn badge(mut self, count: Option<u32>) -> Self {
        self.badge = count;
        self
    }
}

/// What happened on the rail this frame.
pub struct RailResponse {
    /// A different module was clicked (the new index is already in `active`).
    pub switched: bool,
    /// The active module was clicked again.
    pub reclicked: bool,
    /// Index of a footer item clicked this frame.
    pub footer_clicked: Option<usize>,
    /// The logo slot's response (the caller opens the app menu from it).
    pub logo: Option<Response>,
    /// The empty background (drag the window from here).
    pub background: Response,
}

type LogoFn<'a> = Box<dyn FnOnce(&mut Ui) -> Response + 'a>;

/// The module rail. Builder; `show` fills the given `ui` (give it a `RAIL_WIDTH` column).
pub struct Rail<'a> {
    active: &'a mut usize,
    items: Vec<RailItem>,
    footer: Vec<RailItem>,
    logo: Option<LogoFn<'a>>,
}

impl<'a> Rail<'a> {
    pub fn new(active: &'a mut usize) -> Self {
        Self {
            active,
            items: Vec::new(),
            footer: Vec::new(),
            logo: None,
        }
    }
    /// Draw the logo slot (`RAIL_SLOT` square at the top). Return its response.
    pub fn logo(mut self, logo: impl FnOnce(&mut Ui) -> Response + 'a) -> Self {
        self.logo = Some(Box::new(logo));
        self
    }
    pub fn item(mut self, item: RailItem) -> Self {
        self.items.push(item);
        self
    }
    /// An item pinned to the bottom (not a module: it never becomes active).
    pub fn footer(mut self, item: RailItem) -> Self {
        self.footer.push(item);
        self
    }

    pub fn show(self, ui: &mut Ui) -> RailResponse {
        let rect = ui.max_rect();
        // Background first: buttons registered after it win the clicks.
        let background = ui.interact(rect, ui.id().with("rail_bg"), Sense::click_and_drag());
        let inner = Rect::from_min_max(
            rect.min,
            pos2(rect.right() - core::BORDER_THIN, rect.bottom()),
        );
        let mut switched = false;
        let mut reclicked = false;
        let mut footer_clicked = None;
        let mut logo_resp = None;

        let mut col = ui.new_child(
            UiBuilder::new()
                .max_rect(inner)
                .layout(Layout::top_down(Align::Center)),
        );
        col.add_space(core::SPACE_2);
        if let Some(logo) = self.logo {
            logo_resp = Some(logo(&mut col));
        }
        col.add_space(core::SPACE_4);
        col.spacing_mut().item_spacing.y = layout::RAIL_GAP;
        for (i, item) in self.items.into_iter().enumerate() {
            let resp = RailButton::new(item.icon)
                .active(*self.active == i)
                .badge(item.badge)
                .tooltip(item.label, item.shortcut.as_deref())
                .id_source(("rail_item", i))
                .show(&mut col);
            if resp.clicked() {
                if *self.active == i {
                    reclicked = true;
                } else {
                    *self.active = i;
                    switched = true;
                }
            }
        }

        let mut foot = ui.new_child(
            UiBuilder::new()
                .max_rect(inner.shrink2(vec2(0.0, core::SPACE_2)))
                .layout(Layout::bottom_up(Align::Center)),
        );
        foot.spacing_mut().item_spacing.y = core::SPACE_2;
        for (i, item) in self.footer.into_iter().enumerate() {
            let resp = RailButton::new(item.icon)
                .size(layout::RAIL_SLOT)
                .badge(item.badge)
                .tooltip(item.label, item.shortcut.as_deref())
                .id_source(("rail_footer", i))
                .show(&mut foot);
            if resp.clicked() {
                footer_clicked = Some(i);
            }
        }

        let line = Rect::from_min_max(pos2(rect.right() - core::BORDER_THIN, rect.top()), rect.max);
        let mut lui = ui.new_child(UiBuilder::new().max_rect(line));
        Divider::vertical().show(&mut lui);

        RailResponse {
            switched,
            reclicked,
            footer_clicked,
            logo: logo_resp,
            background,
        }
    }
}
