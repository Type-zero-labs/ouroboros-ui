# Credits

`ouroboros-ui` reimplements the **design language** of [shadcn/ui](https://ui.shadcn.com)
— its semantic token model (`background`/`foreground` pairs, `primary`, `muted`, `accent`,
`destructive`, `border`, `ring`…), neutral zinc aesthetic, 4px spacing scale, and
radius/typography conventions — natively in [egui](https://github.com/emilk/egui).

This is **not** a port of shadcn/ui's code (which is React/Tailwind). It is an independent
implementation in Rust that adopts the same vocabulary and visual grammar.

- **shadcn/ui** — MIT License, Copyright (c) 2023 shadcn. <https://github.com/shadcn-ui/ui>

## Bundled fonts

- **Geist Sans** — SIL Open Font License 1.1 (see `assets/fonts/OFL-Geist.txt`).
- **Victor Mono** — SIL Open Font License 1.1 (see `assets/fonts/OFL-VictorMono.txt`).
- **Phosphor Icons** — via the `egui-phosphor` crate (MIT).
