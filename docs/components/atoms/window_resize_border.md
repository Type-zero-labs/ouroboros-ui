# WindowResizeBorder

> **Layer:** atom · **Path:** `src/atoms/window_resize_border.rs` · Since the Oct 2026 shell redesign (Figma frame `200:52739`).

Invisible 4 px grips on the edges/corners of an undecorated window; returns the `ResizeEdge` grabbed (hand it to `winit::Window::drag_resize_window`).

See the storybook page **Shell Lab (Figma)** for the assembled shell, and `tests/shell.rs` for the behavior covered.
