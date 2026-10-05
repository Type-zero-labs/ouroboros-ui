# AppMenu

> **Layer:** organism · **Path:** `src/organisms/app_menu.rs` · Since the Oct 2026 shell redesign (Figma frame `200:52739`).

App-wide menu from the rail logo: sections, `MenuItem` rows (icon, shortcut, check, disabled), separators, submenus; closes on click.

See the storybook page **Shell Lab (Figma)** for the assembled shell, and `tests/shell.rs` for the behavior covered.

The menu is a fixed 288 px wide; `submenu(label, ..)` is a `MenuItem` row with a caret that opens on hover. Row ids derive from the path in the menu (`trigger → sub n → row n`).
