# SidePanel

> **Layer:** organism · **Path:** `src/organisms/side_panel.rs` · Since the Oct 2026 shell redesign (Figma frame `200:52739`).

Module panel next to the rail: lowercase mono title, header actions, collapse button, compact search, scrolling body, resize grip on the right edge (owner keeps width/collapsed).

See the storybook page **Shell Lab (Figma)** for the assembled shell, and `tests/shell.rs` for the behavior covered.

`.no_scroll()` hands the body its plain rect (for bodies that lay out their own regions, e.g. a tree above an explorer). Without `.search(..)` the header is just the title row (48 px).
