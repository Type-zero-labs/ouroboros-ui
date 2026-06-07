# Input

> **Layer:** atom · **Path:** `src/atoms/input.rs` · **Exports:** `input::Input`

A single-line text field over a `&mut String`. A token-painted box (fill `muted`, border `input`/`destructive`/`ring`, animated hover veil) wraps a frameless `egui::TextEdit::singleline` — egui owns the editing, the casing is all token. States: default / focus / disabled / error. (Size variants live here; leading-icon / labels belong to a Field molecule.)

## Design

- **Purpose / when to use** — free-form single-line text entry. For multi-line use [`Textarea`](textarea.md); for numbers use [`NumericField`](numeric_field.md).
- **Anatomy** — filled rect (`muted`) → hover veil → inner frameless `TextEdit` (left-aligned, `body` font, `foreground` text, `muted_foreground` placeholder) → border stroke whose color/weight encodes state.
- **Variants / sizes / states**

  | Size | Height | Pad-x |
  |------|--------|-------|
  | `Sm` | `CONTROL_SM` 26 | `SPACE_3` |
  | `Md` (default) | `CONTROL_MD` 32 | `SPACE_4` |
  | `Lg` | `CONTROL_LG` 38 | `SPACE_4` |

  **Border state** (precedence): error → `destructive` @ `BORDER_THIN`; else focused → `ring` @ `BORDER_FOCUS`; else `input` @ `BORDER_THIN`. Disabled dims fill + border + text via `disabled_color` and uses `add_enabled(false, …)`.

- **Tokens consumed** — `theme.muted` (fill), `theme.input`/`theme.destructive`/`theme.ring` (border), `theme.foreground` (text), `theme.muted_foreground` (placeholder), `theme.hover_overlay`, `core::RADIUS_MD`, `core::BORDER_THIN`/`BORDER_FOCUS`, `core::hover_t`, `core::disabled_color`, `core::Size`, `typography::body`.
- **Accessibility** — focus is egui's `TextEdit` focus (the border switches to the ring). Width = `ui.available_width()`.

## API

| Signature | Effect |
|-----------|--------|
| `Input::new(buf: &mut String) -> Self` | Bind to a string buffer. |
| `.placeholder(text: impl Into<String>) -> Self` | Hint text when empty. |
| `.error(error: bool) -> Self` | Force the destructive border. |
| `.size(s: Size) -> Self` / `.sm()` / `.lg()` | Size (`core::Size`). |
| `.enabled(enabled: bool) -> Self` / `.disabled()` | Enable/disable. |
| `.id_source(id: impl Hash) -> Self` | Stable id for the inner `TextEdit` (else auto-id). |
| `.show(self, ui: &mut Ui) -> Response` | Return the `TextEdit` `Response` (`changed` when edited). |

## Usage

```rust
use ouroboros_ui::atoms::Input;

let mut name = String::new();
Input::new(&mut name).placeholder("Your name").show(ui);
```

```rust
use ouroboros_ui::atoms::Input;

let mut email = String::new();
let invalid = !email.contains('@') && !email.is_empty();
let resp = Input::new(&mut email)
    .placeholder("email@example.com")
    .error(invalid)
    .id_source("email_field")
    .show(ui);
if resp.changed() { /* validate */ }
```

## Composition

Atom: paints the box/border/veil directly and embeds a frameless `egui::TextEdit` in a child `Ui`. Composes no other DS atoms.

## Notes

- Binding is `&mut String`; the returned `Response` is the inner `TextEdit`'s, so `.changed()`/`.has_focus()`/`.lost_focus()` reflect editing.
- Greedily takes `ui.available_width()` — constrain the parent for a fixed width.
- In a loop or repeated layout, set `id_source` so the field keeps focus/cursor across frames.

See [tokens](../../tokens.md) · [theming](../../theming.md) · [typography](../../typography.md) · [guards](../../guards.md).
