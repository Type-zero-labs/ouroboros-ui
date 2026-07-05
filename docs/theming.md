# Theming — semantic tokens & modes

`src/tokens/semantic.rs` + `src/theme/mod.rs`. The semantic layer maps shadcn's
vocabulary onto [core primitives](./tokens.md); the theme layer resolves a palette for a
`Mode` and installs it into the egui context.

> **No raw colors live in this layer.** Every `Theme` field references a `core::*`
> primitive. That keeps the brand hue swappable in one place.

---

## The `Theme` struct

A resolved palette — every color token the design system exposes. Grouped:

### Surfaces (layered)

Dark mode layers the zinc ramp by elevation: `background` 950 → `card`/`popover` 900 →
`muted` 800.

| Token | Dark | Role |
|-------|------|------|
| `background` / `foreground` | `ZINC_950` / `ZINC_50` | deepest layer — panels, window fill |
| `card` / `card_foreground` | `ZINC_900` / `ZINC_50` | raised surface — cards, elevated panels |
| `popover` / `popover_foreground` | `ZINC_900` / `ZINC_50` | floating surface — popovers, menus, tooltips |
| `muted` / `muted_foreground` | `ZINC_800` / `ZINC_400` | inputs, secondary fills / labels, placeholders |
| `disabled_foreground` | `ZINC_600` | disabled text |

### Interactive

| Token | Dark | Role |
|-------|------|------|
| `primary` / `primary_foreground` | `TEAL_200` / `ZINC_950` | default action — turquoise fill, dark text |
| `secondary` / `secondary_foreground` | `ZINC_800` / `ZINC_50` | secondary action |
| `accent` / `accent_foreground` | `ZINC_800` / `ZINC_50` | hover/active surface (shadcn `accent`, not brand) |
| `destructive` / `destructive_foreground` | `RED_500` / `ZINC_50` | destructive action |

### Borders & focus

| Token | Dark | Role |
|-------|------|------|
| `border` | `ZINC_800` | default border / divider |
| `border_strong` | `ZINC_700` | emphasized border |
| `input` | `ZINC_800` | input border |
| `ring` | `TEAL_300` | focus ring |
| `hover_overlay` | white @ 6% | hover veil (dark veil in light mode) |
| `press_overlay` | white @ 12% | pressed veil |
| `scrim` | black @ 60% (`core::SCRIM`) | backdrop veil behind modals and loading overlays — black in **both** modes (a scrim dims, it doesn't invert). Need a lighter/heavier scrim? derive it (`theme.scrim.gamma_multiply(..)`), don't mint a new literal. |

### Status (solid + soft bg)

Each status has a solid hue and a soft `*_bg` (the hue tinted to ~15% alpha).

| Token | Solid | Soft `*_bg` |
|-------|-------|-------------|
| `success` | `GREEN_500` | green @ 15% |
| `warning` | `AMBER_500` | amber @ 15% |
| `error` | `RED_500` | red @ 15% |
| `info` | `BLUE_400` | blue @ 15% |
| `neutral` | `ZINC_500` | zinc @ 15% |

---

## The four palettes

| Constructor | Look |
|-------------|------|
| `Theme::dark()` | **default.** Zinc surfaces (950/900/800), teal-200 primary, teal-300 ring. |
| `Theme::light()` | Off-white surfaces (zinc-50/100), teal-400 primary, dark text, dark hover veils. |
| `Theme::zinc_dark()` | Dark with the **neutral zinc** primary (zinc-50 fill, no brand hue) — the pre-Ouroboros look. |
| `Theme::zinc_light()` | Light with neutral zinc primary (zinc-900 fill). |

`Theme::default()` is `dark()`.

> **Note on light mode:** the crate README and a `Mode` doc-comment still describe Light
> as a "stub that resolves to Dark." That is **stale** — `Theme::light()` is fully
> populated and `Theme::resolve(Mode::Light)` returns it. Light mode works today.

---

## `Mode` & resolution

```rust
pub enum Mode { Dark /* default */, Light }
```

The system always resolves through `Theme::resolve(mode)` so a palette can change without
touching consumers:

```rust
impl Theme {
    pub fn resolve(mode: Mode) -> Self;   // Dark => dark(), Light => light()
}
```

> `resolve` covers the two `Mode` variants (dark/light). The zinc-neutral palettes are
> opt-in via the constructors directly — install them with `apply` after resolving, or
> store them yourself.

---

## Installing the theme

Call **once at startup**, then optionally re-`apply` to switch mode at runtime.

```rust
use ouroboros_ui::{Mode, Theme};

// In eframe::App::new / setup:
Theme::install(ctx, Mode::Dark);   // registers fonts + applies the palette
```

`install` does two things:

1. `typography::register(&mut fonts)` — loads the bundled Iosevka faces + Phosphor icons.
2. `Theme::apply(ctx, mode)` — applies visuals, stores the resolved theme, sets text styles.

### Switching mode at runtime

`apply` reapplies the palette **without re-registering fonts** — use it for a Dark/Light
toggle:

```rust
Theme::apply(ctx, Mode::Light);
```

`apply` also flips egui's own `ThemePreference` so built-in chrome (clear color, native
scrollbars) follows the mode, and wires panel/window/extreme/faint fills + the five
egui `TextStyle`s (Heading→h2, Body/Button→body, Monospace→code, Small→caption).

### Reading the theme inside a widget

Components fetch the installed theme from the context (falling back to `Theme::default()`):

```rust
let theme = Theme::get(ui);              // from a &Ui
let theme = Theme::get_from_ctx(ctx);    // from a &Context
```

The theme is stored in egui's temp data under `Id::NULL`. Every atom calls `Theme::get`
at the top of its `show` to know what to paint with — this is the mechanism that makes
the whole library theme-reactive for free.

---

## Adding a new semantic token

1. Add the field to `Theme` in `semantic.rs`.
2. Populate it in **all four** constructors (`dark`, `light`, `zinc_dark`, `zinc_light`),
   referencing only `core::*` — never a raw color (the guard will reject raw colors in
   atoms, and convention keeps semantic clean).
3. Consume it from an atom via `Theme::get(ui).your_token`.

If the value is component-specific (one button variant, one input state), prefer a
`tokens::component` struct over a global semantic field — see
[architecture.md](./architecture.md#3-component--per-component-overrides).
</content>
