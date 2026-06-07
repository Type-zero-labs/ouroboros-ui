# Guards & conventions

The atomic-design rules are not honor-system — two test guards in `tests/` enforce them,
and they run with `cargo test` (so a violation is a red build, not a review nit). This
page documents exactly what they catch and how to add a component without tripping them.

---

## Guard 1 — atoms (and graph) paint only with tokens

`tests/no_raw_values.rs` → test `atoms_use_only_tokens`. Recursively scans
`src/atoms/**/*.rs` **and `src/graph/**/*.rs`** and fails on any hardcoded design value.
Comments are stripped first, so prose mentioning a pattern is fine.

> The [graph layer](./components/graph/README.md) is allowed to paint (a node editor needs
> grid dots, wires, handles), but it is held to the **same purity contract** as atoms — its
> colors resolve from `Theme` (via `GraphTokens`) and geometry from `core`. That is why this
> guard scans it too.

| Pattern flagged | Why | Use instead |
|-----------------|-----|-------------|
| `Color32::from_*(…)` | hardcoded color | a `Theme` field or `core::*` color |
| `Color32::<CONST>` (e.g. `Color32::WHITE`) | named color constant | a `Theme`/`core` color |
| `FontId::new(…)` | hand-built font | `theme::typography` (`TypeStyle::font_id`, `icon_font`) |
| `Stroke::new(<digit>…)` | raw stroke width | `core::BORDER_THIN` / `BORDER_FOCUS` |
| `CornerRadius::same(<digit>…)` | raw radius | `core::RADIUS_*` |
| `.expand(<digit>…)` | raw offset | a token (e.g. `core::RING_OFFSET`) |

The numeric checks are heuristic: tokens are named consts, so an argument starting with a
letter or `(` is accepted; a **leading digit** means a raw value and trips the guard.
(`Color32` as a bare *type* is fine — only `Color32::` followed by an uppercase name or
`from_` is flagged.)

> Sizes/radii/spacing passed positionally elsewhere aren't all machine-checkable — those
> are caught in review. The guard covers the high-frequency mistakes.

---

## Guard 2 — above atoms you compose, never paint

`tests/no_painter_in_molecules.rs` → test `molecules_compose_never_paint`. Scans
**`src/cells/`, `src/molecules/`, and `src/organisms/`** (despite the file name) and fails
on any direct painting call:

```
ui.painter(   .painter()   rect_filled   rect_stroke   circle_filled
circle_stroke   layout_no_wrap   layout_job   .galley(   Shape::line
hline(   vline(
```

The rule: if a cell/molecule/organism needs to paint something, **the missing piece
becomes an atom** (atoms are the only layer allowed to paint — see `src/atoms/surface.rs`,
the painting primitive everything composes for fills/borders).

> **`src/graph` is deliberately not scanned by this guard.** The node-editor layer is the
> sanctioned exception that paints directly (grid/wires/handles/marquee) — painting there is
> by design. It is still held to token purity by Guard 1, which *does* scan it.

---

## The conventions behind the guards

1. **Builder + `show(ui) -> Response`.** Every component, every layer. Required args in
   `new`, optional props as chainable setters, `show` consumes self and returns an
   `egui::Response`.
2. **Read the theme at the top of `show`.** `let theme = Theme::get(ui);` — this is what
   makes a component react to mode switches for free.
3. **State transitions go through the shared helpers.** Hover via `core::hover_t`,
   disabled via `core::disabled_color`, so every component animates identically.
4. **Domain extensions are marked.** Where the system extends shadcn (status variants,
   `Size` densities), the addition is ours by intent — base variants/anatomy follow shadcn.
5. **New component → storybook page.** Every new component or variant gets an entry in
   `examples/storybook.rs`; without a demo there is nothing to validate visually.

---

## Adding a component without tripping the guards

### A new atom

1. `src/atoms/<name>.rs` — builder struct, `show(self, ui) -> Response`.
2. Source **all** colors from `Theme::get(ui)` / `core::*`, fonts from `typography`,
   strokes/radii from `core::*`. No `Color32::from_`, no `FontId::new`, no raw-digit
   `Stroke::new`/`CornerRadius::same`/`.expand`.
3. Animate state with `core::hover_t` / `core::disabled_color`.
4. Register in `src/atoms/mod.rs` (`pub mod` + `pub use`).
5. Add a storybook page.
6. `cargo test` (guard 1 must stay green) + `cargo fmt`.

### A new cell / molecule / organism

1. File under the right layer dir; builder + `show`.
2. **Compose atoms only** — no painting calls from the forbidden list. If you reach for a
   painter, stop and extract an atom first.
3. Use `auto_layout` / egui layout for arrangement; use `Surface` (atom) for any
   fill/border/casing.
4. Overlay organisms place with egui `Modal`/`Area`/`Popup`; the casing is either a
   `Surface` atom or themed egui visuals driven by `Theme` tokens — never hand-painted.
5. Register in the layer `mod.rs`, add a storybook page, `cargo test` + `cargo fmt`.

---

## Running the guards

```bash
cargo test                              # both guards + unit tests
cargo test --test no_raw_values         # atoms-only
cargo test --test no_painter_in_molecules
```

A failure prints every offending `path:line` with the suggested token, so fixes are
mechanical.
</content>
