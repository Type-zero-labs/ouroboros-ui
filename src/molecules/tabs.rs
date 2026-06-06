//! Tabs molecule — a single-select tab bar. [shadcn Tabs / Unity Tab]
//!
//! Two looks: `Container` (default — segmented chips inside a [`Surface`], so it doesn't read
//! as loose buttons) and `Line` (underlined row). Per-tab icons supported.

use crate::atoms::{Button, ButtonVariant, Divider, Surface};
use crate::tokens::core;
use crate::Theme;
use egui::{Response, Ui};

/// Tab bar look.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsVariant {
    #[default]
    Container,
    Line,
}

/// A tab bar bound to a `&mut usize`. Each tab is `(label, optional icon)`.
pub struct Tabs<'a> {
    selected: &'a mut usize,
    tabs: Vec<(String, Option<&'static str>)>,
    variant: TabsVariant,
}

impl<'a> Tabs<'a> {
    pub fn new(selected: &'a mut usize) -> Self {
        Self {
            selected,
            tabs: Vec::new(),
            variant: TabsVariant::default(),
        }
    }
    pub fn tabs<S: Into<String>>(mut self, tabs: impl IntoIterator<Item = S>) -> Self {
        self.tabs = tabs.into_iter().map(|t| (t.into(), None)).collect();
        self
    }
    /// Add one tab with a leading icon.
    pub fn tab(mut self, label: impl Into<String>, icon: &'static str) -> Self {
        self.tabs.push((label.into(), Some(icon)));
        self
    }
    pub fn variant(mut self, variant: TabsVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn line(self) -> Self {
        self.variant(TabsVariant::Line)
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let selected = self.selected;
        let tabs = self.tabs;
        let primary = Theme::get(ui).primary;
        let button = |label: &str,
                      icon: Option<&'static str>,
                      active: bool,
                      ghost_active: bool,
                      i: usize| {
            let variant = if active && !ghost_active {
                ButtonVariant::Secondary
            } else {
                ButtonVariant::Ghost
            };
            let mut b = Button::new(label)
                .variant(variant)
                .sm()
                .id_source(("tab", i));
            if let Some(glyph) = icon {
                b = b.icon_left(glyph);
            }
            b
        };
        match self.variant {
            // Pill-in-muted-container (shadcn radix default): active tab raised.
            TabsVariant::Container => {
                Surface::new()
                    .muted()
                    .border_none()
                    .pad(core::SPACE_1)
                    .radius(core::RADIUS_MD)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            for (i, (label, icon)) in tabs.iter().enumerate() {
                                if button(label, *icon, *selected == i, false, i)
                                    .show(ui)
                                    .clicked()
                                {
                                    *selected = i;
                                }
                            }
                        });
                    })
                    .response
            }
            // Underline (shadcn radix `variant="line"`): ghost tabs, active gets a primary underline.
            TabsVariant::Line => {
                ui.horizontal(|ui| {
                    for (i, (label, icon)) in tabs.iter().enumerate() {
                        let active = *selected == i;
                        ui.vertical(|ui| {
                            if button(label, *icon, active, true, i).show(ui).clicked() {
                                *selected = i;
                            }
                            ui.add_space(core::SPACE_1);
                            if active {
                                Divider::horizontal().color(primary).thick().show(ui);
                            } else {
                                ui.add_space(core::BORDER_FOCUS);
                            }
                        });
                    }
                })
                .response
            }
        }
    }
}
