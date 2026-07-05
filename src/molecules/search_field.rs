//! SearchField molecule — an [`InputGroup`] preset with a leading search icon. [Unity Search Field]

use crate::molecules::InputGroup;
use egui::{Response, Ui};
use egui_phosphor::light;

/// A search input. Composes [`InputGroup`] with a leading magnifier.
pub struct SearchField<'a> {
    buf: &'a mut String,
    placeholder: Option<String>,
}

impl<'a> SearchField<'a> {
    pub fn new(buf: &'a mut String) -> Self {
        Self {
            buf,
            placeholder: None,
        }
    }
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let mut group = InputGroup::new(self.buf).leading_icon(light::MAGNIFYING_GLASS);
        if let Some(placeholder) = self.placeholder {
            group = group.placeholder(placeholder);
        }
        group.show(ui)
    }
}
