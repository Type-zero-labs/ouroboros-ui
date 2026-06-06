//! Atoms — the smallest design-system components.
//!
//! Each atom is a builder (`Atom::new(...).setter(...).show(ui) -> Response`) that paints
//! **only** with foundation tokens — no hardcoded colors, sizes, radii, fonts, or motion.
//! That invariant is enforced by `tests/no_raw_values.rs`. Atoms may compose smaller atoms
//! (e.g. [`button::Button`] composes [`icon::Icon`] + [`text::Text`]).
//!
//! Wave 1 (proof): text, heading, icon, divider, button. Form/feedback atoms follow.

pub mod button;
pub mod divider;
pub mod heading;
pub mod icon;
pub mod text;

pub use button::{Button, ButtonSize, ButtonVariant};
pub use divider::{Axis, Divider};
pub use heading::{Heading, HeadingLevel};
pub use icon::Icon;
pub use text::{Text, TextRole};
