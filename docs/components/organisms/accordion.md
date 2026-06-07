# Accordion

> **Layer:** organism · **Path:** `src/organisms/accordion.rs` · **Exports:** `accordion::{Accordion, AccordionCtx}`

Stacked collapsible sections (shadcn Accordion). `show` hands you an [`AccordionCtx`](#accordionctx) whose `section(title, body)` appends a [`Collapsible`](../molecules/collapsible.md) molecule separated from the previous one by a horizontal [`Divider`](../atoms/divider.md). Each section owns its open/closed state in egui memory — the organism keeps no state itself.

## Design

- **Purpose / when to use** — group related option blocks that should fold away (inspector groups: Transform / Rendering / Physics). Use when only some sections need to be visible at once.
- **Anatomy** — a vertical stack; each entry is a `Collapsible` (header + body), with a `Divider::horizontal()` + `SPACE_2` padding inserted *before* every section after the first. Optionally wrapped in a card [`Surface`](../atoms/surface.md).
- **Variants / states**

  | Variant / state | How |
  |---|---|
  | plain | `Accordion::new()` — bare vertical stack |
  | card | `.card()` — wraps the stack in a `Surface::new()` card |
  | section open/closed | owned per-`Collapsible` in egui memory (not by Accordion) |

- **Tokens / layout consumed** — `core::SPACE_2` (inter-section gap); card casing via `Surface`.
- **Accessibility** — folding handled by the `Collapsible` molecule (click header to toggle).

## API

### `Accordion`

| Method | Effect |
|---|---|
| `Accordion::new() -> Self` | Bare (no card). |
| `Accordion::default()` | Same as `new()`. |
| `.card() -> Self` | Wrap the section group in a card `Surface`. |
| `.show(ui, build: impl FnOnce(&mut AccordionCtx)) -> Response` | Run `build`, which adds sections via the ctx. Returns the vertical (or `Surface`) `Response`. |

### `AccordionCtx<'u>`

Section builder handed to `show`. Holds `ui: &mut Ui` and a `first` flag internally.

| Method | Effect |
|---|---|
| `.section(title: impl Into<String>, body: impl FnOnce(&mut Ui))` | Add one collapsible section. Inserts a divider + spacing before all but the first. `body` paints arbitrary widgets into the section. |

## Usage

```rust
use ouroboros_ui::organisms::Accordion;

Accordion::new().show(ui, |acc| {
    acc.section("Transform", |ui| { /* fields */ });
    acc.section("Rendering", |ui| { /* fields */ });
});
```

```rust
// realistic — card variant with arbitrary widgets per section (from storybook)
use ouroboros_ui::organisms::Accordion;
use ouroboros_ui::molecules::{Field, VectorField};
use ouroboros_ui::atoms::{Switch, Text};

Accordion::new().card().show(ui, |acc| {
    acc.section("Transform", |ui| {
        let mut p = [1.0_f32, 0.0, -1.0];
        VectorField::new(&mut p).speed(0.05).show(ui);
    });
    acc.section("Rendering", |ui| {
        Field::new("Cast shadows")
            .horizontal()
            .show(ui, |ui| Switch::new(&mut on).show(ui));
    });
    acc.section("Physics", |ui| {
        Text::new("Collider, mass, drag").muted().show(ui);
    });
});
```

## Composition

Composes the [`Collapsible`](../molecules/collapsible.md) molecule (one per section), the [`Divider`](../atoms/divider.md) atom (separators), and optionally the [`Surface`](../atoms/surface.md) atom (card casing). Never paints directly — see [guards](../../guards.md).

## Notes

- Open/closed state lives in egui memory, owned by each `Collapsible` (keyed by its title) — not by `Accordion`. Identical section titles within one accordion would collide on memory id.
- `section` takes `FnOnce` bodies — the body closure runs immediately during `show`.
- The first section never gets a leading divider; ordering of `section` calls is the visual order.
