# NodeSearch

> **Layer:** graph (compose-tier) · **Path:** `src/graph/search.rs` · **Exports:** `NodeSearch`

A command-palette popover for picking a node *kind* to create. It is a [`Popover`](../organisms/popover.md) holding a text [`Input`](../atoms/input.md) filter over a caller-supplied list of node kinds, each rendered as a [`MenuItem`](../cells/menu_item.md). It is **data-model-agnostic**: the caller owns the kind list and decides where the chosen kind is placed. `show` returns the picked [`NodeKindId`](./identity.md); the caller then emits a `create_request` into [`GraphResponse`](./canvas.md) (or just spawns the node directly).

It paints nothing of its own — it composes existing atoms/cells/organisms, honouring the compose-tier contract (see [layer README](./README.md) and [guards](../../guards.md)).

## Design

**Purpose.** Turn a click on an "Add node" trigger into a filtered menu of node kinds, returning the kind the user picked. It does not place the node — placement (and the `(NodeKindId, Pos2)` pairing) is the caller's job.

**Anatomy.**

- A [`Popover`] anchored to a `trigger: &Response` (typically a [`Button`](../atoms/button.md)).
- Inside: an [`Input`] with placeholder `"Search nodes…"`, whose query is persisted in `ui.data` under `ui.id().with("node_search_query")` (survives across frames while the popover is open).
- Below: one [`MenuItem`] per kind whose label (case-insensitively) contains the query. Empty query shows all kinds. Clicking a [`MenuItem`] records that kind as the chosen result.

**API surface.** A small consuming builder: `new()` → chain `.kind(id, label)` once per kind → terminal `.show(ui, trigger)`. `show` takes `self` by value (consumes the builder) and returns `Option<NodeKindId>`.

**Tokens.** None applied directly — all colour/spacing flows through the composed atoms ([`Input`], [`MenuItem`], [`Popover`]), which resolve from [`Theme`](../../tokens.md). `NodeSearch` does not touch [`GraphTokens`](./tokens.md).

## API

`use ouroboros_ui::graph::NodeSearch;`

| Item | Signature | Notes |
|------|-----------|-------|
| field | `kinds: Vec<(NodeKindId, String)>` | private; populated via `.kind()` |
| `NodeSearch::new` | `fn new() -> Self` | empty palette (also `#[derive(Default)]`) |
| `.kind` | `fn kind(self, id: NodeKindId, label: impl Into<String>) -> Self` | append one selectable kind; chainable |
| `.show` | `fn show(self, ui: &mut egui::Ui, trigger: &egui::Response) -> Option<NodeKindId>` | consumes `self`; returns the kind picked this frame, else `None` |

`NodeKindId(pub u64)` is a caller-defined identifier for a node *kind* (not a node instance). See [identity](./identity.md). The picked id is `Copy`; read `.0` for the raw `u64`.

## Usage

Realistic flow: a trigger button feeds `NodeSearch`, and the picked kind is paired with a world position to fill `GraphResponse.create_request`. Because `NodeSearch::show` returns only the kind, the caller supplies the `Pos2` (e.g. the canvas centre, or the last right-click point).

```rust
use egui::{pos2, Pos2};
use ouroboros_ui::atoms::Button;
use ouroboros_ui::graph::{NodeKindId, NodeSearch};

// 1. Trigger.
let trigger = Button::new("Add node")
    .icon_left(egui_phosphor::light::PLUS)
    .id_source("add_node")
    .show(ui);

// 2. Palette anchored to the trigger.
let chosen: Option<NodeKindId> = NodeSearch::new()
    .kind(NodeKindId(1), "Trigger")
    .kind(NodeKindId(2), "Condition")
    .kind(NodeKindId(3), "Action")
    .kind(NodeKindId(4), "Delay")
    .show(ui, &trigger);

// 3. Pair the kind with a drop point and hand it to the caller's create handler.
//    (Mirrors GraphResponse.create_request: Option<(NodeKindId, Pos2)>.)
if let Some(kind) = chosen {
    let drop_at: Pos2 = pos2(120.0, 80.0); // caller-chosen world position
    spawn_node(kind, drop_at);             // caller commits to its own model
}
```

The graph canvas exposes the *committed* form of this on its response:

```rust
let resp = GraphView::new("graph").show(ui, |g| { /* … */ });
if let Some((kind, world_pos)) = resp.create_request {
    // caller adds a node of `kind` at `world_pos` to its data model
}
```

> `NodeSearch` itself does **not** populate `GraphResponse.create_request` — it is a standalone palette. The canvas field is filled by the canvas's own internal create path; `NodeSearch` is the recommended UI for producing the `NodeKindId` half of that pair. See [canvas](./canvas.md).

## Composition / Notes

- **Tier:** compose. Reuses [`Popover`], [`Input`], [`MenuItem`] — no inline painting, satisfying the graph layer's compose-tier rule.
- **Stateless across instances:** the only persisted state is the filter query, keyed off `ui.id()`. Use distinct parent ids if you host two palettes in one `Ui` to avoid query bleed.
- **Caller owns identity & placement:** `NodeKindId`s are caller-defined and the drop position is caller-chosen; the library never sees domain types. This is the same data-agnostic contract the rest of the [graph layer](./README.md) follows.
- **Filtering:** substring, case-insensitive, on the label only. No fuzzy match, no keyboard navigation, no scroll virtualization — fine for the tens-of-kinds palettes this targets.
