//! Accordion organism — stacked collapsible sections. [shadcn Accordion]
//!
//! `show` gives a context whose `section(title, body)` adds a [`Collapsible`] divided from the
//! next. Each section keeps its own open state (egui memory).

use crate::atoms::Divider;
use crate::molecules::Collapsible;
use crate::tokens::core;
use egui::{Response, Ui};

/// A group of collapsible sections.
pub struct Accordion;

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
        Self
    }
    pub fn show(self, ui: &mut Ui, build: impl FnOnce(&mut AccordionCtx)) -> Response {
        ui.vertical(|ui| {
            let mut ctx = AccordionCtx { ui, first: true };
            build(&mut ctx);
        })
        .response
    }
}

impl Default for Accordion {
    fn default() -> Self {
        Self::new()
    }
}
