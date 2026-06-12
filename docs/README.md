# ouroboros-ui — Documentation

Token-first design system for [egui](https://github.com/emilk/egui) — the
[shadcn/ui](https://ui.shadcn.com) design *language* reimplemented natively in Rust.
Not a web port: same vocabulary (semantic tokens, neutral zinc aesthetic, 4px scale),
egui-native immediate-mode rendering.

This is the full documentation set: how the system is built, how to use it, and a
reference page for every component.

## Start here

| Doc | What it covers |
|-----|----------------|
| [Architecture](./architecture.md) | The layered model (core → semantic → component → atoms → cells → molecules → organisms), dependency direction, the primordial atomic-design law. |
| [Governance](./governance.md) | **The law** — use first, extend second, create last: the decision ladder, what is forbidden in studio chrome, the escapes, the component contribution pipeline, enforcement. |
| [Usage](./usage.md) | Install, bootstrap the theme, the builder pattern, common recipes, how to consume the crate. |
| [Guards & conventions](./guards.md) | The two enforcement tests (`no_raw_values`, `no_painter_in_molecules`), what they forbid, how to add a component without tripping them. |

## Foundation reference

| Doc | What it covers |
|-----|----------------|
| [Tokens (core)](./tokens.md) | Raw primitives: color ramps, spacing, radius, shadows, sizing, motion, opacity. |
| [Theming (semantic)](./theming.md) | The `Theme` struct, the four palettes (dark / light / zinc-dark / zinc-light), `Mode`, `install`/`apply`/`get`. |
| [Typography](./typography.md) | Iosevka faces, weights, the named type styles (`display`…`kbd`), icon fonts. |
| [Layout & auto-layout](./layout.md) | Panel/grid/breakpoint tokens, the `Layer` z-order, and the Figma-style `AutoLayout` flow engine. |

## Component reference

Every component has its own page under [`components/`](./components/), grouped by layer:

- **[Atoms](./components/README.md#atoms)** — 23 leaf components that paint with tokens only.
- **[Cells](./components/README.md#cells)** — 7 row/item building blocks.
- **[Molecules](./components/README.md#molecules)** — 14 compositions of atoms.
- **[Organisms](./components/README.md#organisms)** — 14 full UI sections.
- **[Graph](./components/graph/README.md)** — the node-editor peer layer (reactflow-style on `egui::Scene`).

See the [component catalog](./components/README.md) for the complete index.

## Run the storybook

The storybook is the living visual reference — every token and component rendered, with
a Dark/Light mode toggle.

```bash
cd ouroboros-ui
cargo run --example storybook
```

## At a glance

- **egui / eframe** `0.34.1` · **egui_extras** `0.34` · **egui-phosphor** `0.12` (Light variant)
- **Rust** pinned to `1.92.0` (`rust-toolchain.toml`)
- Standalone crate — **zero coupling** to the Ouroboros monorepo workspace
- License: MIT © Type Zero Labs
</content>
</invoke>
