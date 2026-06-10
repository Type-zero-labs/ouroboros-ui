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
| `Sizing` | `{ mode, min, max }` | a `SizeMode` plus optional px clamps (a bare `SizeMode` converts) |

`Padding` — `all(v)` or `symmetric(x, y)`; fields `top/right/bottom/left`.

### `Sizing` — mode × min/max

Each child's main-axis size is a `Sizing`: a mode plus optional `min`/`max` floors and
ceilings (min wins over max, like the HUD solver). Constructors are `const`:
`Sizing::fixed(px)` / `::hug()` / `::fill()`, then `.min(px)` / `.max(px)` /
`.clamped(min, max)`.

| Mode | Without clamps | `min` | `max` |
|------|----------------|-------|-------|
| `Fixed(px)` | exactly `px` | floors `px` | caps `px` |
| `Hug` | sizes to content (bounded by the budget) | never shrinks below `min`, even when content is smaller | caps content, even when content wants more |
| `Fill` | shares leftover space with other fills | never shrinks below `min` — a responsive column that won't collapse | stops growing at `max`; the excess is redistributed to the other fills |

`Hug` measures content *against the budget*: a greedy child (one that expands to
`available_width`) measures as the whole budget — for "should fill" controls use `Fill`
(optionally clamped) instead of `Hug`.

### Builder

```rust
AutoLayout::horizontal()  // or ::vertical()
    .gap(8.0)                          // fixed gap…
    .gap_auto()                        // …or space-between
    .gap_cross(8.0)                    // gap between wrapped lines (defaults to main gap)
    .pad(12.0)                         // .padding(Padding) / .pad_xy(x, y)
    .main_align(MainAlign::Center)
    .cross_align(CrossAlign::Center)
    .wrap()                            // reflow onto new lines (horizontal only)
    .allow_overflow()                  // opt out of budget clamping + cell clipping
    .fixed(28.0, |ui| { /* icon */ })  // child with fixed main size
    .fill(|ui| {})                     // flexible spacer / growing child
    .fill_min(220.0, |ui| {})          // fill that floors at 220px
    .fill_clamped(80.0, 160.0, |ui| {})// fill clamped to [80, 160]px
    .hug(|ui| { /* button */ })        // child sized to content
    .hug_max(240.0, |ui| {})           // hug capped at 240px
    .child(SizeMode::Fill, |ui| {})    // explicit form…
    .sized(Sizing::fill().min(120.0), |ui| {}) // …or with a prebuilt Sizing
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

### Example — responsive columns (`fill_min`)

Two form columns that share the panel but never collapse below a readable width — when
the panel is squeezed under `2 × 220 + gap`, the cells keep their floors and the frame
clips as a last resort instead of overlapping:

```rust
AutoLayout::horizontal()
    .gap(24.0)
    .fill_min(220.0, |ui| left_column(ui))
    .fill_min(220.0, |ui| right_column(ui))
    .show(ui);
```

### Example — stat grid (`wrap`)

One row when wide, reflowing to 2–3 lines when narrow; each cell floors at 72px and the
fills on a line share that line's remainder:

```rust
AutoLayout::horizontal().wrap().gap(8.0).gap_cross(8.0)
    .fill_min(72.0, |ui| stat(ui, "STR"))
    .fill_min(72.0, |ui| stat(ui, "AGI"))
    // … 4 more cells
    .show(ui);
```

### How it works

Child closures are `FnMut`: they run **once invisibly to measure** (a `sizing_pass` ui),
then **once for real** at computed cells. The algorithm:

1. **Measure pass A (bounded)** — render each `Fixed`/`Hug` child invisibly, bounded on
   *both* axes by the frame's budget (the available space), and clamp `Hug` by its
   `min`/`max`. Content can never measure wider than the panel it lives in.
2. **Resolve `Fill`** — distribute the leftover main-axis space among `Fill` children,
   `min`/`max`-aware: whoever clamps is pinned and its excess is redistributed among the
   rest (the HUD solver's `distribute_fill`).
3. **Measure pass B** — measure each `Fill` child's *cross* size at its **resolved** main
   size, so wrapping content (labels, alerts) reports its real height.
4. **Container sizing** — the frame never exceeds a finite budget: with `Fill`,
   `Gap::Auto`, or non-`Start` align it claims the available main axis; otherwise it hugs
   content, clamped to the budget.
5. **Distribution** — leftover space goes to: `Auto` → even gaps between children;
   `Fill` children → already consumed; otherwise → a start offset per `MainAlign`.
6. **Render** — allocate the frame, then place each child in an explicit cell rect. Each
   cell is **clipped as a last resort** (with a small bleed for focus rings): with correct
   sizing it never bites, it only stops legitimate overflow (e.g. floors inside a panel
   squeezed below their sum) from painting over siblings.

`allow_overflow()` opts out of both the budget clamp and the cell clipping — the legacy
behavior, for the rare container that scrolls itself.

### Wrap

`wrap()` (horizontal only) reflows children onto new lines when they don't fit — Figma's
"wrap". Line breaking is greedy over each child's intrinsic contribution (`Fixed`/`Hug` →
natural size, `Fill` → its `min` or 0), with at least one child per line; then each line
is laid out like a non-wrapping row, so a `Fill` child takes the remainder of *its* line.
Spacing between lines comes from `gap_cross(px)` (defaults to the main gap). Not
supported by the rect-returning `layout()` path.

### Responsive contract (anti-ratchet)

The frame's budget comes from the parent — a `Splitter` panel rect, a window — which is
*exogenous to the content*. Because measurement is bounded by that budget and never feeds
back into it, layout is **idempotent per frame**: dragging a panel out and back yields the
same rects, with no ratchet (content can't "remember" the widest it ever was). Inside a
scroll axis there is no finite budget; measurement is effectively unbounded there and
`Fill` resolves to its floor.

> **Cost note:** every child renders twice (measure + real). It is cheap for typical
> toolbar/row counts, but don't nest deeply with heavy children in a hot per-frame path.
