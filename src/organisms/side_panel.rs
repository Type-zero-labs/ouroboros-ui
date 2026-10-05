//! SidePanel organism — the module panel next to the rail. [Figma left panel]
//!
//! A `card` surface with a fixed header (`SIDE_HEADER_H`): the module title in lowercase (mono
//! `heading`), optional header actions and a collapse button (`‹`) on the right, then a compact
//! search field. The body scrolls below. The right edge is a resize grip: the owner keeps the
//! width (persist it) and the collapsed flag; this organism only edits them.

use crate::atoms::{Axis, Surface};
use crate::atoms::{Button, Heading, SplitterHandle};
use crate::molecules::SearchField;
use crate::tokens::{core, layout};
use egui::{pos2, Align, Layout, Rect, Response, Ui, UiBuilder};
use egui_phosphor::light;

type ActionFn<'a> = Box<dyn FnOnce(&mut Ui) + 'a>;

/// What happened on the panel this frame.
pub struct SidePanelResponse {
    /// The header strip (drag the window from here when it is the top of the window).
    pub header: Response,
    /// The width changed by dragging the grip (persist it).
    pub resized: bool,
    /// The collapse button was clicked (`collapsed` is already set).
    pub collapsed_now: bool,
}

/// The side panel. Builder; `show` fills `ui.max_rect()` (the owner sizes it to `width`).
pub struct SidePanel<'a> {
    id: egui::Id,
    title: String,
    action: Option<ActionFn<'a>>,
    search: Option<(&'a mut String, String)>,
    collapsed: Option<&'a mut bool>,
    width: Option<&'a mut f32>,
    scroll: bool,
}

impl<'a> SidePanel<'a> {
    pub fn new(id: impl std::hash::Hash) -> Self {
        Self {
            id: egui::Id::new(id),
            title: String::new(),
            action: None,
            search: None,
            collapsed: None,
            width: None,
            scroll: true,
        }
    }
    /// Give the body its plain rect instead of a scroll area — for bodies that lay out
    /// their own regions (a `Splitter` of tree + explorer) and scroll inside them.
    pub fn no_scroll(mut self) -> Self {
        self.scroll = false;
        self
    }
    /// Module title, shown in lowercase.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into().to_lowercase();
        self
    }
    /// Extra header buttons, left of the collapse button (e.g. `+ New`).
    pub fn action(mut self, action: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.action = Some(Box::new(action));
        self
    }
    pub fn search(mut self, buf: &'a mut String, placeholder: impl Into<String>) -> Self {
        self.search = Some((buf, placeholder.into()));
        self
    }
    /// Show the collapse button; clicking it sets `*collapsed = true`.
    pub fn collapsible(mut self, collapsed: &'a mut bool) -> Self {
        self.collapsed = Some(collapsed);
        self
    }
    /// Make the right edge a resize grip editing `width` (clamped to `SIDE_PANEL_MIN..MAX`).
    pub fn resizable(mut self, width: &'a mut f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn show(self, ui: &mut Ui, body: impl FnOnce(&mut Ui)) -> SidePanelResponse {
        let rect = ui.max_rect();
        let mut resized = false;
        let mut collapsed_now = false;
        // Sem busca, o cabeçalho é só a linha do título.
        let header_h = if self.search.is_some() {
            layout::SIDE_HEADER_H
        } else {
            layout::SIDE_HEADER_H - core::CONTROL_MD - core::SPACE_3
        };
        let header_rect = Rect::from_min_max(rect.min, pos2(rect.right(), rect.top() + header_h));
        let header = ui.interact(
            header_rect,
            self.id.with("header"),
            egui::Sense::click_and_drag(),
        );

        Surface::new()
            .border_none()
            .radius(0.0)
            .pad(0.0)
            .show(ui, |ui| {
                ui.set_min_size(rect.size());
                // ── header: title row + search ──
                let pad = core::SPACE_2;
                let mut h = ui.new_child(
                    UiBuilder::new()
                        .max_rect(header_rect.shrink2(egui::vec2(pad, 0.0)))
                        .layout(Layout::top_down(Align::Min)),
                );
                h.add_space(core::SPACE_3);
                h.horizontal(|ui| {
                    ui.set_height(core::CONTROL_MD);
                    Heading::new(self.title).heading().show(ui);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if let Some(collapsed) = self.collapsed {
                            if Button::new("")
                                .icon_only()
                                .neutral()
                                .icon_left(light::CARET_LEFT)
                                .id_source((self.id, "collapse"))
                                .show(ui)
                                .clicked()
                            {
                                *collapsed = true;
                                collapsed_now = true;
                            }
                        }
                        if let Some(action) = self.action {
                            action(ui);
                        }
                    });
                });
                if let Some((buf, placeholder)) = self.search {
                    h.add_space(core::SPACE_3);
                    SearchField::new(buf).placeholder(placeholder).show(&mut h);
                }

                // ── body ──
                let body_rect =
                    Rect::from_min_max(pos2(rect.left(), header_rect.bottom()), rect.max);
                let mut b = ui.new_child(
                    UiBuilder::new()
                        .max_rect(body_rect.shrink(core::SPACE_1))
                        .layout(Layout::top_down(Align::Min)),
                );
                if self.scroll {
                    egui::ScrollArea::vertical()
                        .id_salt(self.id.with("body"))
                        .auto_shrink([false, false])
                        .show(&mut b, body);
                } else {
                    body(&mut b);
                }
            });

        if let Some(width) = self.width {
            let grip = Rect::from_min_max(
                pos2(rect.right() - core::SPACE_1, rect.top()),
                pos2(rect.right() + core::SPACE_1, rect.bottom()),
            );
            let mut gui = ui.new_child(UiBuilder::new().max_rect(grip));
            let resp = SplitterHandle::new(Axis::Vertical).show(&mut gui);
            if resp.dragged() {
                let w = (*width + resp.drag_delta().x)
                    .clamp(layout::SIDE_PANEL_MIN, layout::SIDE_PANEL_MAX);
                if (w - *width).abs() > f32::EPSILON {
                    *width = w;
                    resized = true;
                }
            }
        }

        SidePanelResponse {
            header,
            resized,
            collapsed_now,
        }
    }
}
