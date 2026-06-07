//! NodeSearch — a command-palette popover for picking a node kind to create.
//!
//! Compose tier: a [`Popover`] holding an [`Input`] filter over caller-supplied node kinds, each
//! a [`MenuItem`]. Data-agnostic: the caller owns the kind list and decides where the chosen
//! kind is placed. Returns the picked [`NodeKindId`] (the caller then emits a `create_request`,
//! or just spawns the node itself).

use egui::Response;

use crate::atoms::Input;
use crate::cells::MenuItem;
use crate::organisms::Popover;

use super::NodeKindId;

/// A searchable list of node kinds, shown as a popover anchored to a trigger.
#[derive(Default)]
pub struct NodeSearch {
    kinds: Vec<(NodeKindId, String)>,
}

impl NodeSearch {
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a selectable node kind.
    pub fn kind(mut self, id: NodeKindId, label: impl Into<String>) -> Self {
        self.kinds.push((id, label.into()));
        self
    }

    /// Show the palette anchored to `trigger`. Returns the picked kind, if any.
    pub fn show(self, ui: &mut egui::Ui, trigger: &Response) -> Option<NodeKindId> {
        let query_id = ui.id().with("node_search_query");
        let mut chosen = None;

        Popover::new().show(trigger, |ui| {
            let mut query: String = ui.data_mut(|d| d.get_temp(query_id).unwrap_or_default());
            Input::new(&mut query).placeholder("Search nodes…").show(ui);

            let q = query.to_lowercase();
            for (id, label) in &self.kinds {
                let matches = q.is_empty() || label.to_lowercase().contains(&q);
                if matches && MenuItem::new(label.clone()).show(ui).clicked() {
                    chosen = Some(*id);
                }
            }
            ui.data_mut(|d| d.insert_temp(query_id, query));
        });

        chosen
    }
}
