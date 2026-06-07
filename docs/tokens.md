# Tokens — core primitives

`src/tokens/core.rs`. The bottom layer: raw `const` values with **no semantic meaning**.
The [semantic layer](./theming.md) maps meaning onto these; nothing here references
anything else. Every value was decided interactively, value-by-value, against shadcn.

> **Design rule:** atoms never read a raw hex. They read either a `core::*` primitive
> (for structural values like spacing/radius) or a `Theme` field (for colors). The
> `no_raw_values` guard enforces it.

---

## Color ramps

### Neutral base — Zinc (cool-neutral)

The temperature of every gray surface/border/text token. Tailwind/shadcn `zinc`, 50→950.

| Const | RGB | Const | RGB |
|-------|-----|-------|-----|
| `ZINC_50`  | `250 250 250` | `ZINC_600` | `82 82 91` |
| `ZINC_100` | `244 244 245` | `ZINC_700` | `63 63 70` |
| `ZINC_200` | `228 228 231` | `ZINC_800` | `39 39 42` |
| `ZINC_300` | `212 212 216` | `ZINC_900` | `24 24 27` |
| `ZINC_400` | `161 161 170` | `ZINC_950` | `9 9 11` |
| `ZINC_500` | `113 113 122` | | |

### Brand — Ouroboros turquoise (Teal)

The **primary** hue: buttons, progress/slider fill, switch-on, focus ring, selection.
Kept light (300/400 in dark) per brand. The zinc ramp is left untouched, so a pure-zinc
theme stays available (see `Theme::zinc_dark`).

| Const | RGB |
|-------|-----|
| `TEAL_200` | `153 246 228` |
| `TEAL_300` | `94 234 212` |
| `TEAL_400` | `45 212 191` |
| `TEAL_500` | `20 184 166` |
| `TEAL_600` | `13 148 136` |

### Status hues — Tailwind 500

The semantic layer composites the soft `*_bg` variants by applying ~15% alpha
(`STATUS_BG_ALPHA = 38`) to these.

| Const | Meaning | RGB |
|-------|---------|-----|
| `GREEN_500` | success | `34 197 94` |
| `RED_500` | error / destructive | `239 68 68` |
| `AMBER_500` | warning | `245 158 11` |
| `BLUE_400` | info (text) | `96 165 250` |
| `BLUE_500` | info (fill base) | `59 130 246` |

---

## Spacing — 4px base

Tailwind numeric keys (key `N` = `N × 4px`). Contiguous 1–6, then 8/10/12 for larger
gaps. Used for padding, gaps, margins.

| Const | px | Const | px |
|-------|----|-------|----|
| `SPACE_0` | 0 | `SPACE_5` | 20 |
| `SPACE_1` | 4 | `SPACE_6` | 24 |
| `SPACE_2` | 8 | `SPACE_8` | 32 |
| `SPACE_3` | 12 | `SPACE_10` | 40 |
| `SPACE_4` | 16 | `SPACE_12` | 48 |

`SPACE_0` is the semantic "no gap / no padding" sentinel (tight tables, full-bleed).

## Corner radius

shadcn classic base (0.5rem). `FULL` is the pill/circle sentinel.

| Const | px | | Const | px |
|-------|----|---|-------|----|
| `RADIUS_NONE` | 0 | | `RADIUS_LG` | 8 |
| `RADIUS_SM` | 4 | | `RADIUS_XL` | 12 |
| `RADIUS_MD` | 6 | | `RADIUS_FULL` | 9999 |

`RADIUS_NONE` is the "square corners" sentinel (full-bleed rows, flush panels).

## Shadows

Dark-tuned (high alpha to read on the zinc background).

| Const | Use | offset / blur / alpha |
|-------|-----|------------------------|
| `SHADOW_SM` | fields, chips | `[0,1]` / 2 / 61 |
| `SHADOW_MD` | cards, pills, popovers | `[0,2]` / 4 / 82 |
| `SHADOW_LG` | modals, overlays | `[0,8]` / 24 / 48 |

`shadow(offset, blur, spread, color)` is a `const fn` builder for elevations beyond the
fixed triple (egui's `Shadow` is foreign, hence a free fn not an inherent constructor).

---

## Typography primitives

Raw values only — the [typography layer](./typography.md) composes these into named styles.

**Type sizes (px)** — dense IDE calibration; body anchors at `TEXT_BASE` (14).

| Const | px | Const | px |
|-------|----|-------|----|
| `TEXT_XS` | 12 | `TEXT_XL` | 20 |
| `TEXT_SM` | 13 | `TEXT_2XL` | 24 |
| `TEXT_BASE` | 14 | `TEXT_3XL` | 30 |
| `TEXT_LG` | 16 | | |

**Line-height multipliers** (× font size): `LEADING_TIGHT` 1.2 (headings/display),
`LEADING_NORMAL` 1.45 (body), `LEADING_RELAXED` 1.6 (long-form).

**Letter-spacing (px)** — scale is *inverse to size* for legibility (big titles stay
`NORMAL`, smaller text gets wider tracking): `TRACKING_TIGHT` −0.25, `TRACKING_NORMAL` 0,
`TRACKING_SM` 0.4, `TRACKING_MD` 0.6, `TRACKING_LG` 0.8, `TRACKING_WIDE` 1.0.

---

## Sizing

**Control heights** — `CONTROL_SM` 26 · `CONTROL_MD` 32 · `CONTROL_LG` 38.
**Icon box** — `ICON_SM` 14 · `ICON_MD` 16 · `ICON_LG` 20 · `ICON_XL` 24.
**Strokes** — `BORDER_THIN` 1 (divider) · `BORDER_FOCUS` 2 (focus ring) · `RING_OFFSET` 2
(gap to ring). **Hit target** — `HIT_MIN` 32 (minimum interactive size).

### `Size` enum — the shared control scale

One source of truth for every form control's footprint, so density (compact toolbar vs.
roomy panel) is expressible uniformly.

```rust
pub enum Size { Sm, Md /* default */, Lg }
```

| Method | Sm | Md | Lg |
|--------|----|----|----|
| `height()` | 26 | 32 | 38 |
| `icon_size()` | 14 | 16 | 20 |
| `pad_x()` | 12 | 16 | 16 |
| `text_style()`¹ | label | label | body_strong |

¹ defined in `theme::typography` (keeps `core` a leaf — see [typography.md](./typography.md)).

---

## Motion

Animation durations (seconds) + easing curves. egui drives hover/focus transitions by
duration (`ctx.animate_*`); `Easing` shapes the progress.

**Durations** — `DURATION_INSTANT` 0 · `DURATION_FAST` 0.10 · `DURATION_NORMAL` 0.18 ·
`DURATION_SLOW` 0.30. **Delays** — `DURATION_DELAY_SHORT` 0.15 · `DURATION_DELAY_LONG` 0.50.

### `Easing` enum

```rust
pub enum Easing { Linear, EaseOut /* default */, EaseInOut, Spring, Bounce }
```

| Variant | Curve | Use |
|---------|-------|-----|
| `Linear` | identity | constant motion |
| `EaseOut` | decelerate | enter/hover (the default) |
| `EaseInOut` | accel then decel | moves/reorders |
| `Spring` | overshoot then settle (ease-out-back) | playful enters, springy toggles |
| `Bounce` | decaying bounces (ease-out-bounce) | drops, attention pulls |

`Easing::apply(t)` maps a normalized progress `t ∈ 0..=1` through the curve.

---

## Opacity & overlays

| Const | Value | Meaning |
|-------|-------|---------|
| `OPACITY_DISABLED` | 0.5 | the disabled veil |
| `OPACITY_MUTED` | 0.7 | secondary/muted content |
| `HOVER_OVERLAY` | 0.06 | white veil over a surface on hover |
| `PRESS_OVERLAY` | 0.12 | stronger veil on press |
| `SCRIM` | black @ 60% | backdrop behind modals |

## Shared helpers

Two functions every atom uses so state is *identical* everywhere:

```rust
/// Blend a color to its disabled appearance (alpha × OPACITY_DISABLED).
/// Atoms gate it behind `if !enabled` so the veil is applied exactly once.
pub fn disabled_color(c: Color32) -> Color32

/// Eased hover progress in 0..=1 for a widget — animates `hovered` over
/// DURATION_FAST, shaped by EaseOut. Pass a stable id (e.g. response.id).
pub fn hover_t(ctx: &egui::Context, id: egui::Id, hovered: bool) -> f32
```

These are the reason hover/disabled states look the same across the whole library: the
math lives in one place, not copy-pasted into 23 atoms.
</content>
