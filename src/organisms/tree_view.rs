//! TreeView organism — a hierarchy of [`TreeNode`] cells. [Unity/O3DE Tree View]

use crate::cells::TreeNode;
use egui::{Id, Ui};
use std::collections::HashSet;

/// One row of a [`TreeView`].
pub struct TreeItem {
    label: String,
    depth: usize,
    icon: Option<&'static str>,
    expanded: Option<bool>,
}

impl TreeItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            depth: 0,
            icon: None,
            expanded: None,
        }
    }
    pub fn depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }
    pub fn icon(mut self, glyph: &'static str) -> Self {
        self.icon = Some(glyph);
        self
    }
    /// Mark the node expandable with the given open state.
    pub fn expanded(mut self, open: bool) -> Self {
        self.expanded = Some(open);
        self
    }
}

/// A tree of items bound to a `&mut usize` selection. Expand/collapse state is kept in egui
/// memory; clicking an expandable node toggles it (and hides its descendants). `show` returns
/// the index clicked, if any. (Wrap in a `ScrollArea` for scrolling.)
pub struct TreeView<'a> {
    selected: &'a mut usize,
    items: Vec<TreeItem>,
    id: Id,
}

impl<'a> TreeView<'a> {
    pub fn new(selected: &'a mut usize) -> Self {
        Self {
            selected,
            items: Vec::new(),
            id: Id::new("tree_view"),
        }
    }
    pub fn items(mut self, items: impl IntoIterator<Item = TreeItem>) -> Self {
        self.items = items.into_iter().collect();
        self
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id = Id::new(id);
        self
    }

    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let selected = self.selected;
        let items = self.items;
        let id = self.id;
        // Expanded-node indices, persisted; first frame seeds from each item's default.
        let mut expanded: HashSet<usize> = ui.data(|d| d.get_temp(id)).unwrap_or_else(|| {
            items
                .iter()
                .enumerate()
                .filter(|(_, it)| it.expanded == Some(true))
                .map(|(i, _)| i)
                .collect()
        });

        let mut clicked = None;
        // While inside a collapsed subtree, skip nodes deeper than the collapse threshold.
        let mut collapse_until: Option<usize> = None;
        for (i, item) in items.iter().enumerate() {
            if let Some(thresh) = collapse_until {
                if item.depth > thresh {
                    continue;
                }
                collapse_until = None;
            }
            let expandable = item.expanded.is_some();
            let is_open = expanded.contains(&i);
            let mut node = TreeNode::new(item.label.clone())
                .depth(item.depth)
                .selected(*selected == i)
                .id_source((id, "node", i));
            if let Some(glyph) = item.icon {
                node = node.icon(glyph);
            }
            if expandable {
                node = node.expandable(is_open);
            }
            if node.show(ui).clicked() {
                *selected = i;
                clicked = Some(i);
                if expandable {
                    if is_open {
                        expanded.remove(&i);
                    } else {
                        expanded.insert(i);
                    }
                }
            }
            if expandable && !expanded.contains(&i) {
                collapse_until = Some(item.depth);
            }
        }

        ui.data_mut(|d| d.insert_temp(id, expanded));
        clicked
    }
}
