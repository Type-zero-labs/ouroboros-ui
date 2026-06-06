//! Atoms — the smallest design-system components.
//!
//! Each atom is a builder (`Atom::new(...).setter(...).show(ui) -> Response`) that paints
//! **only** with foundation tokens — no hardcoded colors, sizes, radii, fonts, or motion.
//! That invariant is enforced by `tests/no_raw_values.rs`. Atoms may compose smaller atoms
//! (e.g. [`button::Button`] composes [`icon::Icon`] + [`text::Text`]).
//!
//! Wave 1 (proof): text, heading, icon, divider, button. Form/feedback atoms follow.

pub mod avatar;
pub mod badge;
pub mod button;
pub mod checkbox;
pub mod divider;
pub mod heading;
pub mod icon;
pub mod input;
pub mod radio;
pub mod spinner;
pub mod surface;
pub mod switch;
pub mod text;
pub mod tooltip;

pub use avatar::{Avatar, AvatarSize};
pub use badge::{Badge, BadgeVariant};
pub use button::{Button, ButtonSize, ButtonVariant};
pub use checkbox::Checkbox;
pub use divider::{Axis, Divider};
pub use heading::{Heading, HeadingLevel};
pub use icon::Icon;
pub use input::Input;
pub use radio::Radio;
pub use spinner::Spinner;
pub use surface::{Surface, SurfaceBorder, SurfaceFill};
pub use switch::Switch;
pub use text::{Text, TextRole};
pub use tooltip::Tooltip;
