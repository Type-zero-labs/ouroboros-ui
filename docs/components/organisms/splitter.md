# Splitter

> **Layer:** organism · **Path:** `src/organisms/splitter.rs` · **Exports:** `splitter::{PanelSpec, Splitter}`

Resizable panes split by draggable dividers (Element Plus Splitter). Horizontal (side-by-side) or vertical (stacked); each panel carries min/max bounds and may be resizable and/or collapsible. Panel sizes persist for the **session** in egui memory (keyed by `id_source`), not to disk. Composes the [`SplitterHandle`](../atoms/splitter_handle.md) atom per divider; never paints directly.

## Design

- **Purpose / when to use** — the single layout primitive: it is the root scaffold of every screen. Fixed chrome bands (header/footer/toolbar/rail) use `PanelSpec::fixed(px)` (non-resizable); the body is a flex panel; resizable regions nest another `Splitter`. Content inside each leaf panel is arranged with `AutoLayout`.
- **Anatomy** — `ui.allocate_exact_size(available)` → along the main axis, alternating panel cells (each a clipped child `Ui`) and a `SplitterHandle` divider of width `core::SPACE_2` after every panel but the last. Cross-axis fills the rect.
- **Variants / states**

  | State | How |
  |---|---|
  | orientation | `Splitter::horizontal()` / `Splitter::vertical()` |
  | panel sized | `PanelSpec::size(fraction)` (else equal share of remainder) |
  | resizing | drag a divider — grows one neighbor, shrinks the other, clamped to both `[min, max]` |
  | collapsed | double-click a divider toggles an adjacent `collapsible` panel (prefers right neighbor) |
  | non-resizable pair | a divider is inert unless **both** adjacent panels are `resizable` |

- **Tokens / layout consumed** — `core::SPACE_2` (divider thickness); `PanelSpec` defaults `layout::PANEL_MIN` / `layout::PANEL_MAX`. See [tokens](../../tokens.md) / [layout](../../layout.md).
- **Accessibility** — drag to resize, double-click to collapse; the handle atom shows the resize affordance and an `active` state when a neighbor is collapsed.

## API

### `Splitter<'a>`

| Method | Effect |
|---|---|
| `Splitter::horizontal() -> Self` | Panels left→right, dividers drag horizontally. |
| `Splitter::vertical() -> Self` | Panels top→bottom, dividers drag vertically. |
| `.id_source(id: impl Hash) -> Self` | Key for session-persisted sizes (defaults to the allocated response id). |
| `.panel(cfg: PanelSpec, add: impl FnMut(&mut Ui) + 'a) -> Self` | Add a panel with its config + content closure. |
| `.show(ui) -> Response` | Lay out panels + dividers, apply drags/toggles, persist state. Returns the allocated `Response`. |

### PanelSpec

`#[derive(Clone, Copy, Debug)]` per-panel config. Builder; pair with a content closure via `Splitter::panel`.

| Method | Effect |
|---|---|
| `PanelSpec::new() -> Self` | Defaults: `size = None` (equal share), `min = PANEL_MIN`, `max = PANEL_MAX`, `resizable = true`, `collapsible = false`. |
| `PanelSpec::default()` | Same as `new()`. |
| `.size(fraction: f32) -> Self` | Initial size as a main-axis fraction (clamped `0.0..=1.0`). |
| `.min(px: f32) -> Self` | Minimum size in px. |
| `.max(px: f32) -> Self` | Maximum size in px. |
| `.resizable(resizable: bool) -> Self` | Whether dividers touching it can drag. |
| `.collapsible(collapsible: bool) -> Self` | Whether a double-click can collapse it. |

## Usage

```rust
use ouroboros_ui::organisms::{Splitter, PanelSpec};

Splitter::horizontal()
    .id_source("editor")
    .panel(PanelSpec::new().min(180.0).max(420.0), |ui| { /* hierarchy */ })
    .panel(PanelSpec::new(), |ui| { /* viewport */ })
    .panel(PanelSpec::new().collapsible(true), |ui| { /* inspector */ })
    .show(ui);
```

```rust
// realistic — nested vertical split inside a horizontal one (from storybook)
use ouroboros_ui::organisms::{Splitter, PanelSpec};

Splitter::horizontal()
    .id_source("outer")
    .panel(PanelSpec::new().min(120.0).max(280.0), |ui| panel(ui, "Hierarchy"))
    .panel(PanelSpec::new(), |ui| {
        Splitter::vertical()
            .id_source("inner")
            .panel(PanelSpec::new(), |ui| panel(ui, "Viewport"))
            .panel(PanelSpec::new().size(0.3).collapsible(true), |ui| panel(ui, "Console"))
            .show(ui);
    })
    .panel(PanelSpec::new().min(160.0).collapsible(true), |ui| panel(ui, "Inspector"))
    .show(ui);
```

## Composition

Composes the [`SplitterHandle`](../atoms/splitter_handle.md) atom (one per divider, `Axis::Vertical` for horizontal splitters and vice-versa) plus your panel content closures into clipped child `Ui`s. It never paints — see [guards](../../guards.md).

## Notes

- **State ownership** — fractions + collapse flags persist for the session in egui memory (`SplitterState`, keyed by `id_source`). State resets if the panel **count** changes (stored fracs length must match `n`). Distinct splitters need distinct `id_source` — nested splitters especially.
- Resizing follows the adjacent-pair rule (one neighbor grows, the other shrinks), with `apply_drag` clamping `a` to a range that honors both panels' `[min, max]`; if the bounds can't be jointly satisfied the drag is ignored.
- Collapse prefers the **right** neighbor of the divider if collapsible, else the left; collapsed panels contribute zero and their fraction is redistributed.
- A divider only drags when **both** adjacent panels are `resizable`.
- `show` consumes `ui.available_size()`; constrain via `allocate_ui` if needed.
