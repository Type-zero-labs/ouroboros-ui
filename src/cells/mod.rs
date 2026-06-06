//! Cells — compound row/item building blocks for lists, trees, menus, tables and inspectors.
//!
//! A cell sits between molecules and organisms: it composes atoms/molecules into a reusable
//! *row* (a property row, a list/menu/tree item, a toolbar button, a table row). Same primordial
//! rule as molecules — cells **compose, never paint** (the `tests/no_painter_in_molecules.rs`
//! guard scans `src/cells/` too).

pub mod list_item;
pub mod menu_item;
pub mod property_row;
pub mod table_cell;
pub mod table_row;
pub mod toolbar_button;
pub mod tree_node;

pub use list_item::ListItem;
pub use menu_item::MenuItem;
pub use property_row::PropertyRow;
pub use table_cell::{CellAlign, TableCell};
pub use table_row::TableRow;
pub use toolbar_button::ToolbarButton;
pub use tree_node::TreeNode;
