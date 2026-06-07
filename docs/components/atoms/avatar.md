# Avatar

> **Layer:** atom · **Path:** `src/atoms/avatar.rs` · **Exports:** `avatar::{Avatar, AvatarSize}`

A circular avatar that renders centered, uppercased initials over a `muted`-filled disc. This wave is initials-only — image loading is a documented later addition. The diameter and the type style both scale together with [`AvatarSize`].

## Design

- **Purpose / when to use** — represent a user/entity compactly when you have no image. Reach for it in lists, headers, mention chips. Do NOT use it for arbitrary iconography (that is [`Icon`](icon.md)).
- **Anatomy** — a filled circle (`theme.muted`) + a centered initials galley in `theme.foreground`. Initials are uppercased at render time.
- **Variants / sizes / states** — three sizes, no interactive states (sense is `hover` only):

  | Size | Diameter token | Type style |
  |------|----------------|------------|
  | `Sm` | `core::CONTROL_SM` (26px) | `typography::caption()` |
  | `Md` (default) | `core::CONTROL_MD` (32px) | `typography::label()` |
  | `Lg` | `core::CONTROL_LG` (38px) | `typography::body_strong()` |

- **Tokens consumed** — `theme.muted` (disc fill), `theme.foreground` (initials), `core::CONTROL_*` (diameter), typography styles via `theme::typography`.
- **Accessibility** — none beyond the bare `Response`; it allocates with `Sense::hover()` and emits no `widget_info`.

## API

| Signature | Effect |
|-----------|--------|
| `Avatar::new(initials: impl Into<String>) -> Self` | Construct with initials text. |
| `.size(size: AvatarSize) -> Self` | Set the size. |
| `.sm(self) -> Self` | Shorthand for `AvatarSize::Sm`. |
| `.lg(self) -> Self` | Shorthand for `AvatarSize::Lg`. |
| `.show(self, ui: &mut Ui) -> Response` | Allocate, paint, return the hover `Response`. |

**`AvatarSize`** (enum): `Sm`, `Md` (default), `Lg`.

## Usage

```rust
use ouroboros_ui::atoms::Avatar;

Avatar::new("JD").show(ui);
```

```rust
use ouroboros_ui::atoms::{Avatar, AvatarSize};

let resp = Avatar::new("ab").size(AvatarSize::Lg).show(ui); // renders "AB"
if resp.hovered() { /* … */ }
```

## Composition

Atom: paints directly with tokens (`circle_filled` + a layout-job galley). It does not compose other atoms; the initials galley is built inline rather than via [`Text`](text.md).

## Notes

- Initials are `.to_uppercase()`d internally — pass any case.
- The galley uses `extra_letter_spacing = style.tracking` and is centered both axes.
- No truncation/length cap: long strings will overflow the disc; pass 1–2 chars.

See [tokens](../../tokens.md) · [theming](../../theming.md) · [typography](../../typography.md).
