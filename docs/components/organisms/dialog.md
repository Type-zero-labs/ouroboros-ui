# Dialog

> **Layer:** organism · **Path:** `src/organisms/dialog.rs` · **Exports:** `dialog::Dialog`

A modal dialog with title / optional description + a free body (shadcn Dialog / Unity Overlay). Built on [`egui::Modal`](https://docs.rs/egui) — a scrim + centered frame that inherits the themed window visuals. Render it only while your "open" flag is true; `show` returns `true` when the modal should close (backdrop click or Esc).

## Design

- **Purpose / when to use** — confirmations, destructive-action prompts, focused forms that must block the rest of the UI until resolved.
- **Anatomy** — `egui::Modal` casing → [`Heading`](../atoms/heading.md) (`h2`, the title) → optional [`Text`](../atoms/text.md) (`muted`, the description) → `SPACE_4` → the body closure (your buttons/fields). Max width clamped to `layout::PANEL_MAX`.
- **Variants / states**

  | State | How |
  |---|---|
  | open | render `Dialog` this frame |
  | closed | don't render it (consumer owns the flag) |
  | with/without description | `.description(...)` optional |
  | dismissed | `show` returns `true` on backdrop click / Esc (`Modal::should_close()`) |

- **Tokens / layout consumed** — `core::SPACE_1` (title→description gap), `core::SPACE_4` (header→body gap), `layout::PANEL_MAX` (max width). See [tokens](../../tokens.md) / [layout](../../layout.md).
- **Layering** — uses **`egui::Modal`** (scrim overlay, centered). The frame is the themed modal/window visuals (not a manual `Surface`); content atoms supply the casing.
- **Accessibility** — `Modal` provides the focus scrim and Esc-to-dismiss; backdrop click also closes. Both surface through the `bool` return.

## API

| Method | Effect |
|---|---|
| `Dialog::new(title: impl Into<String>) -> Self` | New dialog. Default id is `Id::new(format!("dialog::{title}"))`. |
| `.id_source(id: impl Hash) -> Self` | Override the `Modal` id (use when titles collide). |
| `.description(description: impl Into<String>) -> Self` | Add a muted sub-line under the title. |
| `.show(ctx: &Context, body: impl FnOnce(&mut Ui)) -> bool` | Render the modal; returns `true` when it should close. |

Note `show` takes a `&Context` (e.g. `ui.ctx()`), not a `&mut Ui`.

## Usage

```rust
use ouroboros_ui::organisms::Dialog;

if open {
    if Dialog::new("Rename layer")
        .description("Enter a new name.")
        .show(ui.ctx(), |ui| { /* field + buttons */ })
    {
        open = false; // backdrop / Esc
    }
}
```

```rust
// realistic — destructive confirm; consumer owns `open` (from storybook)
use ouroboros_ui::organisms::Dialog;
use ouroboros_ui::atoms::{Button, ButtonVariant};

if open {
    let mut dismiss = false;
    let close = Dialog::new("Delete asset?")
        .description("This action cannot be undone.")
        .show(ui.ctx(), |ui| {
            ui.horizontal(|ui| {
                if Button::new("Delete")
                    .variant(ButtonVariant::Destructive)
                    .id_source("dlg_del")
                    .show(ui).clicked()
                {
                    dismiss = true; // perform delete
                }
                if Button::new("Cancel").ghost().id_source("dlg_cancel").show(ui).clicked() {
                    dismiss = true;
                }
            });
        });
    if close || dismiss { open = false; }
}
```

## Composition

Overlay organism: **`egui::Modal`** container (scrim + centering) + themed window visuals for the casing; content composed from [`Heading`](../atoms/heading.md) and [`Text`](../atoms/text.md) atoms, then your body. It never paints raw shapes — see [guards](../../guards.md).

## Notes

- **State ownership** — the consumer owns the open/closed flag. `Dialog` only reports the should-close signal; act on both that return and your own button-driven dismissals (the storybook pattern ORs `close || dismiss`).
- Default id derives from the title — give two same-titled dialogs distinct `id_source` values.
- `body` is `FnOnce`, executed only while the modal is shown.
- Width capped at `PANEL_MAX`; for wider content set width inside the body closure.
