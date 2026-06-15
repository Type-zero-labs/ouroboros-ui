# Panel

> **Layer:** organism · **Path:** `src/organisms/panel.rs` · **Exports:** `panel::{Panel, PanelEdge}`

A canonical **docked panel**: a background, an optional flush hairline on its docking edge, an optional header (title + action slot) and footer action bar, and a scrollable, token-padded body. Mounts inside a rect — typically a [`Splitter`](splitter.md) band via a child `Ui`. Unlike the elevated, rounded [`Card`](../molecules/card.md), a Panel is **flush** (no radius, no shadow) with a single edge border — the studio inspector / properties chrome, so panels stop hand-rolling `egui::Frame` margins and manually painted borders/headers.

## Design

- **Purpose / when to use** — Any docked side panel / inspector / properties pane. For a free-floating elevated container use [`Card`](../molecules/card.md); for the resizable bands themselves use [`Splitter`](splitter.md) (a Panel goes *inside* a band).
- **Anatomy** — background [`Surface`](../atoms/surface.md) filling the mounted rect → optional flush edge [`Divider`](../atoms/divider.md) carved off the docking side → optional header ([`Heading`](../atoms/heading.md) + right-aligned action, then a full-width divider) → padded, scrollable body → optional footer (full-width divider + action bar pinned to the bottom).
- **Variants / states**

  | Axis | Options |
  |------|---------|
  | `edge` (`PanelEdge`) | `None` (default) · `Left` · `Right` · `Top` · `Bottom` |
  | `fill` (`SurfaceFill`) | `Background` (default) · `Card` · `Muted` · `None` (module paints its own bg) |
  | `scroll` | on (default) · `.no_scroll()` |
  | header / footer | absent unless `.title()`/`.action()` / `.footer()` set |

- **Tokens / layout consumed** — `layout::PANEL_PAD` (body/header/footer inset), `layout::PANEL_GAP` (row gap), `core::BORDER_THIN` (edge weight), the chosen `SurfaceFill` token.
- **Layering** — organism: composes [`Surface`](../atoms/surface.md), [`Divider`](../atoms/divider.md), [`Heading`](../atoms/heading.md); never paints directly (the `no_painter_in_molecules` guard scans organisms too).
- **Accessibility** — scrolling/keyboard come from egui's `ScrollArea`; the body width is pinned so fill controls don't ratchet (egui #1297).

## API

| Signature | Effect |
|-----------|--------|
| `Panel::new(id: impl Hash) -> Self` | New panel; `id` keys the body `ScrollArea`. |
| `.title(title: impl Into<String>) -> Self` | Header title (a `Heading`) above a full-width divider. |
| `.action(f: impl FnOnce(&mut Ui) + 'a) -> Self` | Top-right header slot (button / menu / badge). |
| `.footer(f: impl FnOnce(&mut Ui) + 'a) -> Self` | Bottom action bar above a full-width divider. |
| `.edge(e: PanelEdge) -> Self` / `.left_edge()` / `.right_edge()` | Flush hairline on the docking edge. |
| `.fill(f: SurfaceFill) -> Self` | Background fill (default `Background`). |
| `.no_scroll() -> Self` | Don't wrap the body in a `ScrollArea`. |
| `.show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Response` | Paint the chrome; run `content` in the padded body. |

## Usage

```rust
use ouroboros_ui::organisms::Panel;
use ouroboros_ui::cells::ResponsiveRow;
use ouroboros_ui::atoms::NumericField;

// Right-docked inspector mounted in a Splitter band rect (a child Ui).
Panel::new("world_inspector")
    .left_edge()
    .title("Inspector")
    .show(ui, |ui| {
        ResponsiveRow::new("Mass").show(ui, |ui| {
            NumericField::new(&mut mass).speed(0.05).show(ui)
        });
    });
```

## Composition

Carves the edge hairline off `ui.max_rect()` with `Rect::split_*_at_*`, draws a [`Divider`](../atoms/divider.md) in the strip, and stacks header / scrollable body / footer in the remainder via child `Ui`s. The body is a port of the studio's `panel_body` helper: `ScrollArea::vertical().auto_shrink([true, false])` + a `Surface` (no fill) padded by `PANEL_PAD`, with the content width pinned via `set_min_width` and a `PANEL_GAP` row spacing. A footer is laid bottom-up so it pins to the panel's bottom edge.

## Notes

- The body keeps horizontal auto-shrink **on** (egui #1297): a fill control inside a scroll area with horizontal auto-shrink off ratchets the content width on resize. Don't change this.
- `fill(SurfaceFill::None)` leaves the background to the host module (the legacy studio pattern) while still giving the canonical edge/header/body chrome.
- Replaces hand-rolled studio panel chrome: per-section `Frame::inner_margin`, manual `line_segment`/`rect_filled` edges, and hand-painted section headers.

See [tokens](../../tokens.md) · [theming](../../theming.md) · [guards](../../guards.md).
