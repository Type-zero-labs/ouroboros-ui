# Card

> **Layer:** molecule · **Path:** `src/molecules/card.rs` · **Exports:** `card::{Card, CardSize}`

An elevated [`Surface`](../atoms/surface.md) with an optional header (title + description + a top-right action slot), arbitrary content (a closure), and an optional footer separated by a divider. `size` scales the internal padding and gaps. Models the shadcn Card (Header / Content / Footer / CardAction).

## Design

- **Purpose / when to use** — Group related content into a raised panel: a settings block, a summary, a property editor section.
- **Anatomy** — `Surface::elevated()` (padded by size) → vertical stack of:
  1. **Header** (only if title/description/action set): horizontal row of a vertical [`Heading`](../atoms/heading.md) title + muted caption [`Text`](../atoms/text.md) description, with the **action** closure laid out right-to-left at the top-right.
  2. **Content** — your `content` closure, always run.
  3. **Footer** (if set): a [`Divider`](../atoms/divider.md) then your `footer` closure.
- **Sizes**

  | `CardSize` | pad | header/footer gap |
  |---|---|---|
  | `Default` | `SPACE_4` | `SPACE_3` |
  | `Sm` | `SPACE_3` | `SPACE_2` |

- **Tokens / layout consumed** — `core::SPACE_4 / SPACE_3 / SPACE_2 / SPACE_1`; elevation from [`Surface`](../atoms/surface.md). See [tokens](../../tokens.md).

## API

| Method | Effect |
|---|---|
| `Card::new() -> Self` | Empty card, `CardSize::Default`. (`Default` also available.) |
| `.title(title: impl Into<String>) -> Self` | Header title (renders a `Heading`). |
| `.description(description: impl Into<String>) -> Self` | Header sub-text (muted caption). |
| `.action(action: impl FnOnce(&mut Ui) + 'a) -> Self` | Top-right header slot — a button/menu/badge. |
| `.footer(footer: impl FnOnce(&mut Ui) + 'a) -> Self` | Footer slot below a divider. |
| `.size(size: CardSize) -> Self` | Set the spacing scale. |
| `.sm() -> Self` | Sugar for `CardSize::Sm`. |
| `.show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Response` | Render; `content` is the card body. Returns the surface `Response`. |

**`CardSize`** — `Default` (default), `Sm`.

## Usage

```rust
use ouroboros_ui::molecules::Card;
use ouroboros_ui::atoms::Text;

// minimal
Card::new().title("Compact").show(ui, |ui| {
    Text::new("Body content.").show(ui);
});
```

```rust
use ouroboros_ui::molecules::Card;
use ouroboros_ui::atoms::{Button, Text};
use ouroboros_ui::egui_phosphor::light;

// realistic — header action + footer buttons + closure body
Card::new()
    .title("Project settings")
    .description("Manage your project preferences")
    .action(|ui| {
        Button::new("")
            .icon_left(light::DOTS_THREE)
            .icon_only()
            .ghost()
            .sm()
            .id_source("card_menu")
            .show(ui);
    })
    .footer(|ui| {
        ui.horizontal(|ui| {
            Button::new("Save").id_source("card_save").show(ui);
            Button::new("Cancel").ghost().id_source("card_cancel").show(ui);
        });
    })
    .show(ui, |ui| {
        Text::new("Card body content goes here.").show(ui);
    });
```

## Composition

Composes [`Surface`](../atoms/surface.md) + [`Heading`](../atoms/heading.md) + [`Text`](../atoms/text.md) + [`Divider`](../atoms/divider.md). Body, action, and footer are caller-supplied closures. It never paints — see the [guards](../../guards.md).

## Notes

- `Card<'a>` carries a lifetime: the `action`/`footer` closures are boxed (`Box<dyn FnOnce(&mut Ui) + 'a>`) and may borrow from the surrounding scope.
- `content` is a plain `impl FnOnce(&mut Ui)` (not boxed) and always runs, even with no header/footer.
- The header is omitted entirely when none of title/description/action are set.
- Give interactive widgets inside the slots stable `id_source`s when multiple cards share a frame.
