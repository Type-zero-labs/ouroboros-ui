//! Molecules — components composed **only** from atoms (and smaller molecules).
//!
//! The primordial rule, enforced here: a molecule never paints a primitive (`painter.*`,
//! `rect_*`, `circle_*`, `galley`) — it composes atoms (incl. [`Surface`](crate::atoms::Surface))
//! and `auto_layout`. The guard test `tests/no_painter_in_molecules.rs` checks this in CI. If a
//! molecule needs to paint something, the missing piece becomes an atom instead.

pub mod card;
pub mod checkbox_card;
pub mod field;
pub mod input_group;
pub mod radio_card;
pub mod radio_group;

pub use card::Card;
pub use checkbox_card::CheckboxCard;
pub use field::Field;
pub use input_group::InputGroup;
pub use radio_card::RadioCard;
pub use radio_group::RadioGroup;
