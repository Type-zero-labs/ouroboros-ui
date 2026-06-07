# Tabs

> **Layer:** molecule · **Path:** `src/molecules/tabs.rs` · **Exports:** `tabs::{Tabs, TabsVariant}`

A single-select tab bar bound to a `&mut usize`. Two looks: `Container` (default — segmented chips inside a [`Surface`](../atoms/surface.md), so the bar reads as a unit, not loose buttons) and `Line` (an underlined row where the active tab gets a primary rule the width of the tab). Each tab is a label with an optional leading icon. Models the shadcn / radix Tabs.

## Design

- **Purpose / when to use** — Switch between panels/views in a panel header. The molecule is the bar only; render the active panel yourself based on the selected index.
- **Anatomy** —
  - **Container**: `Surface::pad(SPACE_1).radius(RADIUS_MD)` → horizontal row of small [`Button`](../atoms/button.md)s; the active tab is `ButtonVariant::Secondary` (raised), others `Ghost`.
  - **Line**: a horizontal row of ghost [`Button`](../atoms/button.md)s; under the active one, a primary thick [`Divider`](../atoms/divider.md) sized to the tab width (`BORDER_FOCUS` tall).
- **Variants (`TabsVariant`)**

  | Variant | Look |
  |---|---|
  | `Container` *(default)* | Segmented chips in a surface; active = raised secondary button. |
  | `Line` | Underlined row; active = ghost button + primary underline. |

- **Tokens / layout consumed** — `core::SPACE_1` (surface pad / underline gap), `RADIUS_MD`, `BORDER_FOCUS` (underline thickness); `theme.primary` (underline color). See [tokens](../../tokens.md).

## API

| Method | Effect |
|---|---|
| `Tabs::new(selected: &'a mut usize) -> Self` | Bind the active index. |
| `.tabs<S: Into<String>>(tabs: impl IntoIterator<Item = S>) -> Self` | Set tabs from labels (no icons). |
| `.tab(label: impl Into<String>, icon: &'static str) -> Self` | Append one tab with a leading icon. |
| `.variant(variant: TabsVariant) -> Self` | Set the look. |
| `.line() -> Self` | Sugar for `TabsVariant::Line`. |
| `.show(self, ui: &mut Ui) -> Response` | Render; writes `*selected = i` on click. Returns the layout `Response`. |

**`TabsVariant`** — `Container` (default), `Line`.

## Usage

```rust
use ouroboros_ui::molecules::Tabs;

// minimal — container, labels only
let mut sel = 0usize;
Tabs::new(&mut sel)
    .tabs(["Overview", "Stats", "Notes"])
    .show(ui);
```

```rust
use ouroboros_ui::molecules::Tabs;
use ouroboros_ui::egui_phosphor::light;

// container with per-tab icons
Tabs::new(&mut sel)
    .tab("Scene", light::CUBE)
    .tab("Game", light::PLAY)
    .tab("Assets", light::FOLDER)
    .show(ui);

// line variant
Tabs::new(&mut sel)
    .tabs(["Overview", "Geometry", "Materials"])
    .line()
    .show(ui);

// render the active panel yourself
match sel { 0 => panel_scene(ui), 1 => panel_game(ui), _ => panel_assets(ui) }
```

## Composition

Composes [`Surface`](../atoms/surface.md) + [`Button`](../atoms/button.md) (+ [`Divider`](../atoms/divider.md) in the line variant). It never paints — the underline is a `Divider` atom, not a paint call. See the [guards](../../guards.md).

## Notes

- `.tabs(...)` replaces the tab list with icon-less tabs; `.tab(...)` pushes individual tabs — mixing them, the last `.tabs(...)` wins for the bulk and `.tab(...)` appends.
- Two-way binding via `&mut usize`; the molecule renders only the bar, not panels.
- Per-tab ids use `("tab", i)`, so multiple bars per frame are safe.
