# EmptyState

> **Layer:** molecule · **Path:** `src/molecules/empty_state.rs` · **Exports:** `empty_state::{EmptyState, EmptyStateOutput}`

A centered "nothing here yet" placeholder with call-to-action, shown where a list or
content area has no items yet (scene list, panel list, asset galleries). Analogous to
the shadcn Empty pattern; layout follows the studio launcher's empty state (centered
column, stacked CTAs).

Born in the `scene-hub-shell` spec: *use* fails (only `Table::empty_text`, plain
string) and *extend* fails (no neighboring component) — so it was created here instead of
being hand-rolled per screen.

## Design

- **Purpose / when to use** — Guide the user's next step when a content area is empty.
  Not for error states (use [`Alert`](./alert.md)) nor loading (use `Skeleton`).
- **Anatomy** — vertical column centered horizontally (`Layout::top_down(Align::Center)`),
  pushed down by ⅓ of the available height (optical centering): muted `xl`
  [`Icon`](../atoms/icon.md) → [`Heading`](../atoms/heading.md) (`h2`) → optional muted
  wrapped [`Text`](../atoms/text.md) description → primary [`Button`](../atoms/button.md)
  (default variant) → secondary `Button` (secondary variant), stacked.
- **Variants / states** — none; presence of icon/description/CTAs is driven by the
  builder calls.
- **Tokens / layout consumed** — `core::SPACE_3` (icon→title, CTA gap), `SPACE_2`
  (title→description), `SPACE_5` (description→primary CTA); colors via [`Theme`]
  (muted foreground). See [tokens](../../tokens.md).

## API

| Method | Effect |
|---|---|
| `EmptyState::new(title: impl Into<String>) -> Self` | Construct with the title. |
| `.icon(glyph: &'static str) -> Self` | Muted `xl` glyph above the title. |
| `.description(text: impl Into<String>) -> Self` | Muted, wrapped supporting copy. |
| `.primary_action(label: impl Into<String>) -> Self` | Primary CTA (default variant). |
| `.primary_icon(glyph: &'static str) -> Self` | Leading glyph for the primary CTA. |
| `.secondary_action(label: impl Into<String>) -> Self` | Secondary CTA below the primary. |
| `.secondary_icon(glyph: &'static str) -> Self` | Leading glyph for the secondary CTA. |
| `.show(self, ui: &mut Ui) -> EmptyStateOutput` | Render; reports CTA clicks. |

**`EmptyStateOutput`** — `response: Response`, `primary_clicked: bool`,
`secondary_clicked: bool`.

## Usage

```rust
use ouroboros_ui::egui_phosphor::light;
use ouroboros_ui::molecules::EmptyState;

let out = EmptyState::new("No scenes yet")
    .icon(light::STACK)
    .description("Scenes are the entry points of your game.")
    .primary_action("Create Scene")
    .primary_icon(light::PLUS)
    .show(ui);
if out.primary_clicked {
    // open the create-scene modal
}
```

## Storybook

`cargo run --example storybook` → page **Empty state**.
