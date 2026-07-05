# ouroboros-ui

**The [shadcn/ui](https://ui.shadcn.com) design language, reimplemented natively for [egui](https://github.com/emilk/egui) — token-first, with governance tests that fail the build when a raw value sneaks past the tokens.**

Not a web port. Same vocabulary — semantic tokens, neutral zinc aesthetic, 4px scale — rendered egui-native in Rust. 60+ components across the atomic-design layers, plus a node-editor graph layer. Built for (and extracted from) the Ouroboros authoring IDE, where it powers the whole studio chrome.

> **Why this exists:** UI code is increasingly machine-generated, and it drifts — every session invents its own paddings and grays. A design system only holds if the *build* enforces it. Here, tokens aren't convention; they're law.

## The law, enforced

Two governance guards run with `cargo test` and in CI:

- [`tests/no_raw_values.rs`](./tests/no_raw_values.rs) — no literal colors, font sizes or spacing above the token layer. A hex color in a molecule **fails the build**.
- [`tests/no_painter_in_molecules.rs`](./tests/no_painter_in_molecules.rs) — atoms are the only layer allowed to touch the painter. Everything above composes.

Backed by clippy `disallowed_methods`. One-off escapes are explicit and greppable: `// ds-allow: <reason>`. The contribution pipeline (layer → tokens → builder → storybook → test → doc) lives in [docs/governance.md](./docs/governance.md).

## Layered architecture

Each layer references the one below it; nothing below knows the layer above.

```
organisms   (autocomplete, command palette, …)
molecules   (alert, dialog, dropdown, tabs, …)
cells       (labeled controls, composed groups)
atoms       (button, input, text, icon — the only layer that paints)
    ↓ references
component tokens   (ButtonTokens, InputTokens… default to semantic)
semantic tokens    (background/foreground, primary, muted, border, ring…)
core tokens        (raw primitives — color ramps, spacing, radius, type scale)

graph = peer layer (node editor; paints, but only through tokens)
```

## Quick start

```toml
[dependencies]
ouroboros-ui = { git = "https://github.com/Type-zero-labs/ouroboros-ui" }
```

```rust
use ouroboros_ui::atoms::Button;

if Button::new("Save").show(ui).clicked() {
    // …
}
```

Every component is a builder: `Component::new(..).setter(..).show(ui) -> Response`.

## Wiring the guards into your agent loop

The guards matter most when the code is machine-generated. If you develop with a coding
agent (Claude Code, Cursor, …), run them after every edit so the agent gets the failure
as feedback and self-corrects — the reviewer becomes the build:

```jsonc
// .claude/settings.json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [{
          "type": "command",
          "command": "cargo test -q --test no_raw_values --test no_painter_in_molecules 2>&1 | tail -20 || exit 2"
        }]
      }
    ]
  }
}
```

In practice: the agent invents a plausible gray, the guard fails, the agent reads the
error and reaches for the token instead. No design review required — drift is caught at
zero distance.

## Storybook

A living visual reference with every component and token:

```bash
cargo run --example storybook
```

## Docs

[Component catalog](./docs/components/README.md) (per-component pages: anatomy, variants, states, API) ·
[Architecture](./docs/architecture.md) ·
[Governance](./docs/governance.md) ·
[Layout / AutoLayout](./docs/layout.md)

## Status

Open-sourced in 2026; API pre-1.0 and evolving with the Ouroboros studio. `Mode { Dark, Light }` is first-class; the dark (zinc) palette is complete, light resolves through the same `Theme::resolve` path. Rust `1.92.0` pinned, egui `0.34.1`.

## Credits & license

Design language by [shadcn](https://ui.shadcn.com) — see [CREDITS.md](./CREDITS.md). Code MIT — see [LICENSE](./LICENSE).

---

Built by [Mauricio Juba](https://mauriciojuba.com) · [TYPE:ZERO](https://typezerolabs.com)
