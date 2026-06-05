# Credits

`ouroboros-ui` reimplements the **design language** of [shadcn/ui](https://ui.shadcn.com)
— its semantic token model (`background`/`foreground` pairs, `primary`, `muted`, `accent`,
`destructive`, `border`, `ring`…), neutral zinc aesthetic, 4px spacing scale, and
radius/typography conventions — natively in [egui](https://github.com/emilk/egui).

This is **not** a port of shadcn/ui's code (which is React/Tailwind). It is an independent
implementation in Rust that adopts the same vocabulary and visual grammar.

- **shadcn/ui** — MIT License, Copyright (c) 2023 shadcn. <https://github.com/shadcn-ui/ui>

## Bundled fonts

The UI and code faces are both from the **Iosevka** superfamily by Renzhi Li
(Belleve Invis) — SIL Open Font License 1.1 (see `assets/fonts/OFL-Iosevka.txt`). The
files are **subset** to Latin + punctuation + arrows + math + box-drawing/block/shape
ranges (~103 KB/weight vs ~10 MB full).

- **Iosevka** (monospace) — UI / body. Weights: Light, Regular, Medium, SemiBold, Bold.
- **IosevkaTerm** (terminal monospace) — code / `kbd`. Weights: Regular, Bold.
- **Phosphor Icons** — via the `egui-phosphor` crate (MIT).
