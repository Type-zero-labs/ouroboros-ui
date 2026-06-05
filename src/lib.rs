//! # ouroboros-ui
//!
//! Token-first design system for [egui] — the [shadcn/ui] design *language*
//! reimplemented natively in Rust. Not a web port: same vocabulary (semantic tokens,
//! neutral zinc aesthetic, 4px scale), egui-native rendering.
//!
//! ## Layered token architecture
//!
//! Each layer references the one below; nothing below knows the layer above.
//!
//! - [`tokens::core`] — raw primitives (`const`s, no meaning): color ramps, spacing,
//!   radius, shadow, type sizes.
//! - [`tokens::semantic`] — the [`Theme`] struct: shadcn semantic tokens mapped onto core.
//! - [`tokens::component`] — thin per-component override structs (default to semantic).
//! - [`theme`] — [`Mode`] enum, `Theme::resolve(Mode)`, install/get, typography.
//!
//! Foundation milestone: tokens + theme/modes + storybook. Components come later.
//!
//! [egui]: https://github.com/emilk/egui
//! [shadcn/ui]: https://ui.shadcn.com

pub mod theme;
pub mod tokens;

pub use theme::Mode;
pub use tokens::semantic::Theme;
