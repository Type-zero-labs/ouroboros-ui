//! Autocomplete organism — a search field whose filtered matches appear as a clickable
//! result list below it ("add component" style). [shadcn Combobox / Command]
//!
//! MVP: results render inline under the field (no floating popup); each match is a
//! [`MenuItem`](crate::cells::MenuItem) with a "click to add" tooltip. [`show`](Autocomplete::show)
//! filters the provided labels by case-insensitive substring and returns the index — into the
//! original `items` — of the row clicked this frame.

use crate::atoms::{Surface, Text};
use crate::cells::MenuItem;
use crate::molecules::SearchField;
use crate::tokens::core;
use egui::{Id, Ui};

/// A filtered, clickable search list. Bind a query `String` and pass the full candidate label
/// list; `show` returns the picked original index.
pub struct Autocomplete<'a> {
    query: &'a mut String,
    items: Vec<String>,
    placeholder: Option<String>,
    max_results: usize,
    id_source: Id,
}

impl<'a> Autocomplete<'a> {
    pub fn new<S: Into<String>>(query: &'a mut String, items: impl IntoIterator<Item = S>) -> Self {
        Self {
            query,
            items: items.into_iter().map(Into::into).collect(),
            placeholder: None,
            max_results: 50,
            id_source: Id::new("autocomplete"),
        }
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    /// Cap the number of result rows rendered (default 50).
    pub fn max_results(mut self, n: usize) -> Self {
        self.max_results = n;
        self
    }

    /// Disambiguate row ids when multiple autocompletes share a frame.
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id_source = Id::new(id);
        self
    }

    /// Render the field + filtered results. Returns the index (into `items`) of the row
    /// clicked this frame, if any. Empty query → no results shown, returns `None`.
    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        {
            let mut field = SearchField::new(&mut *self.query);
            if let Some(p) = &self.placeholder {
                field = field.placeholder(p.clone());
            }
            field.show(ui);
        }

        let query = self.query.trim().to_lowercase();
        if query.is_empty() {
            return None;
        }

        let items = self.items;
        let id_source = self.id_source;
        let max_results = self.max_results;
        let mut clicked = None;
        ui.add_space(core::SPACE_1);
        Surface::new().pad(core::SPACE_1).show(ui, |ui| {
            ui.set_width(ui.available_width());
            let mut shown = 0;
            for (i, label) in items.iter().enumerate() {
                if !label.to_lowercase().contains(&query) {
                    continue;
                }
                if shown >= max_results {
                    break;
                }
                shown += 1;
                let resp = MenuItem::new(label)
                    .id_source((id_source, i))
                    .show(ui)
                    .on_hover_text("click to add");
                if resp.clicked() {
                    clicked = Some(i);
                }
            }
            if shown == 0 {
                Text::new("No matches").muted().show(ui);
            }
        });
        clicked
    }
}
