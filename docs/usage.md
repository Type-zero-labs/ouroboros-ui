# Usage

How to depend on, bootstrap, and call ouroboros-ui.

---

## Add the dependency

The crate is standalone (its own workspace) and not yet published to crates.io. Depend on
it by git (or path, if vendored as a sibling):

```toml
[dependencies]
ouroboros-ui = { git = "https://github.com/type-zero-labs/ouroboros-ui" }
# or, vendored locally:
ouroboros-ui = { path = "../ouroboros-ui" }
```

You also need egui/eframe at the matching minor (**0.34**); the toolchain is pinned to
Rust **1.92.0** (`rust-toolchain.toml`).

---

## Bootstrap the theme

Install the theme **once** when the egui context exists (in `eframe::App` setup). This
registers the bundled fonts and applies the palette — without it, text falls back to
egui defaults and `Theme::get` returns `Theme::default()`.

```rust
use eframe::egui;
use ouroboros_ui::{Mode, Theme};

struct App { mode: Mode }

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Theme::install(&cc.egui_ctx, Mode::Dark);   // fonts + palette, once
        Self { mode: Mode::Dark }
    }
}
```

Switch mode at runtime with `apply` (no font re-registration):

```rust
if toggled {
    self.mode = match self.mode { Mode::Dark => Mode::Light, Mode::Light => Mode::Dark };
    Theme::apply(ctx, self.mode);
}
```

See [theming.md](./theming.md) for the four palettes and what `install`/`apply` touch.

---

## The builder pattern

Every component — atom to organism — follows the same shape:

```rust
Component::new(required_args)
    .setter(value)   // chainable, returns Self
    .show(ui)        // consumes self, paints, returns egui::Response
```

So call sites read top-to-bottom and optional props are explicit:

```rust
use ouroboros_ui::atoms::{Button, ButtonVariant};
use ouroboros_ui::egui_phosphor::light::FLOPPY_DISK;

let resp = Button::new("Save")
    .variant(ButtonVariant::Default)
    .icon_left(FLOPPY_DISK)
    .show(ui);

if resp.clicked() {
    // …
}
```

Many components offer **shorthand setters** in addition to the generic one — e.g.
`Button::new("x").secondary().sm()` is the same as
`.variant(ButtonVariant::Secondary).size(Size::Sm)`. Check each component page.

---

## Importing

Components are grouped by layer; pull from the layer module:

```rust
use ouroboros_ui::atoms::{Button, Input, Text, Icon, Badge, Switch};
use ouroboros_ui::cells::{ListItem, PropertyRow, MenuItem};
use ouroboros_ui::molecules::{Field, Card, Alert, Tabs, RadioGroup};
use ouroboros_ui::organisms::{Splitter, PanelSpec, Dialog, Table, TreeView, Toast};
```

Foundation re-exports live at the crate root:

```rust
use ouroboros_ui::{Mode, Size, Theme};                 // common
use ouroboros_ui::theme::typography;                   // type styles
use ouroboros_ui::tokens::{core, layout};              // primitives
use ouroboros_ui::auto_layout::{AutoLayout, MainAlign, CrossAlign};
use ouroboros_ui::egui_phosphor::light;                // icon glyphs
```

---

## Common recipes

### A labelled form field

```rust
use ouroboros_ui::molecules::Field;
use ouroboros_ui::atoms::Input;

Field::new("Display name")
    .hint("Shown to other players")
    .show(ui, |ui| { Input::new(&mut self.name).show(ui); });
```

### A toolbar row with a trailing action

```rust
use ouroboros_ui::auto_layout::{AutoLayout, CrossAlign};

AutoLayout::horizontal()
    .gap(core::SPACE_2).cross_align(CrossAlign::Center)
    .hug(|ui| { Text::new("Scene").role(TextRole::Heading).show(ui); })
    .fill(|ui| {})                                  // spacer
    .hug(|ui| { Button::new("Add").sm().show(ui); })
    .show(ui);
```

### Reading the theme in your own widget

```rust
let theme = Theme::get(ui);
let stroke = egui::Stroke::new(core::BORDER_THIN, theme.border);
```

Prefer composing existing atoms over painting yourself — and if you *do* paint custom
chrome, you are effectively writing an atom, so it must use tokens (see
[guards.md](./guards.md)).

---

## Run / develop

```bash
cd ouroboros-ui
cargo run --example storybook   # the living visual reference
cargo test                      # unit tests + the two atomic-design guards
cargo fmt                       # CI runs `cargo fmt --check`
cargo clippy
```

> **Before pushing:** run `cargo fmt` — the project's CI checks formatting and will fail
> an unformatted branch.
</content>
