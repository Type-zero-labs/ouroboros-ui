# SplitterHandle

> **Layer:** atom · **Path:** `src/atoms/splitter_handle.rs` · **Exports:** `splitter_handle::SplitterHandle`

The draggable divider band between two Splitter panels. It fills its area as a drag hit-target and paints a centered hairline (token `border`) that fades to `ring` on hover/drag, setting the resize cursor. The owning Splitter **organism** reads the returned `Response` (drag delta, double-click) — the atom paints, the organism composes.

## Design

- **Purpose / when to use** — only inside a Splitter organism, as the resize grip between panes. For a static rule use [`Divider`](divider.md).
- **Anatomy** — the full `ui.max_rect()` allocated as a `click_and_drag` hit-target → a centered `border` hairline (`vline`/`hline`) → on hover/drag/active, a `ring` overlay line at `BORDER_FOCUS`, ramped by `hover_t`. Sets `ResizeHorizontal`/`ResizeVertical` cursor while interacting.
- **Variants / sizes / states**
  - `line: Axis` — orientation of the *visible rule*: `Vertical` for a left/right (horizontal) split, `Horizontal` for a top/bottom (vertical) split.
  - `.active(bool)` — force the highlighted state (e.g. mid-drag, or when a neighbor is collapsed).
  - States: hover/drag highlight (cursor + ring fade), plus forced `active`.
- **Tokens consumed** — `theme.border` (hairline), `theme.ring` (highlight), `core::BORDER_THIN` (hairline weight), `core::BORDER_FOCUS` (highlight weight), `core::hover_t` (fade).
- **Accessibility** — sets the appropriate resize cursor while hovered/dragged; returns the full `Response` for the organism to interpret (`drag_delta`, `double_clicked`).

## API

| Signature | Effect |
|-----------|--------|
| `SplitterHandle::new(line: Axis) -> Self` | Construct; `line` = orientation of the visible rule. |
| `.active(active: bool) -> Self` | Force the highlighted state. |
| `.show(self, ui: &mut Ui) -> Response` | Allocate the band, paint, set cursor, return the `Response`. |

**`Axis`** — re-exported from [`Divider`](divider.md): `Horizontal`, `Vertical`.

## Usage

```rust
use ouroboros_ui::atoms::{SplitterHandle, Axis};

// inside a left/right split: visible rule is vertical
let resp = SplitterHandle::new(Axis::Vertical).show(ui);
if resp.dragged() {
    let dx = resp.drag_delta().x;
    // adjust the left pane width by dx
}
```

## Composition

Atom: paints the hairline/highlight directly and sets the cursor. Composes no other atoms; it is itself composed by the Splitter organism.

## Notes

- Pass the **orientation of the visible line**, not the split direction — `Axis::Vertical` for a horizontal (left/right) split. This is the documented gotcha.
- `show` consumes the whole `ui.max_rect()` as the hit-target; give it a dedicated child `Ui` sized to the desired grab width.

See [tokens](../../tokens.md) · [theming](../../theming.md).
