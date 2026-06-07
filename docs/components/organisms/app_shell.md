# AppShell

> **Layer:** organism · **Path:** `src/organisms/app_shell.rs` · **Exports:** `app_shell::AppShell`

A view scaffold with five named slots (Element Plus Container). `header` and `footer` span the full width at fixed heights; the middle band lays `aside_left · main · aside_right` as a horizontal [`Splitter`](splitter.md), so the asides are drag-resizable by default while header/footer stay fixed. Layout is rect-math inside the `ui` it is given (not `egui::SidePanel`), so it composes anywhere and **nests** — an `AppShell` can live inside another's slot.

## Design

- **Purpose / when to use** — the top-level frame of an editor/IDE view: toolbar on top, hierarchy left, viewport center, inspector right, status bar bottom. Also any nested sub-region needing the same scaffold.
- **Anatomy** — three vertical bands computed by rect-math:
  - **header** band (top, full width, `header_height`)
  - **footer** band (bottom, full width, `footer_height`)
  - **middle** band — a horizontal `Splitter` with up to three panels: `aside_left` (sized from `aside_left_width`, clamped `PANEL_MIN`/`PANEL_MAX`), `main` (default `PanelSpec`, takes remainder), `aside_right` (sized from `aside_right_width`, clamped). The splitter is keyed `(id, "appshell_mid")`.

  Each present slot renders into a clipped child `Ui` (`new_child` + `set_clip_rect`). Omitted slots collapse to zero (header/footer heights drop to `0.0` when their slot is `None`).
- **Variants / states**

  | State | How |
  |---|---|
  | slots present/absent | any of the five slots optional; absent header/footer take no vertical space |
  | aside resizing | middle band is a `Splitter`, so asides drag-resize (session-persisted) |
  | nested | an `AppShell` in a slot — give it a distinct `id_source` to avoid splitter-state collisions |

- **Tokens / layout consumed** — defaults from [`tokens::layout`](../../layout.md): `TOOLBAR_HEIGHT` (header), `STATUSBAR_HEIGHT` (footer), `SIDEBAR_WIDTH` (left aside), `INSPECTOR_WIDTH` (right aside), `PANEL_MIN`/`PANEL_MAX` (aside splitter clamps). Aside widths are converted to a main-axis fraction and clamped to `0.05..=0.45`.
- **Accessibility** — n/a (layout scaffold; behavior comes from slot content and the splitter).

## API

| Method | Effect |
|---|---|
| `AppShell::new() -> Self` | New shell with layout defaults, no slots. |
| `AppShell::default()` | Same as `new()`. |
| `.header(add: impl FnMut(&mut Ui) + 'a) -> Self` | Set the top full-width slot. |
| `.aside_left(add: impl FnMut(&mut Ui) + 'a) -> Self` | Set the left middle panel. |
| `.main(add: impl FnMut(&mut Ui) + 'a) -> Self` | Set the center middle panel (remainder). |
| `.aside_right(add: impl FnMut(&mut Ui) + 'a) -> Self` | Set the right middle panel. |
| `.footer(add: impl FnMut(&mut Ui) + 'a) -> Self` | Set the bottom full-width slot. |
| `.header_height(px: f32) -> Self` | Override header band height. |
| `.footer_height(px: f32) -> Self` | Override footer band height. |
| `.aside_left_width(px: f32) -> Self` | Override left aside initial width (px → fraction). |
| `.aside_right_width(px: f32) -> Self` | Override right aside initial width. |
| `.id_source(id: impl Hash) -> Self` | Stable id (seeds the middle splitter's session state). |
| `.show(ui) -> Response` | Allocate `ui.available_size()`, render bands, return the allocated `Response`. |

Slot closures are `FnMut` boxed as `Slot<'a> = Option<Box<dyn FnMut(&mut Ui) + 'a>>`.

## Usage

```rust
use ouroboros_ui::organisms::AppShell;

AppShell::new()
    .header(|ui| { /* toolbar */ })
    .aside_left(|ui| { /* hierarchy */ })
    .main(|ui| { /* viewport */ })
    .aside_right(|ui| { /* inspector */ })
    .footer(|ui| { /* status bar */ })
    .show(ui);
```

```rust
// realistic — nested shell with a stable id (from storybook)
use ouroboros_ui::organisms::AppShell;

AppShell::new()
    .id_source("outer")
    .header(|ui| panel(ui, "Toolbar"))
    .main(|ui| {
        AppShell::new()
            .id_source("inner")
            .aside_left(|ui| panel(ui, "Sub-nav"))
            .main(|ui| panel(ui, "Document"))
            .show(ui);
    })
    .footer(|ui| panel(ui, "Status"))
    .show(ui);
```

## Composition

Composes plain closures for header/footer and the [`Splitter`](splitter.md) + [`PanelSpec`](splitter.md#panelspec) organism for the middle band. It does not use `egui::SidePanel`/`TopBottomPanel` — bands are placed by `Rect` math into clipped child `Ui`s. It never paints — see [guards](../../guards.md).

## Notes

- **State ownership** — the middle splitter persists aside sizes for the session in egui memory, keyed `(id, "appshell_mid")`. Nested shells **must** use distinct `id_source` values or their splitter states collide.
- `aside_left_width` / `aside_right_width` are *initial* hints converted to fractions and clamped to `0.05..=0.45` of the middle band; the splitter then governs live resizing.
- `main` is the only unbounded panel (default `PanelSpec`), absorbing leftover width.
- `show` consumes the full `available_size`; constrain via `ui.allocate_ui(...)` or a parent slot if you need a bounded region.
