# PropertyRow

> **Layer:** cell · **Path:** `src/cells/property_row.rs` · **Exports:** `property_row::PropertyRow`

An aligned inspector row in the Unity-style two-column layout: a fixed-width muted label on the left and an arbitrary control on the right. The control is supplied as a closure at `show` time, so any widget (numeric field, color field, switch, ...) can sit in the value column while labels stay vertically aligned across rows.

## Design

- **Purpose / when to use** — Inspector / property panels where many heterogeneous controls must share a single aligned label gutter.
- **Anatomy** — A horizontal layout: a label slot of fixed `label_width` × `core::CONTROL_MD` holding a muted [`Text`](../atoms/text.md), followed by the consumer-provided control.
- **Variants / states** — None of its own; visual state lives in the control closure.
- **Tokens / layout consumed** — `layout::PROPERTY_LABEL_WIDTH` (default label column = 120px), `core::CONTROL_MD` (32px label slot height, matching the standard control height). See [layout tokens](../../layout.md) and [tokens](../../tokens.md).
- **Accessibility** — Label and control are separate widgets; the alignment is purely visual.

## API

| Method | Signature | Effect |
|--------|-----------|--------|
| `new` | `new(label: impl Into<String>) -> Self` | Construct with the label; `label_width` defaults to `layout::PROPERTY_LABEL_WIDTH` (120px). |
| `label_width` | `label_width(self, width: f32) -> Self` | Override the label column width (e.g. for wider inspectors). |
| `show` | `show(self, ui: &mut Ui, control: impl FnOnce(&mut Ui) -> Response) -> Response` | Render the label column, then run `control` for the value column; returns the **control's** `Response`. |

## Usage

```rust
use ouroboros_ui::cells::PropertyRow;
use ouroboros_ui::atoms::NumericField;

let mut mass = 1.0_f32;
PropertyRow::new("Mass").show(ui, |ui| NumericField::new(&mut mass).speed(0.05).show(ui));
```

```rust
// realistic — a column of aligned inspector rows
let mut vals = [1.0_f32, 0.05, 0.6];
for (i, name) in ["Mass", "Drag", "Bounce"].iter().enumerate() {
    PropertyRow::new(*name).show(ui, |ui| {
        NumericField::new(&mut vals[i]).speed(0.05).show(ui)
    });
}
```

## Composition

Composes the [`Text`](../atoms/text.md) atom for the label; the value column is whatever the caller's closure adds. No painting of its own — it only allocates the label slot and delegates. Enforced by [`tests/no_painter_in_molecules.rs`](../../guards.md).

## Notes

- The `control` closure **must return a `Response`** (e.g. the inner widget's `show(ui)` result); `PropertyRow::show` forwards it.
- The label slot height is fixed at `core::CONTROL_MD` (32px), so controls taller than that will not vertically center against the label.
- Keep `label_width` consistent across rows in the same panel (use the default) so labels line up.
