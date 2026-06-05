# ouroboros-ui

Token-first design system for [egui](https://github.com/emilk/egui) — the
[shadcn/ui](https://ui.shadcn.com) design *language* reimplemented natively in Rust.
Not a web port: same vocabulary (semantic tokens, neutral zinc aesthetic, 4px scale),
egui-native rendering. See [`CREDITS.md`](./CREDITS.md).

> **Status:** foundation in progress — tokens + theme/modes + storybook. Atoms,
> molecules and organisms come in later milestones.

## Layered architecture

Each layer references the one below it; nothing below knows the layer above.

```
component  (per-component overrides — ButtonTokens, InputTokens…)
    ↓ references
semantic   (shadcn tokens — background/foreground, primary, muted, border, ring…)
    ↓ references
core       (raw primitives — color ramps, spacing, radius, shadow, type sizes)
```

- `src/tokens/core.rs` — primitives (`const`s, no meaning).
- `src/tokens/semantic.rs` — the `Theme` struct; semantic tokens mapped onto core.
- `src/tokens/component.rs` — thin per-component override structs (default to semantic).
- `src/theme/` — `Mode` enum, `Theme::resolve(Mode)`, `install`/`get`, typography.

## Modes

`Mode { Dark, Light }` is first-class. Dark (zinc) is populated; Light is currently a
stub that resolves to Dark. Theming is always resolved through `Theme::resolve` so the
light palette can be filled in without touching consumers.

## Run the storybook

```bash
cd ouroboros-ui
cargo run --example storybook
```

Renders the token gallery: color swatches, spacing/radius/shadow scales, and the type
scale. The visual surface for validating token decisions.

## Toolchain

Pinned to Rust `1.92.0` (egui 0.34.1 requires it). Run `cargo` from inside this
directory — the repo is its own workspace with zero monorepo coupling.

## License

MIT © Type Zero Labs. See [`LICENSE`](./LICENSE).
