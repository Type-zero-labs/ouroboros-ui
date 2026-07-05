# TabView

> **Layer:** organism · **Path:** `src/organisms/tab_view.rs` · **Exports:** `tab_view::TabView`

A tab bar plus the selected panel (shadcn Tabs). Bound to a `&mut usize` active index. `show` draws the [`Tabs`](../molecules/tabs.md) molecule, a [`Divider`](../atoms/divider.md), then calls your `panel(ui, index)` closure to render the active body. The selection is written back to `*selected` after the frame.

## Design

- **Purpose / when to use** — switch between sibling views sharing one region (Scene / Game / Console). Use when exactly one panel is visible at a time.
- **Anatomy** — `ui.vertical` → [`Tabs`](../molecules/tabs.md) bar (bound to a local copy of the index) → `SPACE_2` → `Divider::horizontal()` → `SPACE_3` → `panel(ui, idx)` body.
- **Variants / states**

  | State | How |
  |---|---|
  | active tab | `*selected` (mirrored into the `Tabs` molecule) |
  | panel switch | `panel(ui, idx)` renders the body for the current index |

- **Tokens / layout consumed** — `core::SPACE_2` (bar→divider) and `core::SPACE_3` (divider→body). See [tokens](../../tokens.md).
- **Accessibility** — tab keyboard/click behavior comes from the `Tabs` molecule.

## API

| Method | Effect |
|---|---|
| `TabView::new(selected: &'a mut usize) -> Self` | Bind to an active-tab index. |
| `.tabs<S: Into<String>>(tabs: impl IntoIterator<Item = S>) -> Self` | Set the tab labels. |
| `.show(ui, panel: impl FnOnce(&mut Ui, usize)) -> Response` | Draw the bar + divider, then `panel(ui, active_index)`. Returns the vertical `Response`; writes `*selected`. |

Note `panel` receives `(&mut Ui, usize)` — the second arg is the currently selected tab index.

## Usage

```rust
use ouroboros_ui::organisms::TabView;
use ouroboros_ui::atoms::Text;

let mut sel = 0usize;
TabView::new(&mut sel)
    .tabs(["Scene", "Game", "Console"])
    .show(ui, |ui, idx| {
        Text::new(match idx { 0 => "Viewport", 1 => "Preview", _ => "Logs" }).show(ui);
    });
```

```rust
// realistic — persist selection across frames (from storybook)
use ouroboros_ui::organisms::TabView;
use ouroboros_ui::atoms::Text;

let id = egui::Id::new("tabview_demo");
let mut sel = ui.data(|d| d.get_temp::<usize>(id).unwrap_or(0));
TabView::new(&mut sel)
    .tabs(["Scene", "Game", "Console"])
    .show(ui, |ui, idx| {
        let body = match idx { 0 => "3D scene viewport.", 1 => "Game preview.", _ => "Log output…" };
        Text::new(body).muted().show(ui);
    });
ui.data_mut(|d| d.insert_temp(id, sel));
```

## Composition

Composes the [`Tabs`](../molecules/tabs.md) molecule (the bar) and the [`Divider`](../atoms/divider.md) atom (separator), then defers the body to your `panel` closure. It never paints — see [guards](../../guards.md).

## Notes

- **State ownership** — the consumer owns the `&mut usize`. `show` mirrors it into a local `idx`, lets `Tabs` mutate that copy, and writes it back at the end — persist across frames yourself.
- `panel` is `FnOnce`; render the body for `idx` directly (a `match` on the index is the common shape).
