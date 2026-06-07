# TreeView

> **Layer:** organism · **Path:** `src/organisms/tree_view.rs` · **Exports:** `tree_view::{TreeItem, TreeView}`

A hierarchy of [`TreeNode`](../cells/tree_node.md) cells (Unity / O3DE Tree View), bound to a `&mut usize` selection. The tree is a **flat list of [`TreeItem`](#treeitem)s** carrying their own `depth`; the view renders them as nested rows, hiding descendants of collapsed nodes. Expand/collapse state lives in egui memory; clicking an expandable node toggles it. `show` returns the index clicked, if any.

## Design

- **Purpose / when to use** — scene hierarchies, file trees, any depth-indented selectable list. (Wrap in a `ScrollArea` for scrolling — the view itself does not scroll.)
- **Anatomy** — a loop over the flat `items`, each rendered as a `TreeNode` (`.depth(item.depth)`, `.selected(*selected == i)`, keyed `(id, "node", i)`, optional glyph, `.expandable(is_open)` when the item is expandable). Descendants of a collapsed node (deeper `depth`) are skipped via a `collapse_until` threshold.
- **Variants / states**

  | State | How |
  |---|---|
  | leaf | `TreeItem::new(label)` with no `.expanded(...)` |
  | expandable | `.expanded(open)` — marks it toggleable, seeds default open state |
  | open / collapsed | tracked in a `HashSet<usize>` in egui memory |
  | selected | `*selected == i` |
  | depth | `.depth(n)` — indentation level |

- **Tokens / layout consumed** — themed indentation/visuals via the `TreeNode` cell (no direct token use here).
- **Accessibility** — click a row to select; clicking an expandable node both selects and toggles it.

## API

### `TreeView<'a>`

| Method | Effect |
|---|---|
| `TreeView::new(selected: &'a mut usize) -> Self` | Bind to a selection index; default id `Id::new("tree_view")`. |
| `.items(impl IntoIterator<Item = TreeItem>) -> Self` | Set the flat item list (order + `depth` define the tree). |
| `.id_source(id: impl Hash) -> Self` | Key for the persisted expand/collapse set. |
| `.show(ui) -> Option<usize>` | Render; returns the clicked index, writes `*selected`, persists expansion. |

### TreeItem

One row of the tree.

| Method | Effect |
|---|---|
| `TreeItem::new(label: impl Into<String>) -> Self` | New leaf at `depth = 0`, no icon, not expandable. |
| `.depth(depth: usize) -> Self` | Indentation level (defines nesting). |
| `.icon(glyph: &'static str) -> Self` | Leading glyph. |
| `.expanded(open: bool) -> Self` | Mark expandable; `open` seeds the first-frame open state. |

## Usage

```rust
use ouroboros_ui::organisms::{TreeView, TreeItem};
use ouroboros_ui::egui_phosphor::light;

let mut sel = 0usize;
TreeView::new(&mut sel)
    .items([
        TreeItem::new("Scene").icon(light::FOLDER).expanded(true),
        TreeItem::new("Player").depth(1).icon(light::CUBE),
    ])
    .show(ui);
```

```rust
// realistic — scene hierarchy, persisted selection (from storybook)
use ouroboros_ui::organisms::{TreeView, TreeItem};
use ouroboros_ui::egui_phosphor::light;

let id = egui::Id::new("treeview_demo");
let mut sel = ui.data(|d| d.get_temp::<usize>(id).unwrap_or(1));
TreeView::new(&mut sel)
    .items([
        TreeItem::new("Scene").icon(light::FOLDER).expanded(true),
        TreeItem::new("Player").depth(1).icon(light::CUBE),
        TreeItem::new("Camera").depth(1).icon(light::CUBE),
        TreeItem::new("Environment").depth(1).icon(light::FOLDER).expanded(false),
    ])
    .show(ui);
ui.data_mut(|d| d.insert_temp(id, sel));
```

## Composition

Composes [`TreeNode`](../cells/tree_node.md) cells (one per visible row). It never paints — see [guards](../../guards.md).

## Notes

- **Flat model** — the tree is a flat `Vec<TreeItem>` where `depth` encodes nesting; a collapsed expandable node hides every following item with greater `depth` (the `collapse_until` threshold) until depth returns to its level.
- **State ownership** — selection is the consumer's `&mut usize` (persist it yourself). Expand/collapse is a `HashSet<usize>` in egui memory keyed by `id_source`; on the first frame it seeds from each item's `.expanded(true)` default. Indices in the set are positional — reordering items shifts which nodes are "open".
- Distinct trees need distinct `id_source` (default is the literal `"tree_view"`).
- No internal scrolling — wrap in a `ScrollArea` if the tree can overflow.
