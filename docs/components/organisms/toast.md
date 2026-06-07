# Toast

> **Layer:** organism · **Path:** `src/organisms/toast.rs` · **Exports:** `toast::Toast`

A transient notification anchored top-right (shadcn Sonner / Unity notifications). Composes an [`Alert`](../molecules/alert.md) molecule inside a foreground [`egui::Area`](https://docs.rs/egui). The consumer owns visibility and timing — render the `Toast` only while it should be visible.

## Design

- **Purpose / when to use** — short status confirmations ("Build finished in 2.3s"), errors, warnings that don't block interaction. Drive its lifetime yourself (a flag, a timer).
- **Anatomy** — `egui::Area` (foreground order, anchored `RIGHT_TOP` with offset `(-SPACE_4, SPACE_4)`) → `ui.set_max_width(INSPECTOR_WIDTH)` → an `Alert` carrying the message + variant.
- **Variants / states**

  | Variant | How |
  |---|---|
  | default | `Toast::new(msg)` |
  | success | `.success()` |
  | warning | `.warning()` |
  | error | `.error()` |
  | custom | `.variant(AlertVariant)` |

- **Tokens / layout consumed** — `core::SPACE_4` (anchor inset from the top-right corner), `layout::INSPECTOR_WIDTH` (max width). See [tokens](../../tokens.md) / [layout](../../layout.md).
- **Layering** — uses **`egui::Area`** at `Order::Foreground`, anchored `Align2::RIGHT_TOP`. (Not a `Modal`/`Popup` — it doesn't capture input or scrim.)
- **Accessibility** — non-blocking overlay; dismissal/timing is the consumer's responsibility.

## API

| Method | Effect |
|---|---|
| `Toast::new(message: impl Into<String>) -> Self` | New toast; default id `Id::new("toast")`, default variant. |
| `.id_source(id: impl Hash) -> Self` | Override the `Area` id (required for multiple simultaneous toasts). |
| `.variant(variant: AlertVariant) -> Self` | Set the alert variant. |
| `.success() -> Self` / `.warning() -> Self` / `.error() -> Self` | Variant shortcuts. |
| `.show(ctx: &Context)` | Place it top-right. Returns `()`. |

`show` takes a `&Context` (e.g. `ui.ctx()`), not a `&mut Ui`.

## Usage

```rust
use ouroboros_ui::organisms::Toast;

if show_toast {
    Toast::new("Saved").success().show(ui.ctx());
}
```

```rust
// realistic — consumer-owned visibility (from storybook)
use ouroboros_ui::organisms::Toast;

let id = egui::Id::new("toast_show");
let mut show = ui.data(|d| d.get_temp::<bool>(id).unwrap_or(false));
if Button::new(if show { "Hide toast" } else { "Show toast" }).id_source("toast_btn").show(ui).clicked() {
    show = !show;
}
if show {
    Toast::new("Build finished in 2.3s").success().show(ui.ctx());
}
ui.data_mut(|d| d.insert_temp(id, show));
```

## Composition

Overlay organism: an **`egui::Area`** (foreground) container holding an [`Alert`](../molecules/alert.md) molecule for the casing/content. It never paints — see [guards](../../guards.md).

## Notes

- **State ownership** — the consumer owns visibility *and* timing; there is no built-in auto-dismiss.
- The default id is the literal `"toast"` — give each concurrent toast a distinct `id_source`, or they overlap in the same `Area`.
- Anchored top-right with a `SPACE_4` inset; width capped at `INSPECTOR_WIDTH`.
