//! Theme resolution and installation.
//!
//! [`Mode`] selects a palette; `Theme::resolve(Mode)` (T05) produces the resolved
//! [`Theme`](crate::tokens::semantic::Theme); `install`/`get` (T05) wire it into the
//! egui context. [`typography`] registers fonts and exposes the font helpers.

pub mod typography;

/// Color mode. First-class infrastructure; `Dark` is populated, `Light` is a stub
/// that currently resolves to `Dark` (filled in a later milestone).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    Dark,
    Light,
}
