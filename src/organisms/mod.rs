//! Organisms — full UI sections composed from cells, molecules and atoms.
//!
//! Same primordial rule (compose, never paint — the guard scans `src/organisms/` too). Overlay
//! organisms (Dialog/Toast/Popover/DropdownMenu) use egui's `Modal`/`Area`/`Popup` containers
//! for placement + a token [`Surface`](crate::atoms::Surface)/themed visuals for the casing.

pub mod dialog;
pub mod dropdown_menu;
pub mod popover;
pub mod sidebar;
pub mod tab_view;
pub mod table;
pub mod toast;
pub mod toolbar;
pub mod tree_view;

pub use dialog::Dialog;
pub use dropdown_menu::DropdownMenu;
pub use popover::Popover;
pub use sidebar::Sidebar;
pub use tab_view::TabView;
pub use table::Table;
pub use toast::Toast;
pub use toolbar::Toolbar;
pub use tree_view::{TreeItem, TreeView};
