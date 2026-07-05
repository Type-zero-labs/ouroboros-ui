# Collapsible

> **Layer:** molecule · **Path:** `src/molecules/collapsible.rs` · **Exports:** `collapsible::Collapsible`

A caret-headed section that hides or reveals its content. Open state persists in egui temp memory keyed by id (or by the title), so it survives across frames. The `content` closure only runs when open. Analogous to the shadcn Collapsible / Unity Foldout.

## Design

- **Purpose / when to use** — Inspector sections, settings groups, anything foldable to manage vertical density.
- **Anatomy** — A clickable header row: a muted caret [`Icon`](../atoms/icon.md) (`CARET_DOWN` when open, `CARET_RIGHT` when closed) + `SPACE_1` + a `body_strong` [`Text`](../atoms/text.md) title. When open, `SPACE_2` then the `content` closure.
- **States** — open / closed, toggled by clicking the header. Persisted via `ui.data` temp storage under the resolved id.
- **Tokens / layout consumed** — `core::SPACE_1` (caret→title), `SPACE_2` (header→content). See [tokens](../../tokens.md).

## API

| Method | Effect |
|---|---|
| `Collapsible::new(title: impl Into<String>) -> Self` | Construct; defaults to closed. |
| `.default_open(open: bool) -> Self` | Initial open state when no persisted value exists. |
| `.id_source(id: impl std::hash::Hash) -> Self` | Explicit id for persistence + interaction (defaults to `format!("collapsible::{title}")`). |
| `.show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Response` | Render; `content` runs only when open. Returns the header interaction `Response`. |

## Usage

```rust
use ouroboros_ui::molecules::Collapsible;
use ouroboros_ui::atoms::Text;

// minimal
Collapsible::new("Rendering").show(ui, |ui| {
    Text::new("Material, shadows…").muted().show(ui);
});
```

```rust
use ouroboros_ui::molecules::Collapsible;
use ouroboros_ui::atoms::Text;

// realistic — open by default, explicit id
Collapsible::new("Transform")
    .default_open(true)
    .id_source("inspector::transform")
    .show(ui, |ui| {
        Text::new("Position / Rotation / Scale").muted().show(ui);
    });
```

## Composition

Composes [`Icon`](../atoms/icon.md) + [`Text`](../atoms/text.md), with `ui.interact` over the header row's rect for the click target. It never paints — see the [guards](../../guards.md).

## Notes

- Open state lives in `ui.data` temp memory under the resolved id; the interaction id is `id.with("header")`.
- The returned `Response` is the header (use `.clicked()` to react to toggles beyond the visual).
- If two collapsibles share a title, pass distinct `id_source`s to avoid shared open state.
