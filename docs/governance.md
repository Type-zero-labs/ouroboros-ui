# Governance — use first, extend second, create last

This is the law of the design system. [architecture.md](./architecture.md) explains how
the system is built; this page rules how it is **consumed and grown**. Every rule here is
mechanically enforced (see [Enforcement](#enforcement)) — a violation is a red build, not
a review nit.

---

## The rule in one sentence

> **Studio UI is built from `ouro_ds` components. If a component is missing a capability,
> you contribute it to the DS — you never hand-roll raw egui chrome and never hardcode a
> color outside a canvas.**

The studio (and every studio crate) is a *consumer*. The DS is the single place visual
decisions live, so one fix retunes every screen, the Dark/Light toggle keeps working
everywhere, and "did someone hardcode a color" stays a CI failure instead of a judgment
call.

---

## Decision ladder

When you need a piece of UI, walk this ladder **top-down** and stop at the first rung
that works.

### 1. Use — the catalog first

The DS has **71 components** across atoms → cells → molecules → organisms plus the graph
peer layer. Before building anything, check whether it already exists:

- **[Component catalog](./components/README.md)** — every component documented by layer,
  with design intent, API, and usage examples.
- **The storybook** — the living visual reference, every component and token rendered:

  ```bash
  cargo run --example storybook
  ```

Most "I need a custom widget" cases are an existing component plus a builder you hadn't
seen (`Text` alone has roles, `.muted()`, `.color()`, `.wrap()`, `.underline()`,
`.italic()`).

### 2. Extend — a new builder on an existing component

If the component exists but lacks one capability, add a **builder setter** to it instead
of forking or hand-rolling. Real example: the studio needed checkable View-menu rows;
instead of `ui.checkbox(..)` inside a menu, `MenuItem` gained
[`.checked(bool)`](./components/cells/menu_item.md) — a check mark when `true`, a
reserved slot when `false` so siblings stay aligned. One small PR, and every menu in
every studio crate can now have toggle items.

An extension still walks the contribution pipeline below (tokens → storybook → test →
doc), just scoped to the new setter.

### 3. Create — a new component

Only when *use* and *extend* genuinely don't fit: the piece is a new shape, not a variant
of an existing one. Follow the
[component contribution pipeline](#component-contribution-pipeline) — starting with the
spec-lite paragraph that says *why* the first two rungs were not enough.

---

## What is forbidden in studio chrome

The studio's `ds_governance` guard scans every studio crate and **hard-fails** on the
patterns below. This table is the exact mirror of that guard — the substitution is always
a DS component or a theme token.

| Forbidden pattern | Use instead |
|-------------------|-------------|
| `ui.label(..)` | `atoms::Text` |
| `ui.button(..)` | `atoms::Button` |
| `ui.checkbox(..)` | `atoms::Checkbox` / `cells::MenuItem::checked` |
| `ui.separator()` | `atoms::Divider::horizontal()` / `::vertical()` |
| `ui.heading(..)` | `atoms::Heading` |
| `ui.selectable_label(..)` | `cells::ListItem::new(..).selected(..)` / `cells::MenuItem` |
| `ui.text_edit_*` / `TextEdit::*` | `atoms::Input` / `atoms::Textarea` |
| `ComboBox::*` | `organisms::Select` |
| `DragValue` | `atoms::NumericField` |
| `egui::Slider::*` | `atoms::Slider` |
| `RichText::new(..)` | `atoms::Text` builders (`.muted()` / `.caption()` / `.color()` / `.wrap()` / `.italic()`) |
| `egui::Button::new(..)` | `atoms::Button` / `cells::MenuItem` |
| `ui.radio_value(..)` | `atoms::Radio` / `molecules::ToggleGroup` |
| `Color32::from_*(<literal>)` and `Color32::<CONST>` (except `TRANSPARENT`) | `Theme` tokens (`theme.hover_overlay`, `theme.muted`, `theme.scrim`, `theme.info`, …) |
| `FontId::*` | `theme::typography` |

> **`ui.menu_button` is allowed** — as a *container*. egui owns the popup placement; the
> rows inside it are `cells::MenuItem` / `atoms::Divider`, never raw widgets.

---

## Escapes

Two sanctioned escape hatches, both designed to be visible — never silent.

### `CANVAS_ALLOWLIST` — content rendering is not chrome

Some studio modules render *content*: world viewport canvases, node-graph surfaces,
sprite previews. There, a color is **data** (a tile's terrain hue, an event wire), not a
design decision — painting pixels is the feature. These paths are listed in the
`CANVAS_ALLOWLIST` of the studio guard
(`crates/ouroboros-studio/tests/ds_governance.rs`):

```
ouroboros-studio/src/modules/events/canvas.rs
ouroboros-studio/src/modules/interface/canvas.rs
ouroboros-studio/src/modules/world/ui/map_canvas/
ouroboros-level-editor/src/render/
ouroboros-sprite-studio/src/render/
ouroboros-visualizer/src/render/
```

Inside the allowlist, the **color/paint class** is exempt. The **chrome class is not** —
a toolbar over a canvas is still built from DS components. A stale allowlist entry (file
moved or renamed) fails the guard, so the list can't rot.

### `// ds-allow: <reason>` — the one-off escape

For a single legitimate exception, annotate the line (or the line directly above):

```rust
// ds-allow: egui demo window needs its own raw label for the comparison screenshot
ui.label("raw egui baseline");
```

The **reason is mandatory** — a bare `ds-allow:` is itself a violation. The annotation is
the review trail: every escape is grep-able and visible in the diff, so a reviewer can
challenge it.

---

## Component contribution pipeline

How a new component (or an extension to one) enters the DS. The
`mad.component` skill in `claude-skills` walks these steps guided.

### 0. Spec-lite

One paragraph: the concrete use case, and **why *use* and *extend* are not enough**. This
goes in the PR description. If you can't write the paragraph, you're on the wrong rung of
the ladder.

### 1. Pick the layer

- **atom** — it needs to *paint* primitives (fill, stroke, galley). Atoms are the only
  layer allowed to touch the painter.
- **cell / molecule / organism** — it *composes* existing pieces. These layers **never
  paint** — enforced by the `no_painter_in_molecules` guard. If you reach for a painter
  here, stop: the missing piece becomes an atom first.
- **graph** — the node-editor peer layer; paints, but only through tokens.

See [architecture.md](./architecture.md) for the full layer model.

### 2. Tokens first

Any **new visual value** (a color, a spacing, a radius, an overlay alpha) becomes a token
before it is used: a `core` primitive in `src/tokens/core.rs`, surfaced through a `Theme`
field in `src/tokens/semantic.rs` if it carries meaning. Never a literal in the component
— the `no_raw_values` guard rejects it.

### 3. Builder pattern

Every component, every layer, the same shape:

```rust
Component::new(required_args)
    .setter(value)   // chainable, returns Self
    .show(ui)        // consumes self, returns egui::Response
```

### 4. Storybook

Add an entry to the `Page` enum and a page (or extend the existing page) in
`examples/storybook.rs`. Without a demo there is nothing to validate visually — and
nothing for the next person walking rung 1 to find.

### 5. Test

A kittest test in `tests/atoms.rs` (or the layer's suite): the `Harness` +
`Theme::install` pattern — at minimum a smoke render, ideally an interaction or layout
assertion.

### 6. Doc

A page in `docs/components/<layer>/<name>.md` following the
[page template](./components/README.md#page-template), plus a line in the
[catalog index](./components/README.md). For an extension: update the existing page's
API table.

### 7. Green build

```bash
cargo test                                  # includes the guards
cargo clippy --all-targets -- -D warnings
cargo fmt
```

### 8. Ship

PR against `ouroboros-ui` (`develop`) → merge → bump the `ui/` submodule in the studio
repo (with the follow-up studio PR adopting the new API, when applicable).

---

## Enforcement

The ladder is not honor-system. Three layers of machinery:

### The DS's own guards (this repo, run in CI)

- [`tests/no_raw_values.rs`](./guards.md#guard-1--atoms-and-graph-paint-only-with-tokens)
  — atoms and graph paint only with tokens: no literal `Color32`, no named color consts,
  no raw `FontId` / stroke / radius.
- [`tests/no_painter_in_molecules.rs`](./guards.md#guard-2--above-atoms-you-compose-never-paint)
  — cells / molecules / organisms compose, never paint.

Both run with `cargo test`, and CI runs the **full test suite** — the guards are part of
the gate, not optional extras. Details: [guards.md](./guards.md).

### The studio guard (consumer side)

`crates/ouroboros-studio/tests/ds_governance.rs` in the studio repo — hard fail, two
classes:

1. **Chrome class** — the raw-widget patterns from the
   [table above](#what-is-forbidden-in-studio-chrome), scanned across every studio crate.
   Escape: `// ds-allow: <reason>`.
2. **Color/paint class** — literal `Color32` values and `FontId` construction outside the
   `CANVAS_ALLOWLIST`.

### Clippy `disallowed_methods`

The studio's `clippy.toml` disallows the constructors clippy can resolve by path —
`egui::ComboBox::*`, `egui::DragValue::new`, `egui::Slider::new` — so those are caught
at lint time with a pointer back to this document. The guard test covers the
method-call patterns (`ui.label(..)`-style) that clippy cannot.
