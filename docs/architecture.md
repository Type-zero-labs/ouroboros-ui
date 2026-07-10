# Architecture

ouroboros-ui is built in **seven layers**. The lower three are *tokens* (data); the
upper four are *components* (atomic design). The single rule across the whole stack: a
layer may reference the layer below it, and **nothing below knows the layer above**.

```
┌─────────────────────────────────────────────────────────────┐
│  organisms   splitter, dialog, table, tree_view, toast…      │  full UI sections
│      ↓ compose                                                │
│  molecules   field, card, alert, tabs, radio_group…          │  compositions of atoms
│      ↓ compose                                                │
│  cells       list_item, menu_item, property_row, table_row…  │  row/item building blocks
│      ↓ compose                                                │
│  atoms       button, input, text, icon, surface, badge…      │  leaf widgets (paint here)
│      ↓ read                                                   │
│  component   ButtonTokens, BadgeTokens…  (tokens::component)  │  per-component overrides
│      ↓ read                                                   │
│  semantic    Theme { background, primary, border, ring… }    │  shadcn vocabulary
│      ↓ read                                                   │
│  core        ZINC_950, SPACE_4, RADIUS_MD, TEXT_BASE…        │  raw primitives (const)
└─────────────────────────────────────────────────────────────┘

  ┌── graph ──┐  peer layer (node editor on egui::Scene) — reads the same tokens,
  └───────────┘  but is the one place outside atoms allowed to paint. See below.
```

The crate root re-exports the four most-used names so consumers don't reach deep:

```rust
pub use theme::typography::{TypeStyle, Weight};
pub use theme::Mode;
pub use tokens::core::Size;
pub use tokens::semantic::Theme;
pub use egui_phosphor; // icon glyphs, no separate dependency
```

---

## The token layers (data)

### 1. core — raw primitives

`src/tokens/core.rs`. Pure `const`s with **no meaning**: the Zinc neutral ramp
(50→950), the Teal brand ramp (200→600), status hues, the 4px spacing scale, radius
scale, shadows, type sizes, control/icon sizing, motion durations + easing, opacity.

Nothing here references anything. It is a leaf. The only non-trivial logic is the
`Easing` curve math and two helpers (`disabled_color`, `hover_t`) that every atom shares
so state transitions are identical. See [tokens.md](./tokens.md).

### 2. semantic — the shadcn vocabulary

`src/tokens/semantic.rs`. The `Theme` struct maps shadcn's semantic names
(`background`/`foreground`, `primary`, `muted`, `accent`, `destructive`, `border`,
`ring`, plus domain status pairs) onto `core` primitives. **No raw colors live here** —
every field is a `core::*` reference. Four palettes: `dark()`, `light()`, `zinc_dark()`,
`zinc_light()`. See [theming.md](./theming.md).

### 3. component — per-component overrides

`src/tokens/component.rs`. A thin struct per component holding the exact values it
paints with, **derived from `semantic` (never from `core`)**. This lets one component be
retuned — a denser button, a louder input focus — without touching global tokens.
`ButtonTokens` and `BadgeTokens` are the worked examples; most components default
straight to semantic tokens and never need a struct here.

---

## The component layers (atomic design)

### 4. atoms

`src/atoms/`. The smallest components — the **only** layer allowed to paint
primitives (`painter.rect_filled`, `galley`, etc.). Each atom is a builder that paints
exclusively with foundation tokens; no hardcoded colors, sizes, radii, fonts, or motion.
An atom may compose smaller atoms (e.g. `Button` composes `Icon` + `Text`).

### 5. cells

`src/cells/`. Compound *row/item* building blocks that sit between molecules and
organisms — a property row, a list/menu/tree item, a table row, a toolbar button. Cells
**compose, never paint**.

### 6. molecules

`src/molecules/`. Components composed **only** from atoms (and smaller molecules) plus
`auto_layout`. Molecules **compose, never paint**.

### 7. organisms

`src/organisms/`. Full UI sections composed from cells, molecules, and atoms. Overlay
organisms use egui containers for *placement*: Dialog → `Modal`, Toast → `Area`,
Popover/DropdownMenu/Select/Menubar → `Popup::menu`. The *casing* is either a token
`Surface` atom (Toast, Toolbar, bordered Table) or themed egui visuals driven by `Theme`
tokens (the Modal/Popup-based ones) — placement is egui's job, the look is always tokens.

### graph — the node-editor peer layer

`src/graph/`. A peer layer beside the four above — a reactflow-style node editor
(`GraphView`) built on `egui::Scene`. It is the **one place outside `atoms` that paints**: a
node graph needs grid dots, bezier wires, handle circles and a marquee, none of which the
atom vocabulary covers. The atomic-design rules don't fit it, so it has its **own invariant**
instead — *paint, but only through tokens*:

- It **may call the painter**, but every value still flows through a token (colors from
  `Theme` resolved into `GraphTokens`, geometry from `core`). The `no_raw_values` guard is
  **extended to scan `src/graph`**, so it has the same purity contract as atoms.
- The `no_painter_in_molecules` guard **deliberately skips it** — painting here is allowed.

Internally it splits into a **paint tier** (`grid`/`edge`/`handle`/`resizer`) and
a **compose tier** (`node`/`controls`/`minimap`/`toolbar`/`search`, which reuse `Surface` +
atoms). And it follows a **data-model-agnostic contract**: the caller owns the node/edge
data, the library owns only view-state and reports back *intents*. Full docs:
[components/graph](./components/graph/README.md).

---

## The primordial law

> **An organism is built only from molecules and atoms. A molecule is built only from
> atoms and smaller molecules. Nothing above the atom layer hand-rolls a primitive — if a
> piece needs painting, it becomes an atom first.**

This is not a style preference; it is **mechanically enforced** by two test guards that
run with `cargo test`:

- `tests/no_raw_values.rs` — scans `src/atoms/**` **and `src/graph/**`**, fails on hardcoded
  `Color32::from_rgb`, named `Color32` constants, or raw `FontId::new`. Atoms (and the graph
  layer) must source colors from `Theme`/`core` and fonts from `theme::typography`.
- `tests/no_painter_in_molecules.rs` — scans `src/cells/**`, `src/molecules/**`,
  `src/organisms/**`, fails on any painting call (`ui.painter()`, `.rect_filled()`,
  `.circle_stroke()`, `Shape::*`, …). Above atoms you compose; you never paint. **`src/graph`
  is deliberately not scanned** — it is the sanctioned exception that paints (still via
  tokens, enforced by `no_raw_values`).

The consequence: the missing-piece-becomes-an-atom discipline keeps the atom set complete
and every higher layer pure composition. See [guards.md](./guards.md).

---

## Why a Rust/egui design system at all

The Ouroboros Studio (authoring IDE) and the engine HUD are both egui apps. A shared,
token-driven component library means:

- **One visual language** across engine HUD and studio chrome (the `auto_layout` module
  even mirrors the engine HUD's `LayoutDirection`/`MainAlign`/`SizeMode` vocabulary).
- **Theme-able** — swap `Mode::Dark`/`Light` (or the zinc-neutral variants) at runtime
  with no consumer changes, because everything resolves through `Theme::resolve`.
- **Auditable** — the guards make "did someone hardcode a color" a CI failure, not a
  code-review judgment call.

## Builder pattern everywhere

Every component, at every layer, follows the same shape:

```rust
Component::new(required_args)
    .setter(value)   // chainable, returns Self
    .setter(value)
    .show(ui)        // consumes self, paints, returns egui::Response
```

This gives fluent call sites, makes optional props obvious, and keeps the return value a
plain `egui::Response` so components drop into any egui layout. See [usage.md](./usage.md).

## Standalone by design

`ouroboros-ui` is its own cargo workspace with **zero** dependency on the parent monorepo.
It can be cloned, built, and storybooked in isolation. The trade-off: shared vocabulary
with the engine HUD (`auto_layout`) is *re-declared*, not imported, to avoid the coupling.
</content>
