# Graph — the node-editor layer

A peer layer beside [atoms/cells/molecules/organisms](../README.md), blueprinted on
[reactflow.dev/ui](https://reactflow.dev/ui) but built entirely from the design system's
tokens and atoms. It renders a pannable/zoomable node canvas — nodes, ports, bezier wires,
a dot grid, selection, drag, connect — on top of [`egui::Scene`](https://docs.rs/egui).

Source: `src/graph/` (14 files, ~1.8k lines). Single public entry point:
[`GraphView`](./canvas.md).

---

## The paint-but-token invariant

`graph` is the **one place outside `atoms` that paints**. A node graph genuinely needs grid
dots, bezier wires, handle circles and a marquee — none of which the atom vocabulary covers
— so the `no_painter_in_molecules` guard deliberately **does not** scan it.

But the purity contract still holds: **every value flows through a token.** Colors come from
[`Theme`](../../theming.md) (resolved once per frame into [`GraphTokens`](./tokens.md)),
geometry from [`core`](../../tokens.md). No raw `Color32::from_*`, no bare stroke/radius
literals — the `no_raw_values` guard is **extended to scan `src/graph`**, giving this layer
the same purity contract the atoms have. See [guards](../../guards.md).

## Two internal tiers

| Tier | Modules | Rule |
|------|---------|------|
| **paint** | `grid`, `edge`, `handle`, `resizer` | touch the painter, but only via tokens |
| **compose** | `node`, `controls`, `minimap`, `toolbar`, `search` | reuse `Surface` + atoms; never paint inline |

## Data-model-agnostic contract

The **caller owns the data** — node/edge identity, world positions, the edge list. The
**library owns only view state** — pan/zoom (`scene_rect`), selection, what's mid-drag —
held in [`GraphViewState`](./state.md) inside egui memory. Each frame the library reports
**intents** in [`GraphResponse`](./canvas.md) (`node_moved`, `connection`, `delete_edge`,
`delete_nodes`, `edge_clicked`, `create_request`, …) for the caller to commit. The library
never sees the caller's domain types — only the [identity vocabulary](./identity.md)
(`NodeId`/`PortId`/`Port`/`Connection`), which the caller defines.

```
caller model ──describe──▶ GraphView::show(|ctx| { ctx.node(..); ctx.edge(..); })
     ▲                                              │
     └──────────── commit intents ◀── GraphResponse ┘
```

---

## Frame lifecycle

1. `GraphView::new(id).grid(true).controls(true).minimap(true)` — configure.
2. `.show(ui, |ctx| { … })` allocates the canvas, runs `egui::Scene` (pan/zoom), paints the
   grid, reserves an under-node edge layer, and runs your closure in **scene (world)
   coordinates**.
3. Inside the closure you emit nodes ([`ctx.node`](./node.md)) — each declaring its ports
   ([handles](./handle.md)) and body — and wires ([`ctx.edge`](./edge.md)). Edges are
   accumulated and flushed *under* the nodes at scope end, independent of call order.
4. `show` returns [`GraphResponse`](./canvas.md) with the frame's intents.

> **Emit nodes before edges.** Edge routing reads handle positions recorded as nodes are
> emitted; an edge whose endpoint handle wasn't recorded yet resolves to nothing.

---

## Pages

| Page | Covers |
|------|--------|
| [identity](./identity.md) | `NodeId`, `PortId`, `NodeKindId`, `PortSide`, `Port`, `Connection` — the caller-defined vocabulary. |
| [canvas](./canvas.md) | `GraphView` (entry point), `GraphCtx` (per-frame emit surface), `GraphResponse` (intents). |
| [state](./state.md) | `GraphViewState` + the in-flight drag structs. |
| [tokens](./tokens.md) | `GraphTokens` — the single resolve point for everything the layer paints. |
| [node](./node.md) | `NodeFrame`/`NodeResult`/`NodeStatus` + `ctx.node(...)`. Compose-tier. |
| [edge](./edge.md) | `EdgeStyle`/`EdgeResult` + `ctx.edge(...)`. Paint-tier bezier wires. |
| [handle](./handle.md) | `HandleSpec`/`HandleVariant` — ports, declared on `NodeFrame`. Paint-tier. |
| [search](./search.md) | `NodeSearch` — the node-creation palette. Compose-tier. |
| [extras](./extras.md) | Internal support pieces: `grid`, `resizer`, `minimap`, `toolbar`, `controls`. |

## Minimal usage

```rust
use ouroboros_ui::graph::{GraphView, EdgeStyle};

let resp = GraphView::new("my_graph")
    .grid(true).controls(true).minimap(true)
    .show(ui, |ctx| {
        for n in &model.nodes {
            ctx.node(n.id, n.world_pos, n.frame(), |ui| { /* node body */ });
        }
        for e in &model.edges {
            ctx.edge(e.from, e.to, EdgeStyle::Default);
        }
    });

// Commit the intents to your own model:
for (id, delta) in resp.node_moved { model.move_node(id, delta); }
if let Some(c) = resp.connection { model.add_edge(c.from, c.to); }
for id in resp.delete_nodes { model.remove_node(id); }
```

See the storybook (`page_graph_live`, `page_graph_node`, `page_graph_edge`,
`page_graph_search`) for the full intent-commit loop: `cargo run --example storybook`.
</content>
