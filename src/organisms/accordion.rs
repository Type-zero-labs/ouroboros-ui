//! Accordion organism — stacked collapsible sections. [shadcn Accordion]
//!
//! `show` gives a context whose `section(title, body)` adds a [`Collapsible`] divided from the
//! next. Each section keeps its own open state (egui memory).

use crate::atoms::{Divider, Surface};
use crate::molecules::Collapsible;
use crate::tokens::core;
use egui::{Response, Ui};

/// A group of collapsible sections. `.card()` wraps the group in a card [`Surface`]. Each
/// section's body is a free closure (put any content inside).
pub struct Accordion {
    card: bool,
}

/// Section builder handed to [`Accordion::show`].
pub struct AccordionCtx<'u> {
    ui: &'u mut Ui,
    first: bool,
}

impl AccordionCtx<'_> {
    pub fn section(&mut self, title: impl Into<String>, body: impl FnOnce(&mut Ui)) {
        if !self.first {
            self.ui.add_space(core::SPACE_2);
            Divider::horizontal().show(self.ui);
            self.ui.add_space(core::SPACE_2);
        }
        self.first = false;
        Collapsible::new(title).show(self.ui, body);
    }
}

impl Accordion {
    pub fn new() -> Self {
        Self { card: false }
    }
    /// Wrap the sections in a card surface.
    pub fn card(mut self) -> Self {
        self.card = true;
        self
    }
    pub fn show(self, ui: &mut Ui, build: impl FnOnce(&mut AccordionCtx)) -> Response {
        let run = |ui: &mut Ui| {
            let mut ctx = AccordionCtx { ui, first: true };
            build(&mut ctx);
        };
        if self.card {
            Surface::new()
                .show(ui, |ui| {
                    ui.vertical(run);
                })
                .response
        } else {
            ui.vertical(run).response
        }
    }
}

impl Default for Accordion {
    fn default() -> Self {
        Self::new()
    }
}
