//! Molecules — components composed **only** from atoms (and smaller molecules).
//!
//! The primordial rule, enforced here: a molecule never paints a primitive (`painter.*`,
//! `rect_*`, `circle_*`, `galley`) — it composes atoms (incl. [`Surface`](crate::atoms::Surface))
//! and `auto_layout`. The guard test `tests/no_painter_in_molecules.rs` checks this in CI. If a
//! molecule needs to paint something, the missing piece becomes an atom instead.

pub mod alert;
pub mod breadcrumb;
pub mod card;
pub mod checkbox_card;
pub mod collapsible;
pub mod color_field;
pub mod field;
pub mod input_group;
pub mod radio_card;
pub mod radio_group;
pub mod search_field;
pub mod tabs;
pub mod toggle_group;
pub mod vector_field;

pub use alert::{Alert, AlertVariant};
pub use breadcrumb::Breadcrumb;
pub use card::{Card, CardSize};
pub use checkbox_card::CheckboxCard;
pub use collapsible::Collapsible;
pub use color_field::ColorField;
pub use field::{Field, FieldGroup, FieldOrientation, FieldSeparator, FieldSet};
pub use input_group::{InputGroup, Slot};
pub use radio_card::RadioCard;
pub use radio_group::RadioGroup;
pub use search_field::SearchField;
pub use tabs::Tabs;
pub use toggle_group::ToggleGroup;
pub use vector_field::VectorField;
