# Toggle

> **Layer:** atom · **Path:** `src/atoms/toggle.rs` · **Exports:** `toggle::Toggle`

A two-state toggle **button** (distinct from [`Switch`](switch.md)) bound to a `&mut bool`. It looks like a ghost button and fills with `accent` while on. Icon and/or label render as a single `valign`-centered galley, the same technique as [`Button`](button.md). Modeled on shadcn Toggle.

## Design

- **Purpose / when to use** — a pressable that latches on/off, like a formatting-bar "Bold" button. For an on/off setting that reads as a slider use [`Switch`](switch.md); for a non-latching action use [`Button`](button.md).
- **Anatomy** — a rounded rect (`RADIUS_MD`) that is transparent off, `hover_overlay` on hover, and `accent`-filled when on; with a centered single galley of optional icon + optional label. Icon-only (no label) → square (height = `CONTROL_SM`).
- **Variants / sizes / states** — fixed height `CONTROL_SM` (26), icon size `ICON_MD` (16). No size variants. **States**: on (`accent` fill), hover (`hover_overlay`), disabled (`disabled_color`, sense → hover). No focus ring.
- **Tokens consumed** — `theme.accent` (on fill), `theme.hover_overlay` (hover), `theme.foreground` (content), `core::CONTROL_SM` (height), `core::ICON_MD` (icon), `core::RADIUS_MD`, `core::SPACE_1` (icon↔label gap), `core::SPACE_2` (pad-x), `core::TRACKING_NORMAL`, `core::disabled_color`, `typography::body`/`icon_font`.
- **Accessibility** — emits `WidgetInfo::selected(WidgetType::Button, enabled, on, label)`.

## API

| Signature | Effect |
|-----------|--------|
| `Toggle::new(on: &mut bool) -> Self` | Bind to a boolean. |
| `.icon(glyph: &'static str) -> Self` | Leading Phosphor glyph. |
| `.label(label: impl Into<String>) -> Self` | Add a label. |
| `.enabled(enabled: bool) -> Self` / `.disabled()` | Enable/disable. |
| `.id_source(id: impl Hash) -> Self` | Stable id. |
| `.show(self, ui: &mut Ui) -> Response` | Toggle on click, `mark_changed`, return `Response`. |

## Usage

```rust
use ouroboros_ui::atoms::Toggle;
use ouroboros_ui::egui_phosphor::light;

let mut bold = false;
Toggle::new(&mut bold).icon(light::TEXT_B).show(ui);   // icon-only, square
```

```rust
use ouroboros_ui::atoms::Toggle;

let mut grid = true;
if Toggle::new(&mut grid).label("Grid").show(ui).changed() {
    // grid visibility flipped
}
```

## Composition

Atom: paints the fill directly and builds icon+label as one inline `LayoutJob` (same single-galley approach as [`Button`](button.md); does not compose [`Icon`](icon.md)/[`Text`](text.md)).

## Notes

- Binding is `&mut bool`; on click it flips `*on` and calls `mark_changed()` — check `.changed()`.
- With no `label`, it renders icon-only and square; the `id_source` field exists but the on/hover fill is not animated (it is an instantaneous overlay, no `hover_t`).
- No focus ring (unlike [`Button`](button.md)/[`Switch`](switch.md)).

See [tokens](../../tokens.md) · [theming](../../theming.md) · [typography](../../typography.md) · [guards](../../guards.md).
