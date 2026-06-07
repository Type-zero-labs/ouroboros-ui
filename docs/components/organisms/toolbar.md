# Toolbar

> **Layer:** organism · **Path:** `src/organisms/toolbar.rs` · **Exports:** `toolbar::Toolbar`

A horizontal action bar (Unity / O3DE Toolbar). `show` lays your `content` closure out horizontally inside a muted, borderless [`Surface`](../atoms/surface.md). A thin chrome wrapper — you fill it with [`ToolbarButton`](../cells/toolbar_button.md)s, [`Button`](../atoms/button.md)s, [`Divider`](../atoms/divider.md)s, etc.

## Design

- **Purpose / when to use** — the action strip of a view (tool toggles, play/pause, separators), typically the fixed header band (`PanelSpec::fixed`) of a screen's root [`Splitter`](splitter.md).
- **Anatomy** — a `Surface` (`muted`, `border_none`, `pad SPACE_1`, `RADIUS_MD`) → `ui.horizontal(content)`.
- **Variants / states** — none of its own; appearance comes from the muted surface and the widgets you place inside.
- **Tokens / layout consumed** — `core::SPACE_1` (surface padding), `core::RADIUS_MD` (corner radius); muted fill from the theme. See [tokens](../../tokens.md) / [theming](../../theming.md).
- **Accessibility** — n/a (container; behavior comes from child widgets).

## API

| Method | Effect |
|---|---|
| `Toolbar::new() -> Self` | Construct. (Unit struct; no fields.) |
| `Toolbar::default()` | Same as `new()`. |
| `.show(ui, content: impl FnOnce(&mut Ui)) -> Response` | Render the muted surface and lay `content` out horizontally. Returns the `Surface` `Response`. |

## Usage

```rust
use ouroboros_ui::organisms::Toolbar;
use ouroboros_ui::atoms::Button;
use ouroboros_ui::egui_phosphor::light;

Toolbar::new().show(ui, |ui| {
    Button::new("Play").icon_left(light::PLAY).sm().id_source("play").show(ui);
});
```

```rust
// realistic — tool toggles + divider + action (from storybook)
use ouroboros_ui::organisms::Toolbar;
use ouroboros_ui::cells::ToolbarButton;
use ouroboros_ui::atoms::{Button, Divider};
use ouroboros_ui::egui_phosphor::light;

Toolbar::new().show(ui, |ui| {
    ToolbarButton::new(&mut s[0], light::CURSOR).tooltip("Select").id_source("tba").show(ui);
    ToolbarButton::new(&mut s[1], light::ARROWS_OUT).tooltip("Move").id_source("tbb").show(ui);
    ToolbarButton::new(&mut s[2], light::ARROWS_CLOCKWISE).tooltip("Rotate").id_source("tbc").show(ui);
    Divider::vertical().show(ui);
    Button::new("Play").icon_left(light::PLAY).sm().id_source("tb_play").show(ui);
});
```

## Composition

Composes a single [`Surface`](../atoms/surface.md) atom (muted casing) plus whatever you place in the horizontal `content` closure — typically [`ToolbarButton`](../cells/toolbar_button.md) cells, [`Button`](../atoms/button.md) and [`Divider`](../atoms/divider.md) atoms. It never paints — see [guards](../../guards.md).

## Notes

- The bar holds no state; toggle/selection state lives in the widgets you place inside (e.g. each `ToolbarButton`'s `&mut bool`).
- `content` is `FnOnce`, run inside an `ui.horizontal` scope.
- Give each child widget a distinct `id_source` when several share the bar.
