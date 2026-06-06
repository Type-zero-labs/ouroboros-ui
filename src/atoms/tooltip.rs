//! Tooltip atom — a DS-styled hover tooltip attached to a [`Response`].
//!
//! `Tooltip::new("…").show(response)` shows the text on hover, composing the [`Text`] atom.

use crate::atoms::Text;
use egui::Response;

/// A hover tooltip. Builder; `show` attaches it to `response` and returns it back.
pub struct Tooltip {
    text: String,
}

impl Tooltip {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    /// Attach the tooltip to `response` (shown on hover). Returns the response for chaining.
    pub fn show(self, response: Response) -> Response {
        let text = self.text;
        response.on_hover_ui(move |ui| {
            Text::new(text).show(ui);
        })
    }
}
