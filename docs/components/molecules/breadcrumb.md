# Breadcrumb

> **Layer:** molecule · **Path:** `src/molecules/breadcrumb.rs` · **Exports:** `breadcrumb::Breadcrumb`

A horizontal path trail. Every item except the last renders as a link-style [`Button`](../atoms/button.md) separated by caret icons; the last item is plain strong text (the current location). `show` reports which crumb was clicked this frame. Analogous to the shadcn Breadcrumb.

## Design

- **Purpose / when to use** — Show hierarchical location (asset path, navigation trail) and let the user jump back to an ancestor.
- **Anatomy** — `ui.horizontal` row of: per non-last item a small link [`Button`](../atoms/button.md) followed by a muted `CARET_RIGHT` [`Icon`](../atoms/icon.md); the final item a `body_strong` [`Text`](../atoms/text.md).
- **Tokens / layout consumed** — none directly; relies on atom sizing (`Button::sm`, `Icon::sm`). See [tokens](../../tokens.md).

## API

| Method | Effect |
|---|---|
| `Breadcrumb::new() -> Self` | Empty trail. (`Default` also available.) |
| `.items<S: Into<String>>(items: impl IntoIterator<Item = S>) -> Self` | Set the ordered crumbs. |
| `.show(self, ui: &mut Ui) -> Option<usize>` | Render; returns `Some(i)` for the crumb clicked this frame, else `None`. |

## Usage

```rust
use ouroboros_ui::molecules::Breadcrumb;

// minimal
Breadcrumb::new().items(["Home", "Library"]).show(ui);
```

```rust
use ouroboros_ui::molecules::Breadcrumb;

// realistic — react to a click
if let Some(i) = Breadcrumb::new()
    .items(["Assets", "Models", "Characters", "hero.fbx"])
    .show(ui)
{
    navigate_to_depth(i);
}
```

## Composition

Composes [`Button`](../atoms/button.md) (`ButtonVariant::Link`), [`Icon`](../atoms/icon.md), and [`Text`](../atoms/text.md). It never paints — see the [guards](../../guards.md).

## Notes

- Returns `Option<usize>` rather than a `Response` — the index is the actionable signal.
- Per-crumb button ids are derived via `.id_source(("crumb", i))`, so multiple breadcrumbs in one frame are safe.
- The last crumb is non-interactive by design (it's the current node).
