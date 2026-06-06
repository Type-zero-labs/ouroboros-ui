//! TreeView organism — a hierarchy of [`TreeNode`] cells. [Unity/O3DE Tree View]

use crate::cells::TreeNode;
use egui::{ScrollArea, Ui};

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

/// A tree of items bound to a `&mut usize` selection. `show` returns the index clicked, if any.
pub struct TreeView<'a> {
    selected: &'a mut usize,
    items: Vec<TreeItem>,
}

impl<'a> TreeView<'a> {
    pub fn new(selected: &'a mut usize) -> Self {
        Self {
            selected,
            items: Vec::new(),
        }
    }
    pub fn items(mut self, items: impl IntoIterator<Item = TreeItem>) -> Self {
        self.items = items.into_iter().collect();
        self
    }

    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let selected = self.selected;
        let items = self.items;
        ScrollArea::vertical()
            .show(ui, |ui| {
                let mut clicked = None;
                for (i, item) in items.into_iter().enumerate() {
                    let mut node = TreeNode::new(item.label)
                        .depth(item.depth)
                        .selected(*selected == i)
                        .id_source(("tree", i));
                    if let Some(glyph) = item.icon {
                        node = node.icon(glyph);
                    }
                    if let Some(open) = item.expanded {
                        node = node.expandable(open);
                    }
                    if node.show(ui).clicked() {
                        *selected = i;
                        clicked = Some(i);
                    }
                }
                clicked
            })
            .inner
    }
}
