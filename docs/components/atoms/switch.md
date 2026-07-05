# Switch

> **Layer:** atom · **Path:** `src/atoms/switch.rs` · **Exports:** `switch::Switch`

A boolean toggle bound to a `&mut bool`, with an animated sliding thumb. A pill track (`primary` when on, `border_strong` when off) holds a `background`-filled thumb that slides via `animate_bool_with_time`. All dimensions derive from tokens; includes a focus ring and disabled dim.

## Design

- **Purpose / when to use** — an on/off setting that applies immediately (the "iOS toggle"). For a checkbox-style boolean (especially in forms/lists with labels) use [`Checkbox`](checkbox.md); for a pressable two-state *button* use [`Toggle`](toggle.md).
- **Anatomy** — pill track → hover veil → circular thumb sliding left↔right with the animated `t` → focus ring. Track width = `track_h + SPACE_4`.
- **Variants / sizes / states**

  | Size | Track height | Thumb diameter |
  |------|--------------|----------------|
  | `Sm` | `ICON_MD` 16 | `ICON_SM` 14 |
  | `Md` (default) | `ICON_LG` 20 | `ICON_MD` 16 |
  | `Lg` | `ICON_XL` 24 | `ICON_LG` 20 |

  **States**: on (`primary` track) / off (`border_strong` track — chosen over `muted` so the thumb stays legible in dark mode); hover (`hover_t` veil); focus (ring); disabled (`disabled_color`, sense → hover).

- **Tokens consumed** — `theme.primary` (on track), `theme.border_strong` (off track), `theme.background` (thumb), `theme.hover_overlay`, `theme.ring`, `core::ICON_*` (dims), `core::SPACE_4` (track extra width), `core::DURATION_FAST` (thumb slide), `core::hover_t`, `core::disabled_color`, `core::Size`.
- **Accessibility** — emits `WidgetInfo::selected(WidgetType::Checkbox, enabled, on, "")` (no label). Focus ring via `focus::focus_ring_rect`.

## API

| Signature | Effect |
|-----------|--------|
| `Switch::new(on: &mut bool) -> Self` | Bind to a boolean. |
| `.enabled(enabled: bool) -> Self` / `.disabled()` | Enable/disable. |
| `.size(s: Size) -> Self` / `.sm()` / `.lg()` | Size (`core::Size`). |
| `.id_source(id: impl Hash) -> Self` | Stable id for animation/interaction (else `response.id`). |
| `.show(self, ui: &mut Ui) -> Response` | Toggle on click, `mark_changed`, return `Response`. |

## Usage

```rust
use ouroboros_ui::atoms::Switch;

let mut dark = true;
if Switch::new(&mut dark).show(ui).changed() {
    // apply theme
}
```

```rust
use ouroboros_ui::atoms::Switch;

let mut enabled = false;
Switch::new(&mut enabled).lg().id_source("notifications").show(ui);
```

## Composition

Atom: paints track + thumb directly. Composes no other atoms. The Switch carries **no label** — pair it with a [`Text`](text.md) atom in a molecule/row for a labeled setting.

## Notes

- Binding is `&mut bool`. On click it flips `*on` and calls `mark_changed()`; check `.changed()`.
- The thumb position animates over `DURATION_FAST` keyed by `id_source` (or `response.id`) — set `id_source` for switches in loops to avoid animation cross-talk.
- Off-state track is `border_strong`, intentionally not `muted`, for dark-mode legibility.

See [tokens](../../tokens.md) · [theming](../../theming.md) · [guards](../../guards.md).
