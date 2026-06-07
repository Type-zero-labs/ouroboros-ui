# Button

> **Layer:** atom · **Path:** `src/atoms/button.rs` · **Exports:** `button::{Button, ButtonVariant}`

A labeled, optionally-iconed click control with the shadcn variant set. Icon(s) and label render as a **single galley** with each section `valign`-centered, so mixed icon/text fonts share one optical baseline. Fill/foreground/border come from [`ButtonTokens`]; hover/press/disabled/focus all come from motion/opacity/border tokens, so light+dark and every state stay token-driven.

## Design

- **Purpose / when to use** — the primary click affordance. Use `icon_only()` for toolbar buttons, `loading(true)` for in-flight actions. For a two-state pressable use [`Toggle`](toggle.md); for a boolean setting use [`Switch`](switch.md).
- **Anatomy** — filled rect (variant fill) → animated hover overlay → press overlay → optional border stroke → focus ring (on focus) → centered content galley (icon-left, label, icon-right) **or** an indeterminate spinner arc when loading.
- **Variants / sizes / states**

  **Variants** (`ButtonVariant`): `Default`, `Secondary`, `Destructive`, `Outline`, `Ghost`, `Link`. Each maps to a `ButtonTokens::*` constructor; `Link` underlines its label.

  | Size | Height | Icon size | Pad-x | Text style |
  |------|--------|-----------|-------|------------|
  | `Sm` | `CONTROL_SM` 26 | `ICON_SM` 14 | `SPACE_3` | `Size::text_style()` |
  | `Md` (default) | `CONTROL_MD` 32 | `ICON_MD` 16 | `SPACE_4` | … |
  | `Lg` | `CONTROL_LG` 38 | `ICON_LG` 20 | `SPACE_4` | … |

  **States**: hover (`theme.hover_overlay` ramped by `core::hover_t`), pressed (`theme.press_overlay`), focused (focus ring), disabled (`enabled(false)` → colors via `core::disabled_color`, sense drops to hover), loading (spinner arc, clicks ignored, width preserved).

- **Tokens consumed** — `ButtonTokens` (fill/foreground/border/radius/underline), `theme.hover_overlay`, `theme.press_overlay`, `theme.ring`, `core::SPACE_2` (icon gap), `core::BORDER_THIN`/`BORDER_FOCUS`, `core::hover_t`, `core::disabled_color`, `core::Size`, `typography::icon_font`.
- **Accessibility** — emits `WidgetInfo::labeled(WidgetType::Button, enabled, label)`. Focus ring via `focus::focus_ring_rect`. Hit target = full allocated rect (square when `icon_only`).

## API

| Signature | Effect |
|-----------|--------|
| `Button::new(label: impl Into<String>) -> Self` | Construct with a label. |
| `.variant(v: ButtonVariant) -> Self` | Set variant. |
| `.secondary()` / `.destructive()` / `.outline()` / `.ghost()` / `.link()` | Variant shorthands. |
| `.size(s: Size) -> Self` / `.sm()` / `.lg()` | Size (`core::Size`). |
| `.icon_only(self) -> Self` | Square button, label dropped. |
| `.loading(loading: bool) -> Self` | Replace content with a spinner; ignore clicks; preserve width. |
| `.icon_left(glyph: &'static str) -> Self` | Leading Phosphor glyph. |
| `.icon_right(glyph: &'static str) -> Self` | Trailing Phosphor glyph. |
| `.enabled(enabled: bool) -> Self` / `.disabled()` | Enable/disable. |
| `.id_source(id: impl Hash) -> Self` | Stable id for the hover animation (else `response.id`). |
| `.show(self, ui: &mut Ui) -> Response` | Paint and return the `Response` (`clicked`, `hovered`, …). |

**`ButtonVariant`** (enum): `Default` (default), `Secondary`, `Destructive`, `Outline`, `Ghost`, `Link`.

## Usage

```rust
use ouroboros_ui::atoms::Button;

if Button::new("Save").show(ui).clicked() {
    // …
}
```

```rust
use ouroboros_ui::atoms::Button;
use ouroboros_ui::egui_phosphor::light;

let saving = true;
let resp = Button::new("Save")
    .destructive()
    .icon_left(light::FLOPPY_DISK)
    .loading(saving)
    .show(ui);
if resp.clicked() && !saving { /* … */ }

// icon-only toolbar button
Button::new("").icon_left(light::GEAR).icon_only().ghost().sm().show(ui);
```

## Composition

Atom: paints fill/overlays/border/ring directly. Builds icon+label as one inline `LayoutJob` (does NOT compose the [`Icon`](icon.md)/[`Text`](text.md) atoms — single-galley alignment is intentional). The loading arc is the same form as the [`Spinner`](spinner.md) atom, inlined.

## Notes

- `loading(true)` overrides clicks regardless of `enabled`; width is computed from the content galley so the button does not resize when toggled into loading.
- `icon_only()` drops the label from layout but the label string is still reported in `widget_info` — pass a meaningful label for a11y even with `icon_only`.
- The hover animation keys off `id_source` if set; give icon-only buttons in a loop a stable `id_source` to avoid animation cross-talk.

See [tokens](../../tokens.md) · [theming](../../theming.md) · [typography](../../typography.md) · [guards](../../guards.md).
