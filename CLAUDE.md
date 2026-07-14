# CLAUDE.md — ouroboros-ui

Token-first design system for egui — the shadcn/ui design *language* reimplemented
natively in Rust (egui/eframe `0.34.1`, Rust `1.92.0` pinned). Standalone workspace:
zero coupling to the Ouroboros monorepo. The studio consumes it as `ouro_ds` via the
`ui/` submodule.

## Layers

```
tokens (core → semantic/Theme → component) → atoms → cells → molecules → organisms
graph = peer layer (node editor; paints, but only through tokens)
```

**Nothing above atoms paints.** Atoms are the only layer that touches the painter; cells,
molecules, and organisms compose. Every component is a builder:
`Component::new(..).setter(..).show(ui) -> Response`.

## Commands

```bash
cargo test                                  # unit + kittest + the governance guards
cargo run --example storybook               # living visual reference (all components/tokens)
cargo clippy --all-targets -- -D warnings
cargo fmt
```

## The law → [docs/governance.md](./docs/governance.md)

**Use first, extend second, create last.** No raw values: colors/fonts/spacing come from
tokens, never literals. The guards (`tests/no_raw_values.rs`,
`tests/no_painter_in_molecules.rs`) hard-fail the build on violations — they run with
`cargo test` and in CI. New components walk the contribution pipeline in governance.md
(layer → tokens → builder → storybook → test → doc).

## Docs map

[docs/architecture.md](./docs/architecture.md) (layer model) ·
[docs/governance.md](./docs/governance.md) (the law) ·
[docs/layout.md](./docs/layout.md) (AutoLayout/panels) ·
[docs/components/](./docs/components/README.md) (catalog, 60 components)
