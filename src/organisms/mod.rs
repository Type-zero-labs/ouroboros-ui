//! Organisms — full UI sections composed from cells, molecules and atoms.
//!
//! Same primordial rule (compose, never paint — the guard scans `src/organisms/` too). Overlay
//! organisms (Dialog/Toast/Popover/DropdownMenu) use egui's `Modal`/`Area`/`Popup` containers
//! for placement + a token [`Surface`](crate::atoms::Surface)/themed visuals for the casing.

pub mod accordion;
pub mod app_menu;
pub mod autocomplete;
pub mod dialog;
pub mod drawer;
pub mod dropdown_menu;
pub mod floating_toolbar;
pub mod menubar;
pub mod panel;
pub mod popover;
pub mod rail;
pub mod select;
pub mod side_panel;
pub mod sidebar;
pub mod splitter;
pub mod tab_view;
pub mod table;
pub mod toast;
pub mod toolbar;
pub mod tree_view;

pub use accordion::{Accordion, AccordionCtx};
pub use app_menu::{AppMenu, MenuUi};
pub use autocomplete::Autocomplete;
pub use dialog::{Dialog, DialogChoice};
pub use drawer::Drawer;
pub use dropdown_menu::DropdownMenu;
pub use floating_toolbar::{toolbar_separator, FloatingToolbar};
pub use menubar::Menubar;
pub use panel::{Panel, PanelEdge};
pub use popover::Popover;
pub use rail::{Rail, RailItem, RailResponse};
pub use select::Select;
pub use side_panel::{SidePanel, SidePanelResponse};
pub use sidebar::Sidebar;
pub use splitter::{is_collapsed, set_collapsed, PanelSpec, Splitter, SplitterLayout};
pub use tab_view::TabView;
pub use table::{ColWidth, Column, Table, TableLayout};
pub use toast::{Toast, ToastResult};
pub use toolbar::Toolbar;
pub use tree_view::{TreeItem, TreeView};
