# TreeNode

> **Layer:** cell · **Path:** `src/cells/tree_node.rs` · **Exports:** `tree_node::TreeNode`

An indented hierarchy row: a depth-based indent, an optional expand/collapse caret, an optional icon, and a label. Modelled on Unity/O3DE tree views. Expansion and selection are **inputs**, not state — `show` returns the [`Response`] and the consumer toggles expand/selection itself.

## Design

- **Purpose / when to use** — Scene graphs, file trees, outliners — any flattened hierarchy rendered row-by-row with per-row `depth`.
- **Anatomy** — A [`Surface`](../atoms/surface.md) (interactive, padded) wrapping a horizontal layout: indent space (`depth × core::SPACE_4`), a caret [`Icon`](../atoms/icon.md) (`CARET_DOWN`/`CARET_RIGHT`, small, muted) **or** an `core::ICON_SM` spacer when not expandable, optional muted [`Icon`](../atoms/icon.md), then the [`Text`](../atoms/text.md) label.
- **Variants / states**

  | State | Effect |
  |-------|--------|
  | default | `Surface::fill_none().border_none()` |
  | selected (`selected(true)`) | `Surface::muted()` fill |
  | expandable + collapsed | `light::CARET_RIGHT` caret |
  | expandable + expanded | `light::CARET_DOWN` caret |
  | not expandable | `core::ICON_SM` spacer (keeps labels aligned with carets) |
  | indent | `depth as f32 * core::SPACE_4` leading space |

- **Tokens / layout consumed** — `core::SPACE_4` (16px per depth level), `core::SPACE_1` (4px outer pad + gaps), `core::ICON_SM` (14px caret slot), `core::RADIUS_SM` (4px). See [tokens](../../tokens.md).
- **Accessibility** — Selection/expansion are visual inputs; pair with a real tree model.

## API

| Method | Signature | Effect |
|--------|-----------|--------|
| `new` | `new(label: impl Into<String>) -> Self` | Construct at `depth=0`, not expandable, not selected. |
| `depth` | `depth(self, depth: usize) -> Self` | Indent level (× `core::SPACE_4`). |
| `icon` | `icon(self, glyph: &'static str) -> Self` | Leading muted icon (after the caret slot). |
| `expandable` | `expandable(self, expanded: bool) -> Self` | Mark expandable **and** set the expanded flag (caret direction) in one call. |
| `selected` | `selected(self, selected: bool) -> Self` | Toggle the `muted` selected fill. |
| `id_source` | `id_source(self, id: impl std::hash::Hash) -> Self` | Stable id for the underlying `Surface`. |
| `show` | `show(self, ui: &mut Ui) -> Response` | Render; returns the `Surface` response (`.clicked()`). |

## Usage

```rust
use ouroboros_ui::cells::TreeNode;
use ouroboros_ui::egui_phosphor::light;

TreeNode::new("Scene").icon(light::FOLDER).expandable(true).id_source("tn0").show(ui);
```

```rust
// realistic — a flattened tree with depths and selection
TreeNode::new("Scene").icon(light::FOLDER).expandable(true).id_source("tn0").show(ui);
TreeNode::new("Player").depth(1).icon(light::CUBE).expandable(false).selected(true).id_source("tn1").show(ui);
TreeNode::new("Camera").depth(1).icon(light::CUBE).id_source("tn2").show(ui);
TreeNode::new("Mesh").depth(2).icon(light::CUBE).id_source("tn3").show(ui);
```

## Composition

Composes the [`Surface`](../atoms/surface.md), [`Icon`](../atoms/icon.md) (caret + leading icon), and [`Text`](../atoms/text.md) atoms only. It paints nothing — visuals come from `Surface` + atoms. Enforced by [`tests/no_painter_in_molecules.rs`](../../guards.md).

## Notes

- `expandable(expanded)` does double duty: it both **marks** the node expandable and **sets** the caret direction. There is no separate "is expandable" flag — calling it at all makes the node expandable.
- Expansion/selection are not remembered by the cell. Drive them from a tree model and toggle on `.clicked()` (and detect a caret-vs-row click yourself if needed).
- Non-expandable rows reserve an `core::ICON_SM` spacer so their labels align with sibling carets.
- Caret glyphs come straight from `egui_phosphor::light` (`CARET_DOWN` / `CARET_RIGHT`).
