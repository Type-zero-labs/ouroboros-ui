# ResponsiveRow

> **Layer:** cell · **Path:** `src/cells/responsive_row.rs` · **Exports:** `responsive_row::ResponsiveRow`

The responsive sibling of [`PropertyRow`](property_row.md). Wide, it keeps the Unity-style aligned label column with a right-anchored control; once the available width drops below `INSPECTOR_ROW_STACK_MIN` it stacks the label above a full-width control, so a squeezed side panel never clips the label↔control pair. The control is supplied as a closure at `show` time, like `PropertyRow`.

## Design

- **Purpose / when to use** — Inspector / property panels that live in **resizable** side panels, where a fixed two-column row would clip when the panel is dragged narrow. For panels that never get narrow, the simpler [`PropertyRow`](property_row.md) is fine.
- **Anatomy**
  - **Wide** (`available_width >= threshold`): a label slot of `label_width` × `core::CONTROL_MD` holding a muted [`Text`](../atoms/text.md), then the consumer control anchored right (`Layout::right_to_left`) — the gap flexes on resize.
  - **Narrow** (`available_width < threshold`): a vertical stack — muted label, `core::SPACE_1` gap, then the control filling the width.
- **Variants / states** — no visual variants; the single axis is the wide↔narrow switch driven by available width. Mirrors [`Field`](../molecules/field.md)'s responsive orientation, with a lower default threshold tuned for inspector panels.
- **Tokens / layout consumed** — `layout::PROPERTY_LABEL_WIDTH` (default label column), `layout::INSPECTOR_ROW_STACK_MIN` (default stack threshold), `core::CONTROL_MD`, `core::SPACE_1`.
- **Layering** — cell: composes the [`Text`](../atoms/text.md) atom + the consumer control; never paints (the `no_painter_in_molecules` guard scans cells).
- **Accessibility** — inherits from the embedded control.

## API

| Signature | Effect |
|-----------|--------|
| `ResponsiveRow::new(label: impl Into<String>) -> Self` | A row labelled `label`; defaults `PROPERTY_LABEL_WIDTH` / `INSPECTOR_ROW_STACK_MIN`. |
| `.label_width(width: f32) -> Self` | Override the aligned label-column width (wide layout). |
| `.threshold(px: f32) -> Self` | Override the available width below which the row stacks. |
| `.show(self, ui: &mut Ui, control: impl FnOnce(&mut Ui) -> Response) -> Response` | Lay out label + control; returns the control's `Response`. |

## Usage

```rust
use ouroboros_ui::cells::ResponsiveRow;
use ouroboros_ui::atoms::NumericField;

let mut mass = 1.0_f32;
ResponsiveRow::new("Mass").show(ui, |ui| {
    NumericField::new(&mut mass).speed(0.05).show(ui)
});
// Wide panel → "Mass    [ 1.0 ]" on one line; narrow panel → "Mass" above "[ 1.0 ]".
```

## Composition

Cell: same two-column construction as [`PropertyRow`](property_row.md) in the wide branch, plus a vertical-stack branch chosen by `ui.available_width()` against the threshold. Composes the [`Text`](../atoms/text.md) atom and the consumer-provided control.

## Notes

- Stacking decision reads `ui.available_width()`, which is set by the parent panel/splitter — the same exogenous-budget model the rest of the layout uses.
- `PropertyRow` is **not** replaced: pick `ResponsiveRow` when the row sits in a resizable inspector that can get narrow; keep `PropertyRow` for fixed-width contexts (e.g. data-table value columns).

See [tokens](../../tokens.md) · [theming](../../theming.md) · [guards](../../guards.md).
