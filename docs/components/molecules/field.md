# Field (family)

> **Layer:** molecule · **Path:** `src/molecules/field.rs` · **Exports:** `field::{Field, FieldGroup, FieldOrientation, FieldSeparator, FieldSet}`

The form-layout family. [`Field`](#field) wraps any control with a label and a hint/error line in one of three orientations. [`FieldGroup`](#fieldgroup) stacks fields with a standard gap; [`FieldSet`](#fieldset) groups them semantically under a legend; [`FieldSeparator`](#fieldseparator) divides groups with an optional inline label. [`FieldOrientation`](#fieldorientation) selects vertical / horizontal / responsive layout. Modeled on the shadcn Field primitives.

## Design

- **Purpose / when to use** — Build consistent forms: every labeled control through `Field`, grouped by `FieldGroup`/`FieldSet`, divided by `FieldSeparator`.
- **Anatomy (Field)** — A label row (`Text::label` + an optional `*` in `theme.destructive` when required) and a control closure, with a hint or error caption below. Layout is vertical or horizontal per orientation.
- **Tokens / layout consumed** — `core::SPACE_4` (horizontal label↔control gap; FieldGroup item gap; FieldSet pad), `SPACE_2`, `SPACE_1`; `layout::FIELD_HORIZONTAL_MIN` (= `480.0`) for responsive switching. See [tokens](../../tokens.md) and [layout](../../layout.md).
- **Accessibility** — required state is marked with a destructive-colored asterisk; error text uses `theme.error` and replaces the hint when present.

---

## Field

A labeled form field. `show` runs the `control` closure and lays out the label + hint/error around it.

### API

| Method | Effect |
|---|---|
| `Field::new(label: impl Into<String>) -> Self` | Construct with a label; orientation `Vertical`. |
| `.required() -> Self` | Append a destructive `*` after the label. |
| `.hint(hint: impl Into<String>) -> Self` | Muted caption below the control (suppressed if an error is set). |
| `.error(error: impl Into<String>) -> Self` | Error caption below the control (takes priority over hint). |
| `.orientation(orientation: FieldOrientation) -> Self` | Set layout mode. |
| `.horizontal() -> Self` | Sugar for `FieldOrientation::Horizontal`. |
| `.responsive() -> Self` | Sugar for `FieldOrientation::Responsive`. |
| `.show(self, ui: &mut Ui, control: impl FnOnce(&mut Ui) -> Response) -> Response` | Render; returns the **control's** `Response` (not a wrapper). |

> Note the closure signature: `control` must return a `Response` (e.g. the return of an `Input`/`Switch`/`Slider` `.show(ui)`).

### Usage

```rust
use ouroboros_ui::molecules::Field;
use ouroboros_ui::atoms::Input;

// minimal — vertical
Field::new("Email").show(ui, |ui| {
    Input::new(&mut email).placeholder("you@example.com").show(ui)
});
```

```rust
use ouroboros_ui::molecules::Field;
use ouroboros_ui::atoms::{Input, Switch};

// required + hint, then error state
Field::new("Email")
    .required()
    .hint("We never share it")
    .show(ui, |ui| Input::new(&mut email).show(ui));

Field::new("Username")
    .error("Already taken")
    .show(ui, |ui| Input::new(&mut user).error(true).show(ui));

// horizontal — label ↔ control (good for switches)
Field::new("Vsync")
    .horizontal()
    .show(ui, |ui| Switch::new(&mut vsync).show(ui));
```

---

## FieldOrientation

Field layout mode.

| Variant | Behavior |
|---|---|
| `Vertical` *(default)* | Label above, control below. |
| `Horizontal` | Label and control side by side (`SPACE_4` gap), hint/error under the control. |
| `Responsive` | Horizontal when `ui.available_width() >= layout::FIELD_HORIZONTAL_MIN` (480.0), else vertical. |

Responsive evaluates the available width at `show` time each frame, so a field re-stacks as its container resizes. See [layout](../../layout.md).

---

## FieldGroup

A zero-config stacker: runs a `content` closure inside a vertical layout with `item_spacing.y = SPACE_4`, so consecutive fields get uniform spacing.

### API

| Method | Effect |
|---|---|
| `FieldGroup::new() -> Self` | Construct. (`Default` also available.) |
| `.show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Response` | Render the stacked content. Returns the vertical layout `Response`. |

### Usage

```rust
use ouroboros_ui::molecules::{Field, FieldGroup};

FieldGroup::new().show(ui, |ui| {
    Field::new("Name").show(ui, |ui| Input::new(&mut name).show(ui));
    Field::new("Email").show(ui, |ui| Input::new(&mut email).show(ui));
});
```

---

## FieldSet

A semantic group with an optional legend, rendered inside a `Surface` with `SurfaceFill::None` (padding only, no fill).

### API

| Method | Effect |
|---|---|
| `FieldSet::new() -> Self` | Construct. (`Default` also available.) |
| `.legend(legend: impl Into<String>) -> Self` | Optional heading label above the content (`Text::label`). |
| `.show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Response` | Render the legend + content. Returns the surface `Response`. |

### Usage

```rust
use ouroboros_ui::molecules::{FieldSet, RadioGroup};

FieldSet::new().legend("Display").show(ui, |ui| {
    RadioGroup::new(&mut sel)
        .options(["Windowed", "Fullscreen"])
        .show(ui);
});
```

---

## FieldSeparator

A horizontal [`Divider`](../atoms/divider.md) between field groups, optionally with a centered inline caption below the rule.

> *(v1: rule + centered caption; true inline line–text–line is a later refinement.)*

### API

| Method | Effect |
|---|---|
| `FieldSeparator::new() -> Self` | Plain divider. (`Default` also available.) |
| `.label(label: impl Into<String>) -> Self` | Add a centered muted caption (e.g. `"OR"`). |
| `.show(self, ui: &mut Ui) -> Response` | Render; returns the divider `Response`. |

### Usage

```rust
use ouroboros_ui::molecules::FieldSeparator;

FieldSeparator::new().show(ui);              // bare rule
FieldSeparator::new().label("OR").show(ui);  // rule + centered caption
```

## Composition

The family composes [`Text`](../atoms/text.md), [`Surface`](../atoms/surface.md) (with `SurfaceFill::None`), and [`Divider`](../atoms/divider.md) only; controls and grouped content are caller closures. Nothing here paints — see the [guards](../../guards.md).

## Notes

- `Field::show`'s `control` closure must **return a `Response`** — pass through the inner atom's `.show(ui)`.
- Error wins over hint: when both are set, only the error caption renders.
- The label row is omitted (and no label→control gap added) when the label string is empty.
- `Responsive` reads `available_width` live, so wrap it in a sized region if you need deterministic behavior.
