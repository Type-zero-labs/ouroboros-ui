# Graph identity vocabulary

> **Layer:** graph · **Path:** `src/graph/mod.rs` · **Exports:** `NodeId`, `PortId`, `NodeKindId`, `PortSide`, `Port`, `Connection`

The small set of identifier and address types the graph layer carries around. They are the
**only vocabulary shared between caller and library** — the [data-model-agnostic
contract](./README.md#data-model-agnostic-contract) means the library never sees the
caller's domain types, only these.

> **The caller assigns all ids.** The library hashes nothing on its own — it just carries
> these values and reports them back in [`GraphResponse`](./canvas.md). Ids must be **stable
> across frames** (e.g. a hash of the node's domain key), or selection/drag/connect state
> won't line up frame-to-frame.

---

## Identifiers

| Type | Definition | Meaning |
|------|------------|---------|
| `NodeId` | `pub struct NodeId(pub u64)` | Stable id of a node, assigned by the caller. |
| `PortId` | `pub struct PortId(pub u32)` | Id of a port within a node's port list, caller-assigned. |
| `NodeKindId` | `pub struct NodeKindId(pub u64)` | Id of a node *kind* offered by [search](./search.md) — caller-defined. |

All three are `Clone + Copy + PartialEq + Eq + Hash + Debug` newtypes — wrap your own
identity (an index, a slotmap key, a hash) and hand it in.

## Port addressing

```rust
pub enum PortSide { In, Out }

pub struct Port {
    pub node: NodeId,
    pub port: PortId,
    pub side: PortSide,
}
```

- **`PortSide`** — which side of a node a port lives on. Inputs anchor on the left/top,
  outputs on the right/bottom. A connection always runs **`Out → In`**.
- **`Port`** — a fully-qualified port: a node, a port within it, and the side it sits on.
  This is the address used everywhere edges and handles are referenced.

## Connection

```rust
pub struct Connection {
    pub from: Port,   // the Out side
    pub to: Port,     // the In side
}
```

A requested edge between two ports, emitted by the library on a successful connect-drag
(in [`GraphResponse.connection`](./canvas.md)). The library **orients it `Out → In`**
regardless of which end the user dragged from, so `from` is always the output port and `to`
the input. The caller commits it to its own edge list.

---

## How they flow

```
caller defines ids ──▶ ctx.node(NodeId, …) / NodeFrame.input(PortId) / .output(PortId)
                                     │
                          user drags Out handle → In handle
                                     │
                       GraphResponse.connection: Option<Connection {from: Out, to: In}>
                                     │
                          caller: model.add_edge(c.from, c.to)
```

Edges the caller passes back in are addressed the same way: `ctx.edge(from: Port, to: Port,
…)`. Deletion intents (`delete_edge: Option<(Port, Port)>`, `edge_clicked`,
`delete_nodes: Vec<NodeId>`) all speak this same vocabulary.

See [node](./node.md) for declaring ports on a `NodeFrame`, [handle](./handle.md) for how a
port renders, and [canvas](./canvas.md) for the full intent set.
</content>
