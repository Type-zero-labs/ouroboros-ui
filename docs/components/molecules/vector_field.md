# VectorField

> **Layer:** molecule · **Path:** `src/molecules/vector_field.rs` · **Exports:** `vector_field::VectorField`

N numeric components edited side-by-side in a row — a Vec2/Vec3/Vec4 editor bound to a `&mut [f32]` slice. Each component is an axis label (X/Y/Z/W) plus a draggable [`NumericField`](../atoms/numeric_field.md). Inspired by Unity's Vector Field.

## Design

- **Purpose / when to use** — Edit transform-style vectors (position, rotation, scale) or any fixed set of float components inline.
- **Anatomy** — `ui.horizontal` row; per component: a muted caption [`Text`](../atoms/text.md) axis label (`"X"`/`"Y"`/`"Z"`/`"W"`, or `"·"` beyond 4) + `SPACE_1` + a width-allocated [`NumericField`](../atoms/numeric_field.md) (`CONTROL_MD` tall) + `SPACE_2`.
- **Tokens / layout consumed** — `core::SPACE_6` (per-component overhead reserve), `SPACE_12` (minimum field width), `SPACE_1` / `SPACE_2` (gaps), `CONTROL_MD` (field height). The available row width is split evenly across components. See [tokens](../../tokens.md).

## API

| Method | Effect |
|---|---|
| `VectorField::new(values: &'a mut [f32]) -> Self` | Bind the component slice; default drag `speed` `0.1`. |
| `.speed(speed: f32) -> Self` | Drag sensitivity passed to each `NumericField`. |
| `.show(self, ui: &mut Ui)` | Render. **Returns `()`** — edits land directly in the bound slice. |

## Usage

```rust
use ouroboros_ui::molecules::VectorField;

// minimal — a Vec3
let mut v = [1.0f32, 0.0, -1.0];
VectorField::new(&mut v).show(ui);
```

```rust
use ouroboros_ui::molecules::VectorField;

// finer drag speed
VectorField::new(&mut v).speed(0.05).show(ui);
```

## Composition

Composes [`Text`](../atoms/text.md) (axis labels) + [`NumericField`](../atoms/numeric_field.md) atoms. It never paints — see the [guards](../../guards.md).

## Notes

- Binds a `&mut [f32]` of arbitrary length; component count follows the slice (axis labels only cover X/Y/Z/W, others show `·`).
- `show` returns `()`, not a `Response` — mutation flows through the slice; observe values directly.
- Row width is divided evenly by component count with a per-field floor of `SPACE_12`, so it stays usable in narrow inspectors.
