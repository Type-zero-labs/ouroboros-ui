# ToggleGroup

> **Layer:** molecule · **Path:** `src/molecules/toggle_group.rs` · **Exports:** `toggle_group::ToggleGroup`

A segmented single-select control bound to a `&mut usize`. Options render as a connected row of [`Button`](../atoms/button.md)s inside a [`Surface`](../atoms/surface.md) container: the selected segment is a raised `Secondary` button (looks like a real button), the rest `Ghost`. Models the shadcn Toggle Group / Button Group.

## Design

- **Purpose / when to use** — Compact mutually-exclusive choice with short labels (gizmo space: Local/World, alignment, view mode). For longer labels or descriptions use [`RadioGroup`](radio_group.md)/[`RadioCard`](radio_card.md).
- **Anatomy** — `Surface::pad(SPACE_1).radius(RADIUS_MD)` → horizontal row of small [`Button`](../atoms/button.md)s; active = `ButtonVariant::Secondary`, others `ButtonVariant::Ghost`.
- **States** — exactly one segment is the raised secondary button (`*selected == i`).
- **Tokens / layout consumed** — `core::SPACE_1` (surface pad), `RADIUS_MD`. See [tokens](../../tokens.md).

## API

| Method | Effect |
|---|---|
| `ToggleGroup::new(selected: &'a mut usize) -> Self` | Bind the active index. |
| `.options<S: Into<String>>(options: impl IntoIterator<Item = S>) -> Self` | Set the segment labels. |
| `.show(self, ui: &mut Ui) -> Response` | Render; writes `*selected = i` on click. Returns the surface `Response`. |

## Usage

```rust
use ouroboros_ui::molecules::ToggleGroup;

// minimal
let mut sel = 0usize;
ToggleGroup::new(&mut sel)
    .options(["Local", "World"])
    .show(ui);
```

## Composition

Composes [`Surface`](../atoms/surface.md) + [`Button`](../atoms/button.md). It never paints — see the [guards](../../guards.md).

## Notes

- Two-way binding via `&mut usize`.
- Per-segment ids use `("toggle_group", i)`, so multiple groups per frame are safe.
- Visually near-identical to [`Tabs`](tabs.md) `Container`, but semantically a value selector rather than a view switcher.
