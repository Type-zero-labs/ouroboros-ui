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
pub mod color_swatch;
pub mod divider;
pub(crate) mod focus;
pub mod heading;
pub mod icon;
pub mod input;
pub mod kbd;
pub mod numeric_field;
pub mod progress;
pub mod radio;
pub mod skeleton;
pub mod slider;
pub mod spinner;
pub mod splitter_handle;
pub mod surface;
pub mod switch;
pub mod text;
pub mod textarea;
pub mod toggle;
pub mod tooltip;

pub use avatar::{Avatar, AvatarSize};
pub use badge::{Badge, BadgeVariant};
pub use button::{Button, ButtonVariant};
pub use checkbox::Checkbox;
pub use color_swatch::ColorSwatch;
pub use divider::{Axis, Divider};
pub use heading::{Heading, HeadingLevel};
pub use icon::Icon;
pub use input::Input;
pub use kbd::Kbd;
pub use numeric_field::NumericField;
pub use progress::Progress;
pub use radio::Radio;
pub use skeleton::Skeleton;
pub use slider::Slider;
pub use spinner::Spinner;
pub use splitter_handle::SplitterHandle;
pub use surface::{Surface, SurfaceBorder, SurfaceFill};
pub use switch::Switch;
pub use text::{Text, TextRole};
pub use textarea::Textarea;
pub use toggle::Toggle;
pub use tooltip::Tooltip;
