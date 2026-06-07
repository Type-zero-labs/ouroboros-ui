# Layout & auto-layout

Two pieces: **layout tokens** (`src/tokens/layout.rs`) — fixed dimensions and z-order
roles a layout reads — and the **`AutoLayout`** engine (`src/auto_layout.rs`) — a
Figma-style flow layout for egui.

---

## Layout tokens

egui is immediate-mode (no CSS grid), so these are *primitives* a component or helper
reads. Tune them to the real studio shell.

### Panels (px)

| Const | px | Role |
|-------|----|------|
| `SIDEBAR_WIDTH` | 240 | left nav / tree sidebar |
| `INSPECTOR_WIDTH` | 300 | right properties / inspector |
| `PANEL_MIN` | 180 | min a resizable panel may shrink to |
| `PANEL_MAX` | 480 | max a resizable panel may grow to |
| `TOOLBAR_HEIGHT` | 40 | top toolbar |
| `STATUSBAR_HEIGHT` | 24 | bottom status bar |

### Content grid

`GRID_COLUMNS` 12 · `GRID_GUTTER` 16 (= `SPACE_4`) · `CONTAINER_MAX` 1200 (max readable
width before centering).

### Breakpoints (window width, px)

| Const | px | Below this |
|-------|----|------------|
| `BREAKPOINT_COMPACT` | 720 | compact — single column, collapsed panels |
| `BREAKPOINT_NORMAL` | 1024 | normal — one side panel |
| `BREAKPOINT_WIDE` | 1440 | wide — both side panels, roomy |

Component-level thresholds: `FIELD_HORIZONTAL_MIN` 480 (a [`Field`](./components/molecules/field.md)
goes side-by-side at/above this, else stacks), `PROPERTY_LABEL_WIDTH` 120 (fixed label
column for [`PropertyRow`](./components/cells/property_row.md)), `TABLE_ROW_HEIGHT` 28.

### `SizeClass`

```rust
pub enum SizeClass { Compact, Normal, Wide }
SizeClass::from_width(available_width) -> SizeClass
```

Classifies an available width against the breakpoints (`< NORMAL` → Compact,
`< WIDE` → Normal, else Wide) so a component can adapt density.

### `Layer` — z-order roles

Stacking roles for floating surfaces, mapped onto `egui::Order`. Ordered base → tooltip.

```rust
pub enum Layer { Base, Dropdown, Popover, Modal, Toast, Tooltip }
```

| Method | Returns |
|--------|---------|
| `order()` | the `egui::Order` (Base→Middle; Dropdown/Popover/Modal/Toast→Foreground; Tooltip→Tooltip) |
| `priority()` | relative priority within a shared order (higher = on top; the enum's discriminant) |

egui's order set is coarse; finer ordering within a layer is by creation/`priority`.

---

## `AutoLayout` — Figma-style flow

A flexbox-like flow layout for egui that mirrors the **exact vocabulary** of the studio's
HUD model (`ouroboros-hud::model`) — `LayoutDirection`, `MainAlign`, `CrossAlign`, `Gap`,
`Padding`, `SizeMode` — so designers get one mental model across the engine HUD and the
studio UI. It is re-declared (not imported) to keep `ouroboros-ui` standalone.

### Model

| Type | Variants | Meaning |
|------|----------|---------|
| `LayoutDirection` | `Horizontal`, `Vertical` (default) | the main axis children flow along |
| `MainAlign` | `Start` (default), `Center`, `End` | alignment of the child block on the main axis |
| `CrossAlign` | `Start` (default), `Center`, `End` | per-child alignment on the cross axis |
| `Gap` | `Fixed(px)` (default 0), `Auto` | spacing; `Auto` = space-between (distributes leftover, ignores `MainAlign`) |
| `SizeMode` | `Fixed(px)`, `Hug` (default), `Fill` | per-child main-axis sizing |

`Padding` — `all(v)` or `symmetric(x, y)`; fields `top/right/bottom/left`.

**`SizeMode` per child:** `Hug` sizes to content, `Fixed(px)` is exact, `Fill` grows to
share leftover main-axis space (an empty `Fill` child is a flexible spacer).

### Builder

```rust
AutoLayout::horizontal()  // or ::vertical()
    .gap(8.0)                          // fixed gap…
    .gap_auto()                        // …or space-between
    .pad(12.0)                         // .padding(Padding) / .pad_xy(x, y)
    .main_align(MainAlign::Center)
    .cross_align(CrossAlign::Center)
    .fixed(28.0, |ui| { /* icon */ })  // child with fixed main size
    .fill(|ui| {})                     // flexible spacer / growing child
    .hug(|ui| { /* button */ })        // child sized to content
    .child(SizeMode::Fill, |ui| {})    // explicit form
    .show(ui) -> Response
```

### Example — toolbar with a trailing button

```rust
AutoLayout::horizontal()
    .gap(8.0).pad(12.0).cross_align(CrossAlign::Center)
    .fixed(28.0, |ui| { Icon::new(GEAR).show(ui); })
    .fill(|ui| {})                 // spacer pushes the next child to the end
    .hug(|ui| { Button::new("Save").show(ui); })
    .show(ui);
```

### How it works

Child closures are `FnMut`: they run **once invisibly to measure** (a `sizing_pass` ui),
then **once for real** at computed cells. The algorithm:

1. **Measure pass** — render each child invisibly to get natural main/cross sizes;
   count `Fill` children.
2. **Container sizing** — if any `Fill`, `Gap::Auto`, or non-`Start` align is present, the
   frame claims the available main-axis space; otherwise it hugs its content.
3. **Distribution** — leftover space goes to: `Auto` → even gaps between children;
   `Fill` children → shared equally; otherwise → a start offset per `MainAlign`.
4. **Render** — allocate the frame, then place each child in an explicit cell rect.

> **Cost note:** every child renders twice (measure + real). It is cheap for typical
> toolbar/row counts, but don't nest deeply with heavy children in a hot per-frame path.
</content>
