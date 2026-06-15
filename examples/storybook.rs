//! Storybook — organized gallery for ouroboros-ui, built with the DS itself.
//!
//! The chrome dogfoods the atoms: nav + toggle are [`Button`]s, every text run is a
//! [`Text`]/[`Heading`], separators are [`Divider`]s, and all spacing/radius/color come
//! from tokens. The only primitive painting is the token swatches/bars/shapes — the demo
//! *of* a token must draw the token. Toggle ◐/◑ to compare light and dark.
//!
//! Run: `cargo run --example storybook`

use egui::{vec2, Align, Color32, CornerRadius, Layout, Sense, Stroke, StrokeKind, Ui};
use egui_phosphor::light;
use ouroboros_ui::atoms::{
    Avatar, AvatarSize, Badge, BadgeVariant, Button, ButtonVariant, Checkbox, ColorSwatch, Divider,
    Heading, Icon, Input, Kbd, NumericField, Progress, Radio, Skeleton, Slider, Spinner, Surface,
    SurfaceFill, Switch, Text, TextRole, Textarea, Toggle, Tooltip,
};
use ouroboros_ui::auto_layout::{AutoLayout, CrossAlign, MainAlign};
use ouroboros_ui::cells::{
    ListItem, MenuItem, PropertyRow, ResponsiveRow, TableCell, TableRow, ToolbarButton, TreeNode,
};
use ouroboros_ui::graph::{
    EdgeStyle, GraphView, HandleSpec, NodeFrame, NodeId, NodeKindId, NodeSearch, NodeStatus, Port,
    PortId, PortSide,
};
use ouroboros_ui::molecules::{
    Alert, AlertVariant, Breadcrumb, Card, CheckboxCard, Collapsible, ColorField, Field,
    FieldSeparator, FieldSet, InputGroup, RadioCard, RadioGroup, SearchField, Slot, Tabs,
    ToggleGroup, VectorField,
};
use ouroboros_ui::organisms::{
    Accordion, Column, Dialog, DialogChoice, DropdownMenu, Menubar, Panel, PanelSpec, Popover,
    Select, Sidebar, Splitter, TabView, Table, Toast, Toolbar, TreeItem, TreeView,
};
use ouroboros_ui::theme::typography;
use ouroboros_ui::tokens::{core, layout};
use ouroboros_ui::{Mode, Size, Theme};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Colors,
    Typography,
    Spacing,
    Radius,
    Shadows,
    Sizing,
    Opacity,
    Motion,
    LayoutTokens,
    AutoLayoutDemo,
    ResizeLab,
    Text,
    Heading,
    Icon,
    Divider,
    Button,
    Checkbox,
    Radio,
    Switch,
    Input,
    Badge,
    Spinner,
    Avatar,
    Tooltip,
    Surface,
    Textarea,
    Slider,
    NumericField,
    ColorSwatch,
    Progress,
    Skeleton,
    Toggle,
    Kbd,
    Field,
    RadioGroup,
    Card,
    CheckboxCard,
    RadioCard,
    InputGroup,
    Tabs,
    Collapsible,
    Alert,
    ToggleGroup,
    Breadcrumb,
    VectorField,
    ColorField,
    SearchField,
    PropertyRow,
    ResponsiveRow,
    ListItem,
    MenuItem,
    TreeNode,
    ToolbarButton,
    TableRow,
    Toolbar,
    TabView,
    Table,
    TreeView,
    Sidebar,
    Dialog,
    Toast,
    Popover,
    DropdownMenu,
    Select,
    Accordion,
    Menubar,
    Splitter,
    Panel,
    GraphLive,
    GraphNode,
    GraphEdge,
    GraphSearch,
}

impl Page {
    fn label(self) -> &'static str {
        match self {
            Page::Colors => "Colors",
            Page::Typography => "Typography",
            Page::Spacing => "Spacing",
            Page::Radius => "Radius",
            Page::Shadows => "Shadows",
            Page::Sizing => "Sizing",
            Page::Opacity => "Opacity & overlays",
            Page::Motion => "Motion",
            Page::LayoutTokens => "Layout & panels",
            Page::AutoLayoutDemo => "Auto-layout (Figma)",
            Page::ResizeLab => "Resize Lab",
            Page::Text => "Text",
            Page::Heading => "Heading",
            Page::Icon => "Icon",
            Page::Divider => "Divider",
            Page::Button => "Button",
            Page::Checkbox => "Checkbox",
            Page::Radio => "Radio",
            Page::Switch => "Switch",
            Page::Input => "Input",
            Page::Badge => "Badge",
            Page::Spinner => "Spinner",
            Page::Avatar => "Avatar",
            Page::Tooltip => "Tooltip",
            Page::Surface => "Surface",
            Page::Textarea => "Textarea",
            Page::Slider => "Slider",
            Page::NumericField => "Numeric field",
            Page::ColorSwatch => "Color swatch",
            Page::Progress => "Progress",
            Page::Skeleton => "Skeleton",
            Page::Toggle => "Toggle",
            Page::Kbd => "Kbd",
            Page::Field => "Field",
            Page::RadioGroup => "RadioGroup",
            Page::Card => "Card",
            Page::CheckboxCard => "Checkbox card",
            Page::RadioCard => "Radio card",
            Page::InputGroup => "Input group",
            Page::Tabs => "Tabs",
            Page::Collapsible => "Collapsible",
            Page::Alert => "Alert",
            Page::ToggleGroup => "Toggle group",
            Page::Breadcrumb => "Breadcrumb",
            Page::VectorField => "Vector field",
            Page::ColorField => "Color field",
            Page::SearchField => "Search field",
            Page::PropertyRow => "Property row",
            Page::ResponsiveRow => "Responsive row",
            Page::ListItem => "List item",
            Page::MenuItem => "Menu item",
            Page::TreeNode => "Tree node",
            Page::ToolbarButton => "Toolbar button",
            Page::TableRow => "Table row",
            Page::Toolbar => "Toolbar",
            Page::TabView => "Tab view",
            Page::Table => "Table",
            Page::TreeView => "Tree view",
            Page::Sidebar => "Sidebar",
            Page::Dialog => "Dialog",
            Page::Toast => "Toast",
            Page::Popover => "Popover",
            Page::DropdownMenu => "Dropdown menu",
            Page::Select => "Select",
            Page::Accordion => "Accordion",
            Page::Menubar => "Menubar",
            Page::Splitter => "Splitter",
            Page::Panel => "Panel",
            Page::GraphLive => "Live graph",
            Page::GraphNode => "Node variants",
            Page::GraphEdge => "Edge variants",
            Page::GraphSearch => "Node search",
        }
    }
}

const NAV: &[(&str, &[Page])] = &[
    (
        "TOKENS",
        &[
            Page::Colors,
            Page::Typography,
            Page::Spacing,
            Page::Radius,
            Page::Shadows,
            Page::Sizing,
            Page::Opacity,
            Page::Motion,
        ],
    ),
    (
        "LAYOUT",
        &[Page::LayoutTokens, Page::AutoLayoutDemo, Page::ResizeLab],
    ),
    (
        "ATOMS",
        &[
            Page::Text,
            Page::Heading,
            Page::Icon,
            Page::Divider,
            Page::Button,
            Page::Checkbox,
            Page::Radio,
            Page::Switch,
            Page::Input,
            Page::Badge,
            Page::Spinner,
            Page::Avatar,
            Page::Tooltip,
            Page::Surface,
            Page::Textarea,
            Page::Slider,
            Page::NumericField,
            Page::ColorSwatch,
            Page::Progress,
            Page::Skeleton,
            Page::Toggle,
            Page::Kbd,
        ],
    ),
    (
        "MOLECULES",
        &[
            Page::Field,
            Page::RadioGroup,
            Page::Card,
            Page::CheckboxCard,
            Page::RadioCard,
            Page::InputGroup,
            Page::Tabs,
            Page::Collapsible,
            Page::Alert,
            Page::ToggleGroup,
            Page::Breadcrumb,
            Page::VectorField,
            Page::ColorField,
            Page::SearchField,
        ],
    ),
    (
        "CELLS",
        &[
            Page::PropertyRow,
            Page::ResponsiveRow,
            Page::ListItem,
            Page::MenuItem,
            Page::TreeNode,
            Page::ToolbarButton,
            Page::TableRow,
        ],
    ),
    (
        "ORGANISMS",
        &[
            Page::Toolbar,
            Page::TabView,
            Page::Table,
            Page::TreeView,
            Page::Sidebar,
            Page::Dialog,
            Page::Toast,
            Page::Popover,
            Page::DropdownMenu,
            Page::Select,
            Page::Accordion,
            Page::Menubar,
            Page::Splitter,
            Page::Panel,
        ],
    ),
    (
        "GRAPH",
        &[
            Page::GraphLive,
            Page::GraphNode,
            Page::GraphEdge,
            Page::GraphSearch,
        ],
    ),
];

fn main() -> eframe::Result<()> {
    let mut installed = false;
    let mut mode = Mode::Dark;
    let mut page = Page::Colors;
    eframe::run_ui_native(
        "ouroboros-ui storybook",
        eframe::NativeOptions::default(),
        move |ui, _frame| {
            if !installed {
                // `set_fonts` only takes effect next frame — install, then skip this frame.
                Theme::install(ui.ctx(), mode);
                installed = true;
                ui.ctx().request_repaint();
                return;
            }
            let theme = Theme::get(ui);
            ui.painter()
                .rect_filled(ui.clip_rect(), 0.0, theme.background);

            egui::Panel::top("header")
                .frame(header_frame(&theme))
                .show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        Heading::new("ouroboros-ui").h2().show(ui);
                        ui.add_space(core::SPACE_3);
                        Text::new("design system").muted().show(ui);
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let label = match mode {
                                Mode::Dark => "◐ Dark",
                                Mode::Light => "◑ Light",
                            };
                            if Button::new(label)
                                .ghost()
                                .sm()
                                .id_source("toggle")
                                .show(ui)
                                .clicked()
                            {
                                mode = match mode {
                                    Mode::Dark => Mode::Light,
                                    Mode::Light => Mode::Dark,
                                };
                                Theme::apply(ui.ctx(), mode);
                                ui.ctx().request_repaint();
                            }
                        });
                    });
                });

            egui::Panel::left("nav")
                .resizable(false)
                .exact_size(190.0)
                .frame(nav_frame(&theme))
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("nav_scroll")
                        .show(ui, |ui| nav(ui, &theme, &mut page));
                });

            egui::CentralPanel::default()
                .frame(content_frame(&theme))
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("content_scroll")
                        .show(ui, |ui| {
                            ui.set_max_width(760.0);
                            Heading::new(page.label()).h1().show(ui);
                            ui.add_space(core::SPACE_3);
                            Divider::horizontal().show(ui);
                            ui.add_space(core::SPACE_5);
                            render_page(ui, &theme, page);
                            ui.add_space(core::SPACE_8);
                        });
                });
        },
    )
}

// ── Frames (token-driven) ─────────────────────────────────────────────────────

fn header_frame(theme: &Theme) -> egui::Frame {
    egui::Frame::default()
        .fill(theme.background)
        .inner_margin(egui::Margin::symmetric(
            core::SPACE_5 as i8,
            core::SPACE_3 as i8,
        ))
        .stroke(Stroke::new(core::BORDER_THIN, theme.border))
}

fn nav_frame(theme: &Theme) -> egui::Frame {
    egui::Frame::default()
        .fill(theme.card)
        .inner_margin(core::SPACE_3)
        .stroke(Stroke::new(core::BORDER_THIN, theme.border))
}

fn content_frame(theme: &Theme) -> egui::Frame {
    egui::Frame::default()
        .fill(theme.background)
        .inner_margin(core::SPACE_6)
}

// ── Nav (dogfoods Button + Text) ──────────────────────────────────────────────

fn nav(ui: &mut Ui, theme: &Theme, page: &mut Page) {
    for (category, pages) in NAV {
        ui.add_space(core::SPACE_3);
        Text::new(*category).caption().muted().show(ui);
        ui.add_space(core::SPACE_1);
        for &p in *pages {
            let variant = if *page == p {
                ButtonVariant::Secondary
            } else {
                ButtonVariant::Ghost
            };
            if Button::new(p.label())
                .variant(variant)
                .sm()
                .id_source(("nav", p.label()))
                .show(ui)
                .clicked()
            {
                *page = p;
            }
        }
    }
    let _ = theme;
}

// ── Pages ─────────────────────────────────────────────────────────────────────

fn render_page(ui: &mut Ui, theme: &Theme, page: Page) {
    match page {
        Page::Colors => page_colors(ui, theme),
        Page::Typography => page_typography(ui, theme),
        Page::Spacing => page_spacing(ui, theme),
        Page::Radius => page_radius(ui, theme),
        Page::Shadows => page_shadows(ui, theme),
        Page::Sizing => page_sizing(ui, theme),
        Page::Opacity => page_opacity(ui, theme),
        Page::Motion => page_motion(ui, theme),
        Page::LayoutTokens => page_layout_tokens(ui, theme),
        Page::AutoLayoutDemo => page_auto_layout(ui, theme),
        Page::ResizeLab => page_resize_lab(ui, theme),
        Page::Text => page_text(ui, theme),
        Page::Heading => page_heading(ui, theme),
        Page::Icon => page_icon(ui, theme),
        Page::Divider => page_divider(ui, theme),
        Page::Button => page_button(ui, theme),
        Page::Checkbox => page_checkbox(ui, theme),
        Page::Radio => page_radio(ui, theme),
        Page::Switch => page_switch(ui, theme),
        Page::Input => page_input(ui, theme),
        Page::Badge => page_badge(ui, theme),
        Page::Spinner => page_spinner(ui, theme),
        Page::Avatar => page_avatar(ui, theme),
        Page::Tooltip => page_tooltip(ui, theme),
        Page::Surface => page_surface(ui, theme),
        Page::Textarea => page_textarea(ui, theme),
        Page::Slider => page_slider(ui, theme),
        Page::NumericField => page_numeric_field(ui, theme),
        Page::ColorSwatch => page_color_swatch(ui, theme),
        Page::Progress => page_progress(ui, theme),
        Page::Skeleton => page_skeleton(ui, theme),
        Page::Toggle => page_toggle(ui, theme),
        Page::Kbd => page_kbd(ui, theme),
        Page::Field => page_field(ui, theme),
        Page::RadioGroup => page_radio_group(ui, theme),
        Page::Card => page_card(ui, theme),
        Page::CheckboxCard => page_checkbox_card(ui, theme),
        Page::RadioCard => page_radio_card(ui, theme),
        Page::InputGroup => page_input_group(ui, theme),
        Page::Tabs => page_tabs(ui, theme),
        Page::Collapsible => page_collapsible(ui, theme),
        Page::Alert => page_alert(ui, theme),
        Page::ToggleGroup => page_toggle_group(ui, theme),
        Page::Breadcrumb => page_breadcrumb(ui, theme),
        Page::VectorField => page_vector_field(ui, theme),
        Page::ColorField => page_color_field(ui, theme),
        Page::SearchField => page_search_field(ui, theme),
        Page::PropertyRow => page_property_row(ui, theme),
        Page::ResponsiveRow => page_responsive_row(ui, theme),
        Page::ListItem => page_list_item(ui, theme),
        Page::MenuItem => page_menu_item(ui, theme),
        Page::TreeNode => page_tree_node(ui, theme),
        Page::ToolbarButton => page_toolbar_button(ui, theme),
        Page::TableRow => page_table_row(ui, theme),
        Page::Toolbar => page_toolbar(ui, theme),
        Page::TabView => page_tab_view(ui, theme),
        Page::Table => page_table(ui, theme),
        Page::TreeView => page_tree_view(ui, theme),
        Page::Sidebar => page_sidebar(ui, theme),
        Page::Dialog => page_dialog(ui, theme),
        Page::Toast => page_toast(ui, theme),
        Page::Popover => page_popover(ui, theme),
        Page::DropdownMenu => page_dropdown_menu(ui, theme),
        Page::Select => page_select(ui, theme),
        Page::Accordion => page_accordion(ui, theme),
        Page::Menubar => page_menubar(ui, theme),
        Page::Splitter => page_splitter(ui, theme),
        Page::Panel => page_panel(ui, theme),
        Page::GraphLive => page_graph_live(ui, theme),
        Page::GraphNode => page_graph_node(ui, theme),
        Page::GraphEdge => page_graph_edge(ui, theme),
        Page::GraphSearch => page_graph_search(ui, theme),
    }
}

fn page_graph_node(ui: &mut Ui, _theme: &Theme) {
    caption(
        ui,
        "Node variants: base · status · placeholder · sized · labeled handles",
    );
    ui.add_space(core::SPACE_3);
    GraphView::new("sb_graph_node")
        .size(vec2(680.0, 360.0))
        .show(ui, |g| {
            g.node(
                NodeId(1),
                egui::pos2(20.0, 20.0),
                NodeFrame::base()
                    .title("Status")
                    .status(NodeStatus::Running)
                    .appendix("last run 2s ago")
                    .tooltip("a node with a status badge")
                    .handle(HandleSpec::input(0).label("in"))
                    .handle(HandleSpec::output(1).label("out")),
                |ui| {
                    Text::new("body content").muted().show(ui);
                },
            );
            g.node(
                NodeId(2),
                egui::pos2(300.0, 20.0),
                NodeFrame::placeholder().title("Placeholder"),
                |ui| {
                    Text::new("drop a node here").muted().show(ui);
                },
            );
            g.node(
                NodeId(3),
                egui::pos2(20.0, 200.0),
                NodeFrame::base()
                    .title("Sized")
                    .size(vec2(180.0, 96.0))
                    .input(0),
                |ui| {
                    Text::new("fixed size (select to resize)").muted().show(ui);
                },
            );
        });
}

fn page_graph_edge(ui: &mut Ui, _theme: &Theme) {
    caption(
        ui,
        "Edge variants: default · animated · with-button · with-label",
    );
    ui.add_space(core::SPACE_3);
    let styles = [
        ("Default", EdgeStyle::Default),
        ("Animated", EdgeStyle::Animated),
        ("Button", EdgeStyle::WithButton),
        ("Label", EdgeStyle::WithLabel),
    ];
    GraphView::new("sb_graph_edge")
        .size(vec2(680.0, 380.0))
        .show(ui, |g| {
            for (i, (label, style)) in styles.iter().enumerate() {
                let y = 20.0 + i as f32 * 88.0;
                let a = 100 + i as u64 * 2;
                let b = a + 1;
                g.node(
                    NodeId(a),
                    egui::pos2(20.0, y),
                    NodeFrame::base().title(*label).output(1),
                    |_ui| {},
                );
                g.node(
                    NodeId(b),
                    egui::pos2(360.0, y),
                    NodeFrame::base().input(0),
                    |_ui| {},
                );
                g.edge(
                    Port {
                        node: NodeId(a),
                        port: PortId(1),
                        side: PortSide::Out,
                    },
                    Port {
                        node: NodeId(b),
                        port: PortId(0),
                        side: PortSide::In,
                    },
                    *style,
                );
            }
        });
}

fn page_graph_search(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Node search — a command palette of node kinds");
    ui.add_space(core::SPACE_3);
    let trigger = Button::new("Add node")
        .icon_left(light::PLUS)
        .id_source("sb_search_trigger")
        .show(ui);
    let chosen = NodeSearch::new()
        .kind(NodeKindId(1), "Trigger")
        .kind(NodeKindId(2), "Condition")
        .kind(NodeKindId(3), "Action")
        .kind(NodeKindId(4), "Delay")
        .show(ui, &trigger);
    if let Some(k) = chosen {
        ui.data_mut(|d| d.insert_temp(egui::Id::new("sb_search_last"), k.0));
    }
    let last: Option<u64> = ui.data(|d| d.get_temp(egui::Id::new("sb_search_last")));
    ui.add_space(core::SPACE_2);
    if let Some(k) = last {
        Text::new(format!("last picked kind id: {k}"))
            .muted()
            .show(ui);
    } else {
        Text::new("click \"Add node\" and pick a kind")
            .muted()
            .show(ui);
    }
}

fn page_graph_live(ui: &mut Ui, _theme: &Theme) {
    caption(
        ui,
        "Drag nodes to move · click to select · middle-drag to pan · scroll to zoom",
    );
    ui.add_space(core::SPACE_3);

    // Demo graph data lives in egui memory (the caller owns it; the lib owns only view state).
    let data_id = egui::Id::new("storybook_graph_data");
    let mut nodes: Vec<(u64, egui::Pos2, String)> = ui.data_mut(|d| {
        d.get_temp(data_id).unwrap_or_else(|| {
            vec![
                (1, egui::pos2(40.0, 60.0), "Trigger".to_owned()),
                (2, egui::pos2(340.0, 60.0), "Condition".to_owned()),
                (3, egui::pos2(340.0, 240.0), "Action".to_owned()),
            ]
        })
    });
    let edges_id = egui::Id::new("storybook_graph_edges");
    let mut edges: Vec<(u64, u64)> = ui.data_mut(|d| d.get_temp(edges_id).unwrap_or_default());
    // Last frame's selection (the lib owns it; mirrored here to drive the per-node toolbar).
    let sel_id = egui::Id::new("storybook_graph_sel");
    let prev_sel: Vec<u64> = ui.data_mut(|d| d.get_temp(sel_id).unwrap_or_default());
    let mut tb_delete: Vec<u64> = Vec::new();

    let resp = GraphView::new("storybook_graph_live")
        .size(vec2(720.0, 420.0))
        .grid(true)
        .controls(true)
        .minimap(true)
        .show(ui, |g| {
            // Nodes first (so their handle positions are known), each input 0 / output 1.
            for (id, pos, label) in &nodes {
                let frame = NodeFrame::base().title(label.clone()).input(0).output(1);
                g.node(NodeId(*id), *pos, frame, |ui| {
                    Text::new("body content").muted().show(ui);
                });
            }
            // A toolbar above each selected node (delete action).
            for &sid in &prev_sel {
                g.node_toolbar(NodeId(sid), |ui| {
                    if Button::new("")
                        .ghost()
                        .sm()
                        .icon_only()
                        .icon_left(light::TRASH)
                        .id_source(("tb_del", sid))
                        .show(ui)
                        .clicked()
                    {
                        tb_delete.push(sid);
                    }
                });
            }
            // Then edges (bezier; drawn under the nodes via the reserved slot).
            for (from, to) in &edges {
                g.edge(
                    Port {
                        node: NodeId(*from),
                        port: PortId(1),
                        side: PortSide::Out,
                    },
                    Port {
                        node: NodeId(*to),
                        port: PortId(0),
                        side: PortSide::In,
                    },
                    EdgeStyle::Default,
                );
            }
        });

    // Commit the library's intents back into the caller-owned data.
    for (moved_id, delta) in &resp.node_moved {
        if let Some((_, pos, _)) = nodes.iter_mut().find(|(id, _, _)| NodeId(*id) == *moved_id) {
            *pos += *delta;
        }
    }
    if let Some(c) = resp.connection {
        let pair = (c.from.node.0, c.to.node.0);
        if c.from.node != c.to.node && !edges.contains(&pair) {
            edges.push(pair);
        }
    }
    // Commit deletes (Delete/Backspace): drop the edge, or the selected nodes + their edges.
    if let Some((from, to)) = resp.delete_edge {
        edges.retain(|(f, t)| !(*f == from.node.0 && *t == to.node.0));
    }
    if !resp.delete_nodes.is_empty() {
        let gone: Vec<u64> = resp.delete_nodes.iter().map(|n| n.0).collect();
        nodes.retain(|(id, _, _)| !gone.contains(id));
        edges.retain(|(f, t)| !gone.contains(f) && !gone.contains(t));
    }
    // Toolbar delete buttons.
    if !tb_delete.is_empty() {
        nodes.retain(|(id, _, _)| !tb_delete.contains(id));
        edges.retain(|(f, t)| !tb_delete.contains(f) && !tb_delete.contains(t));
    }
    // Mirror current selection for next frame's toolbars.
    let cur_sel: Vec<u64> = resp.selection.iter().map(|n| n.0).collect();
    ui.data_mut(|d| {
        d.insert_temp(data_id, nodes);
        d.insert_temp(edges_id, edges);
        d.insert_temp(sel_id, cur_sel);
    });
}

fn page_splitter(ui: &mut Ui, _theme: &Theme) {
    caption(
        ui,
        "Drag dividers · min/max · double-click to collapse · nested",
    );
    let panel = |ui: &mut Ui, label: &str| {
        Surface::new().muted().show(ui, |ui| {
            ui.set_min_size(ui.available_size());
            Text::new(label).muted().show(ui);
        });
    };
    ui.allocate_ui(vec2(560.0, 320.0), |ui| {
        Splitter::horizontal()
            .id_source("sp_demo")
            .panel(PanelSpec::new().min(120.0).max(280.0), |ui| {
                panel(ui, "Hierarchy")
            })
            .panel(PanelSpec::new(), |ui| {
                // Nested vertical split inside the center panel.
                Splitter::vertical()
                    .id_source("sp_demo_nested")
                    .panel(PanelSpec::new(), |ui| panel(ui, "Viewport"))
                    .panel(PanelSpec::new().size(0.3).collapsible(true), |ui| {
                        panel(ui, "Console (collapsible)")
                    })
                    .show(ui);
            })
            .panel(PanelSpec::new().min(160.0).collapsible(true), |ui| {
                panel(ui, "Inspector (collapsible)")
            })
            .show(ui);
    });

    ui.add_space(core::SPACE_4);
    caption(
        ui,
        "Fixed bands (PanelSpec::fixed) — header/footer keep their px; the flex body takes the rest",
    );
    ui.allocate_ui(vec2(560.0, 240.0), |ui| {
        Splitter::vertical()
            .id_source("sp_demo_fixed")
            .panel(PanelSpec::fixed(layout::TOOLBAR_HEIGHT), |ui| {
                panel(ui, "Header — fixed 40px")
            })
            .panel(PanelSpec::flex(), |ui| {
                // The body itself can still host a resizable horizontal split.
                Splitter::horizontal()
                    .id_source("sp_demo_fixed_body")
                    .panel(PanelSpec::new().size(0.25), |ui| panel(ui, "Aside"))
                    .panel(PanelSpec::flex(), |ui| panel(ui, "Body — flex"))
                    .show(ui);
            })
            .panel(PanelSpec::fixed(layout::STATUSBAR_HEIGHT), |ui| {
                panel(ui, "Footer — fixed 24px")
            })
            .show(ui);
    });
}

fn page_select(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Dropdown single-select");
    let id = egui::Id::new("select_demo");
    let mut sel = ui.data(|d| d.get_temp::<usize>(id).unwrap_or(0));
    Select::new(&mut sel)
        .options(["Opaque", "Cutout", "Transparent", "Additive"])
        .placeholder("Blend mode…")
        .show(ui);
    ui.data_mut(|d| d.insert_temp(id, sel));
    subhead(ui, "Sizes (Sm / Md / Lg)");
    ui.horizontal(|ui| {
        for (salt, mk) in [
            ("sel_sm", Size::Sm),
            ("sel_md", Size::Md),
            ("sel_lg", Size::Lg),
        ] {
            let sid = egui::Id::new(salt);
            let mut s = ui.data(|d| d.get_temp::<usize>(sid).unwrap_or(0));
            ui.push_id(salt, |ui| {
                Select::new(&mut s)
                    .options(["Low", "Medium", "High"])
                    .size(mk)
                    .show(ui);
            });
            ui.data_mut(|d| d.insert_temp(sid, s));
            ui.add_space(core::SPACE_2);
        }
    });
}

fn page_panel(ui: &mut Ui, _theme: &Theme) {
    caption(
        ui,
        "Docked panel chrome — bg + flush edge + header + token-padded body",
    );
    let id = egui::Id::new("panel_demo");
    let mut vals = ui.data(|d| d.get_temp::<[f32; 3]>(id).unwrap_or([1.0, 0.5, 0.2]));
    ui.horizontal_top(|ui| {
        // Right-docked inspector: left edge + header + responsive rows.
        ui.allocate_ui(vec2(280.0, 300.0), |ui| {
            Panel::new("panel_inspector")
                .left_edge()
                .title("Inspector")
                .show(ui, |ui| {
                    for (i, name) in ["Mass", "Drag", "Bounce"].iter().enumerate() {
                        ResponsiveRow::new(*name).show(ui, |ui| {
                            NumericField::new(&mut vals[i]).speed(0.05).show(ui)
                        });
                    }
                });
        });
        ui.add_space(core::SPACE_4);
        // Left-docked panel with a footer action bar.
        ui.allocate_ui(vec2(220.0, 300.0), |ui| {
            Panel::new("panel_footer")
                .right_edge()
                .title("Properties")
                .footer(|ui| {
                    let _ = Button::new("Apply").sm().show(ui);
                })
                .show(ui, |ui| {
                    ResponsiveRow::new("X").show(ui, |ui| {
                        NumericField::new(&mut vals[0]).speed(0.05).show(ui)
                    });
                    ResponsiveRow::new("Y").show(ui, |ui| {
                        NumericField::new(&mut vals[1]).speed(0.05).show(ui)
                    });
                });
        });
    });
    ui.data_mut(|d| d.insert_temp(id, vals));
}

fn page_accordion(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Card variant · free content (any widgets)");
    let id = egui::Id::new("acc_bool");
    let mut on = ui.data(|d| d.get_temp::<bool>(id).unwrap_or(true));
    ui.allocate_ui(vec2(360.0, 220.0), |ui| {
        Accordion::new().card().show(ui, |acc| {
            acc.section("Transform", |ui| {
                let mut p = [1.0_f32, 0.0, -1.0];
                VectorField::new(&mut p).speed(0.05).show(ui);
            });
            acc.section("Rendering", |ui| {
                Field::new("Cast shadows")
                    .horizontal()
                    .show(ui, |ui| Switch::new(&mut on).show(ui));
            });
            acc.section("Physics", |ui| {
                Text::new("Collider, mass, drag").muted().show(ui);
            });
        });
    });
    ui.data_mut(|d| d.insert_temp(id, on));
}

fn page_menubar(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Application menu bar");
    if let Some((m, i)) = Menubar::new()
        .menu("File", ["New", "Open", "Save", "Quit"])
        .menu("Edit", ["Undo", "Redo", "Preferences"])
        .menu("View", ["Zoom in", "Zoom out", "Reset"])
        .show(ui)
    {
        ui.data_mut(|d| d.insert_temp(egui::Id::new("mb_last"), (m, i)));
    }
    if let Some((m, i)) = ui.data(|d| d.get_temp::<(usize, usize)>(egui::Id::new("mb_last"))) {
        ui.add_space(core::SPACE_2);
        Text::new(format!("Chose menu {m}, item {i}"))
            .muted()
            .caption()
            .show(ui);
    }
}

fn page_toolbar(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Action bar (icon toggles + dividers)");
    let id = egui::Id::new("toolbar_demo");
    let mut s = ui.data(|d| d.get_temp::<[bool; 3]>(id).unwrap_or([true, false, false]));
    Toolbar::new().show(ui, |ui| {
        ToolbarButton::new(&mut s[0], light::CURSOR)
            .tooltip("Select")
            .id_source("tba")
            .show(ui);
        ToolbarButton::new(&mut s[1], light::ARROWS_OUT)
            .tooltip("Move")
            .id_source("tbb")
            .show(ui);
        ToolbarButton::new(&mut s[2], light::ARROWS_CLOCKWISE)
            .tooltip("Rotate")
            .id_source("tbc")
            .show(ui);
        Divider::vertical().show(ui);
        Button::new("Play")
            .icon_left(light::PLAY)
            .sm()
            .id_source("tb_play")
            .show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id, s));
}

fn page_tab_view(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Tabs + active panel");
    let id = egui::Id::new("tabview_demo");
    let mut sel = ui.data(|d| d.get_temp::<usize>(id).unwrap_or(0));
    TabView::new(&mut sel)
        .tabs(["Scene", "Game", "Console"])
        .show(ui, |ui, idx| {
            let body = match idx {
                0 => "3D scene viewport.",
                1 => "Game preview.",
                _ => "Log output…",
            };
            Text::new(body).muted().show(ui);
        });
    ui.data_mut(|d| d.insert_temp(id, sel));
}

fn page_table(ui: &mut Ui, theme: &Theme) {
    caption(
        ui,
        "Column-defined · striped · border · selectable · sticky header (click a row)",
    );
    let data = [
        ("hero.fbx", "Mesh", "2.1 MB", theme.success),
        ("grass.png", "Texture", "512 KB", theme.success),
        ("main.rs", "Script", "8 KB", theme.success),
        ("sky.hdr", "Texture", "4.0 MB", theme.success),
        ("orphan.mat", "Material", "1 KB", theme.error),
        ("anim.act", "Animation", "320 KB", theme.success),
    ];
    ui.allocate_ui(vec2(460.0, 220.0), |ui| {
        Table::new()
            .id_source("tbl_main")
            .columns([
                Column::new("Name"),
                Column::new("Type").exact(110.0),
                Column::new("Size").exact(90.0).end(),
                Column::new("Status").exact(110.0),
            ])
            .rows(data.iter().map(|(n, t, s, c)| {
                TableRow::new([
                    TableCell::text(*n),
                    TableCell::text(*t).muted(),
                    TableCell::text(*s).end(),
                    TableCell::text("ref").status(*c),
                ])
            }))
            .striped(true)
            .border(true)
            .selectable(true)
            .max_height(150.0)
            .show(ui);
    });
    subhead(ui, "Sizes (Sm / Md / Lg)");
    for (key, mk) in [
        ("tbl_sm", Size::Sm),
        ("tbl_md", Size::Md),
        ("tbl_lg", Size::Lg),
    ] {
        ui.allocate_ui(vec2(300.0, 90.0), |ui| {
            Table::new()
                .id_source(key)
                .size(mk)
                .border(true)
                .columns([Column::new("Key"), Column::new("Value").end()])
                .rows([
                    TableRow::new([TableCell::text("width"), TableCell::text("1920").end()]),
                    TableRow::new([TableCell::text("height"), TableCell::text("1080").end()]),
                ])
                .show(ui);
        });
        ui.add_space(core::SPACE_2);
    }
    subhead(ui, "Empty · loading");
    ui.horizontal(|ui| {
        ui.allocate_ui(vec2(220.0, 70.0), |ui| {
            Table::new()
                .border(true)
                .columns([Column::new("Assets")])
                .empty_text("No assets")
                .show(ui);
        });
        ui.add_space(core::SPACE_4);
        ui.allocate_ui(vec2(220.0, 70.0), |ui| {
            Table::new()
                .border(true)
                .columns([Column::new("Assets")])
                .rows([TableRow::new([TableCell::text("…")])])
                .loading(true)
                .show(ui);
        });
    });

    subhead(ui, "Editable cells (Table::layout + NumericField)");
    let id = egui::Id::new("tbl_edit_vals");
    let mut vals = ui
        .data(|d| d.get_temp::<[f32; 3]>(id))
        .unwrap_or([10.0, 20.0, 30.0]);
    let labels = ["STR", "AGI", "VIT"];
    ui.allocate_ui(vec2(300.0, 150.0), |ui| {
        let layout = Table::new()
            .id_source("tbl_edit")
            .border(true)
            .columns([Column::new("Stat"), Column::new("Value").exact(120.0)])
            .layout(ui, 3);
        for (i, row) in layout.rects.iter().enumerate() {
            if row.len() == 2 {
                let mut lui = ui.new_child(egui::UiBuilder::new().max_rect(row[0]));
                lui.set_clip_rect(row[0]);
                Text::new(labels[i]).show(&mut lui);
                let mut vui = ui.new_child(egui::UiBuilder::new().max_rect(row[1]));
                vui.set_clip_rect(row[1]);
                NumericField::new(&mut vals[i]).decimals(0).show(&mut vui);
            }
        }
    });
    ui.data_mut(|d| d.insert_temp(id, vals));
}

fn page_tree_view(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Scene hierarchy");
    let id = egui::Id::new("treeview_demo");
    let mut sel = ui.data(|d| d.get_temp::<usize>(id).unwrap_or(1));
    ui.allocate_ui(vec2(260.0, 180.0), |ui| {
        TreeView::new(&mut sel)
            .items([
                TreeItem::new("Scene").icon(light::FOLDER).expanded(true),
                TreeItem::new("Player").depth(1).icon(light::CUBE),
                TreeItem::new("Camera").depth(1).icon(light::CUBE),
                TreeItem::new("Environment")
                    .depth(1)
                    .icon(light::FOLDER)
                    .expanded(false),
            ])
            .show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id, sel));
}

fn page_sidebar(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Navigation list + icon rail");
    let id = egui::Id::new("sidebar_demo");
    let mut sel = ui.data(|d| d.get_temp::<usize>(id).unwrap_or(0));
    ui.horizontal(|ui| {
        ui.allocate_ui(vec2(180.0, 180.0), |ui| {
            Sidebar::new(&mut sel)
                .item(light::HOUSE, "Home")
                .item(light::CUBE, "Assets")
                .item(light::GEAR, "Settings")
                .show(ui);
        });
        ui.add_space(core::SPACE_4);
        ui.allocate_ui(vec2(40.0, 180.0), |ui| {
            Sidebar::new(&mut sel)
                .item(light::HOUSE, "Home")
                .item(light::CUBE, "Assets")
                .item(light::GEAR, "Settings")
                .icons_only()
                .show(ui);
        });
        ui.add_space(core::SPACE_4);
        // Icon rail with 24px glyphs (ICON_XL) — a denser editor-style rail.
        ui.allocate_ui(vec2(56.0, 180.0), |ui| {
            Sidebar::new(&mut sel)
                .item(light::HOUSE, "Home")
                .item(light::CUBE, "Assets")
                .item(light::GEAR, "Settings")
                .icons_only()
                .icon_size(core::ICON_XL)
                .show(ui);
        });
    });
    ui.data_mut(|d| d.insert_temp(id, sel));
}

fn page_dialog(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Modal dialog");
    let id = egui::Id::new("dialog_open");
    let mut open = ui.data(|d| d.get_temp::<bool>(id).unwrap_or(false));
    if Button::new("Delete asset…")
        .variant(ButtonVariant::Destructive)
        .id_source("dlg_trigger")
        .show(ui)
        .clicked()
    {
        open = true;
    }
    if open {
        let mut dismiss = false;
        let close = Dialog::new("Delete asset?")
            .description("This action cannot be undone.")
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    if Button::new("Delete")
                        .variant(ButtonVariant::Destructive)
                        .id_source("dlg_del")
                        .show(ui)
                        .clicked()
                    {
                        dismiss = true;
                    }
                    ui.add_space(core::SPACE_2);
                    if Button::new("Cancel")
                        .ghost()
                        .id_source("dlg_cancel")
                        .show(ui)
                        .clicked()
                    {
                        dismiss = true;
                    }
                });
            });
        if close || dismiss {
            open = false;
        }
    }
    ui.data_mut(|d| d.insert_temp(id, open));

    ui.add_space(core::SPACE_4);
    caption(
        ui,
        "Confirm variant (.confirm — replaces the legacy prompt)",
    );
    let cid = egui::Id::new("dlg_confirm_open");
    let mut copen = ui.data(|d| d.get_temp::<bool>(cid).unwrap_or(false));
    if Button::new("Discard changes…")
        .secondary()
        .id_source("dlg_c_trigger")
        .show(ui)
        .clicked()
    {
        copen = true;
    }
    if copen {
        let choice = Dialog::new("Discard changes?")
            .description("Your edits will be lost. This cannot be undone.")
            .destructive()
            .id_source("dlg_confirm")
            .confirm(ui.ctx(), "Discard", "Keep editing");
        if choice != DialogChoice::None {
            copen = false;
        }
    }
    ui.data_mut(|d| d.insert_temp(cid, copen));
}

fn page_toast(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Transient notification (top-right)");
    let id = egui::Id::new("toast_show");
    let mut show = ui.data(|d| d.get_temp::<bool>(id).unwrap_or(false));
    if Button::new(if show { "Hide toast" } else { "Show toast" })
        .id_source("toast_btn")
        .show(ui)
        .clicked()
    {
        show = !show;
    }
    if show
        && Toast::new("Build finished in 2.3s")
            .success()
            .dismissible()
            .show(ui.ctx())
    {
        show = false;
    }
    ui.data_mut(|d| d.insert_temp(id, show));
}

fn page_popover(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Click to open anchored content");
    let resp = Button::new("Open popover").id_source("pop_btn").show(ui);
    Popover::new().show(&resp, |ui| {
        Text::new("Popover content").body_strong().show(ui);
        ui.add_space(core::SPACE_1);
        Text::new("Anchored to the trigger.")
            .muted()
            .caption()
            .show(ui);
    });
}

fn page_dropdown_menu(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Menu from a trigger");
    let resp = Button::new("Actions")
        .icon_right(light::CARET_DOWN)
        .id_source("dd_btn")
        .show(ui);
    if let Some(i) = DropdownMenu::new()
        .item(light::COPY, "Copy")
        .item(light::CLIPBOARD, "Paste")
        .item(light::TRASH, "Delete")
        .show(&resp)
    {
        ui.data_mut(|d| d.insert_temp(egui::Id::new("dd_last"), i));
    }
    let last = ui.data(|d| d.get_temp::<usize>(egui::Id::new("dd_last")));
    if let Some(i) = last {
        ui.add_space(core::SPACE_2);
        Text::new(format!("Last action index: {i}"))
            .muted()
            .caption()
            .show(ui);
    }
}

fn page_property_row(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Inspector rows (aligned label column)");
    let id = egui::Id::new("prop_demo");
    let mut vals = ui.data(|d| d.get_temp::<[f32; 3]>(id).unwrap_or([1.0, 0.05, 0.6]));
    ui.allocate_ui(vec2(360.0, 140.0), |ui| {
        for (i, name) in ["Mass", "Drag", "Bounce"].iter().enumerate() {
            PropertyRow::new(*name).show(ui, |ui| {
                NumericField::new(&mut vals[i]).speed(0.05).show(ui)
            });
        }
    });
    ui.data_mut(|d| d.insert_temp(id, vals));
}

fn page_responsive_row(ui: &mut Ui, _theme: &Theme) {
    caption(
        ui,
        "Inspector rows that stack when narrow (< INSPECTOR_ROW_STACK_MIN)",
    );
    let id = egui::Id::new("resprow_demo");
    let mut vals = ui.data(|d| d.get_temp::<[f32; 3]>(id).unwrap_or([1.0, 0.05, 0.6]));
    subhead(ui, "Wide (≥ threshold) — aligned label column");
    ui.allocate_ui(vec2(360.0, 120.0), |ui| {
        for (i, name) in ["Mass", "Drag", "Bounce"].iter().enumerate() {
            ResponsiveRow::new(*name).show(ui, |ui| {
                NumericField::new(&mut vals[i]).speed(0.05).show(ui)
            });
        }
    });
    subhead(ui, "Narrow (< threshold) — label stacked above control");
    ui.allocate_ui(vec2(180.0, 200.0), |ui| {
        for (i, name) in ["Mass", "Drag", "Bounce"].iter().enumerate() {
            ResponsiveRow::new(*name).show(ui, |ui| {
                NumericField::new(&mut vals[i]).speed(0.05).show(ui)
            });
        }
    });
    ui.data_mut(|d| d.insert_temp(id, vals));
}

fn page_list_item(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Selectable list rows");
    let id = egui::Id::new("li_demo");
    let mut sel = ui.data(|d| d.get_temp::<usize>(id).unwrap_or(0));
    ui.allocate_ui(vec2(280.0, 160.0), |ui| {
        for (i, (icon, title, sub)) in [
            (light::CUBE, "Cube", "Mesh"),
            (light::STAR, "Light", "Point"),
            (light::GEAR, "Settings", "Project"),
        ]
        .iter()
        .enumerate()
        {
            if ListItem::new(*title)
                .icon(icon)
                .subtitle(*sub)
                .selected(sel == i)
                .id_source(("li", i))
                .show(ui)
                .clicked()
            {
                sel = i;
            }
        }
    });
    ui.data_mut(|d| d.insert_temp(id, sel));
}

fn page_menu_item(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Menu rows (icon + label + shortcut)");
    ui.allocate_ui(vec2(240.0, 140.0), |ui| {
        MenuItem::new("Copy")
            .icon(light::COPY)
            .shortcut("Ctrl C")
            .id_source("mi_c")
            .show(ui);
        MenuItem::new("Paste")
            .icon(light::CLIPBOARD)
            .shortcut("Ctrl V")
            .id_source("mi_v")
            .show(ui);
        MenuItem::new("Delete")
            .icon(light::TRASH)
            .id_source("mi_d")
            .show(ui);
        MenuItem::new("Disabled")
            .enabled(false)
            .id_source("mi_x")
            .show(ui);
    });
    subhead(
        ui,
        "Checkable (View-menu toggles — unchecked reserves the mark slot)",
    );
    ui.allocate_ui(vec2(240.0, 110.0), |ui| {
        MenuItem::new("Show Grid")
            .checked(true)
            .id_source("mi_chk_on")
            .show(ui);
        MenuItem::new("Show Gizmos")
            .checked(false)
            .id_source("mi_chk_off")
            .show(ui);
        MenuItem::new("Snap to Grid")
            .checked(true)
            .shortcut("Ctrl G")
            .id_source("mi_chk_sc")
            .show(ui);
    });
}

fn page_tree_node(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Hierarchy rows (indent + caret)");
    ui.allocate_ui(vec2(280.0, 180.0), |ui| {
        TreeNode::new("Scene")
            .icon(light::FOLDER)
            .expandable(true)
            .id_source("tn0")
            .show(ui);
        TreeNode::new("Player")
            .depth(1)
            .icon(light::CUBE)
            .expandable(false)
            .selected(true)
            .id_source("tn1")
            .show(ui);
        TreeNode::new("Camera")
            .depth(1)
            .icon(light::CUBE)
            .id_source("tn2")
            .show(ui);
        TreeNode::new("Mesh")
            .depth(2)
            .icon(light::CUBE)
            .id_source("tn3")
            .show(ui);
    });
}

fn page_toolbar_button(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Toolbar icon toggles (hover for tooltip)");
    let id = egui::Id::new("tb_demo");
    let mut state = ui.data(|d| d.get_temp::<[bool; 3]>(id).unwrap_or([true, false, false]));
    ui.horizontal(|ui| {
        ToolbarButton::new(&mut state[0], light::CURSOR)
            .tooltip("Select")
            .id_source("tb0")
            .show(ui);
        ToolbarButton::new(&mut state[1], light::ARROWS_OUT)
            .tooltip("Move")
            .id_source("tb1")
            .show(ui);
        ToolbarButton::new(&mut state[2], light::ARROWS_CLOCKWISE)
            .tooltip("Rotate")
            .id_source("tb2")
            .show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id, state));
}

fn page_table_row(ui: &mut Ui, theme: &Theme) {
    caption(
        ui,
        "TableCell — cell vs header (weight), alignment, status dot",
    );
    // The cell fills the width it's given; demo it inside fixed-width boxes.
    fn cell(ui: &mut Ui, w: f32, c: TableCell) {
        ui.allocate_ui(vec2(w, core::CONTROL_MD), |ui| {
            c.show(ui);
        });
    }
    let widths = [140.0_f32, 90.0, 120.0];
    ui.horizontal(|ui| {
        cell(ui, widths[0], TableCell::text("Name").header());
        cell(ui, widths[1], TableCell::text("Size").header().end());
        cell(ui, widths[2], TableCell::text("Status").header());
    });
    Divider::horizontal().show(ui);
    for (n, s, label, color) in [
        ("hero.fbx", "2.1 MB", "ref", theme.success),
        ("orphan.mat", "1 KB", "broken", theme.error),
    ] {
        ui.horizontal(|ui| {
            cell(ui, widths[0], TableCell::text(n));
            cell(ui, widths[1], TableCell::text(s).end());
            cell(ui, widths[2], TableCell::text(label).status(color));
        });
    }
}

fn page_tabs(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Container (default) + icons");
    let id = egui::Id::new("tabs_demo");
    let mut sel = ui.data(|d| d.get_temp::<usize>(id).unwrap_or(0));
    Tabs::new(&mut sel)
        .tab("Scene", light::CUBE)
        .tab("Game", light::PLAY)
        .tab("Assets", light::FOLDER)
        .show(ui);
    ui.data_mut(|d| d.insert_temp(id, sel));
    subhead(ui, "Line variant");
    let id2 = egui::Id::new("tabs_line");
    let mut s2 = ui.data(|d| d.get_temp::<usize>(id2).unwrap_or(0));
    Tabs::new(&mut s2)
        .tabs([
            "Overview",
            "Stats",
            "Geometry",
            "Materials",
            "Physics",
            "Notes",
        ])
        .line()
        .show(ui);
    ui.data_mut(|d| d.insert_temp(id2, s2));
}

fn page_collapsible(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Foldout (inspector section)");
    ui.allocate_ui(vec2(360.0, 120.0), |ui| {
        Collapsible::new("Transform")
            .default_open(true)
            .show(ui, |ui| {
                Text::new("Position / Rotation / Scale").muted().show(ui);
            });
        ui.add_space(core::SPACE_2);
        Collapsible::new("Rendering").show(ui, |ui| {
            Text::new("Material, shadows…").muted().show(ui);
        });
    });
}

fn page_alert(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Status callouts");
    ui.allocate_ui(vec2(420.0, 260.0), |ui| {
        for (v, msg) in [
            (AlertVariant::Info, "Build finished in 2.3s."),
            (AlertVariant::Success, "All tests passed."),
            (AlertVariant::Warning, "Mesh has no UVs."),
            (AlertVariant::Error, "Shader failed to compile."),
        ] {
            Alert::new(msg).variant(v).title("Notice").show(ui);
            ui.add_space(core::SPACE_2);
        }
    });
}

fn page_toggle_group(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Segmented single-select");
    let id = egui::Id::new("tgg_demo");
    let mut sel = ui.data(|d| d.get_temp::<usize>(id).unwrap_or(0));
    ToggleGroup::new(&mut sel)
        .options(["Local", "World"])
        .show(ui);
    ui.data_mut(|d| d.insert_temp(id, sel));
}

fn page_breadcrumb(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Path trail (click a crumb)");
    Breadcrumb::new()
        .items(["Assets", "Models", "Characters", "hero.fbx"])
        .show(ui);
}

fn page_vector_field(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Vec3 (transform-style)");
    let id = egui::Id::new("vec_demo");
    let mut v = ui.data(|d| d.get_temp::<[f32; 3]>(id).unwrap_or([1.0, 0.0, -1.0]));
    ui.allocate_ui(vec2(360.0, core::CONTROL_MD), |ui| {
        VectorField::new(&mut v).speed(0.05).show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id, v));
}

fn page_color_field(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Editable hex · click the swatch → HSV/RGB picker");
    let id = egui::Id::new("cf_demo");
    let mut c = ui.data(|d| d.get_temp::<Color32>(id).unwrap_or(core::TEAL_400));
    ui.allocate_ui(vec2(240.0, core::CONTROL_MD), |ui| {
        ColorField::new(&mut c).id_source("cf1").show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id, c));
}

fn page_search_field(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Search preset");
    let id = egui::Id::new("search_demo");
    let mut s = ui.data(|d| d.get_temp::<String>(id).unwrap_or_default());
    ui.allocate_ui(vec2(300.0, core::CONTROL_MD + core::SPACE_4), |ui| {
        SearchField::new(&mut s)
            .placeholder("Search assets…")
            .show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id, s));
}

fn page_surface(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Fills + border + elevation");
    ui.horizontal(|ui| {
        Surface::new().show(ui, |ui| {
            Text::new("card + border").show(ui);
        });
        ui.add_space(core::SPACE_3);
        Surface::new().muted().border_none().show(ui, |ui| {
            Text::new("muted").show(ui);
        });
        ui.add_space(core::SPACE_3);
        Surface::new().elevated().show(ui, |ui| {
            Text::new("elevated").show(ui);
        });
    });
    subhead(ui, "Fill none / interactive");
    Surface::new()
        .fill(SurfaceFill::None)
        .border_strong()
        .show(ui, |ui| {
            Text::new("outline only").show(ui);
        });
}

fn page_slider(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Drag or click · range/step");
    let id = egui::Id::new("sld_a");
    let mut v = ui.data(|d| d.get_temp::<f32>(id).unwrap_or(0.5));
    ui.allocate_ui(vec2(320.0, 24.0), |ui| {
        Slider::new(&mut v).show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id, v));
    ui.add_space(core::SPACE_2);
    Text::new(format!("value = {v:.2}"))
        .muted()
        .caption()
        .show(ui);
    let id2 = egui::Id::new("sld_b");
    let mut s = ui.data(|d| d.get_temp::<f32>(id2).unwrap_or(50.0));
    ui.allocate_ui(vec2(320.0, 24.0), |ui| {
        Slider::new(&mut s).range(0.0, 100.0).step(5.0).show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id2, s));
    subhead(ui, "Sizes (Sm / Md / Lg)");
    for (key, mk) in [
        ("sld_sm", Size::Sm),
        ("sld_md", Size::Md),
        ("sld_lg", Size::Lg),
    ] {
        let id = egui::Id::new(key);
        let mut z = ui.data(|d| d.get_temp::<f32>(id).unwrap_or(0.5));
        ui.allocate_ui(vec2(320.0, 24.0), |ui| {
            Slider::new(&mut z).size(mk).show(ui);
        });
        ui.data_mut(|d| d.insert_temp(id, z));
        ui.add_space(core::SPACE_2);
    }
    subhead(ui, "Disabled");
    let mut d0 = 0.4;
    ui.allocate_ui(vec2(320.0, 24.0), |ui| {
        Slider::new(&mut d0).disabled().show(ui);
    });
}

fn page_numeric_field(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Right-aligned · suffix / no suffix");
    let id = egui::Id::new("num_a");
    let mut v = ui.data(|d| d.get_temp::<f32>(id).unwrap_or(1.0));
    ui.allocate_ui(vec2(160.0, core::CONTROL_MD), |ui| {
        NumericField::new(&mut v).speed(0.05).suffix(" m").show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id, v));
    ui.add_space(core::SPACE_2);
    let id0 = egui::Id::new("num_plain");
    let mut p = ui.data(|d| d.get_temp::<f32>(id0).unwrap_or(42.0));
    ui.allocate_ui(vec2(160.0, core::CONTROL_MD), |ui| {
        NumericField::new(&mut p).show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id0, p));
    subhead(ui, "Fixed decimals (.decimals(2))");
    let idd = egui::Id::new("num_dec");
    let mut d2 = ui.data(|d| d.get_temp::<f32>(idd).unwrap_or(1.5));
    ui.allocate_ui(vec2(160.0, core::CONTROL_MD), |ui| {
        NumericField::new(&mut d2).speed(0.01).decimals(2).show(ui);
    });
    ui.data_mut(|d| d.insert_temp(idd, d2));
    subhead(ui, "Stepper (−/+)");
    let id2 = egui::Id::new("num_step");
    let mut s = ui.data(|d| d.get_temp::<f32>(id2).unwrap_or(3.0));
    ui.allocate_ui(vec2(160.0, core::CONTROL_MD), |ui| {
        NumericField::new(&mut s)
            .range(0.0, 10.0)
            .step(1.0)
            .stepper()
            .show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id2, s));
    subhead(
        ui,
        "Stepper · fixed width (.fixed_width()) — constant under squeeze",
    );
    let id2f = egui::Id::new("num_step_fixed");
    let mut sf = ui.data(|d| d.get_temp::<f32>(id2f).unwrap_or(3.0));
    // Allocated into a wide box, but `.fixed_width()` pins it to NUMERIC_STEPPER_W so the
    // value never slides behind the `−` when the panel is squeezed (ds-inspector).
    ui.allocate_ui(vec2(320.0, core::CONTROL_MD), |ui| {
        NumericField::new(&mut sf)
            .range(0.0, 10.0)
            .step(1.0)
            .stepper()
            .fixed_width()
            .show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id2f, sf));
    subhead(ui, "Sizes (Sm / Md / Lg)");
    let idz = egui::Id::new("num_sizes");
    let mut z = ui.data(|d| d.get_temp::<f32>(idz).unwrap_or(5.0));
    ui.horizontal(|ui| {
        ui.allocate_ui(vec2(90.0, core::CONTROL_LG), |ui| {
            NumericField::new(&mut z).sm().show(ui);
        });
        ui.add_space(core::SPACE_2);
        ui.allocate_ui(vec2(90.0, core::CONTROL_LG), |ui| {
            NumericField::new(&mut z).show(ui);
        });
        ui.add_space(core::SPACE_2);
        ui.allocate_ui(vec2(90.0, core::CONTROL_LG), |ui| {
            NumericField::new(&mut z).lg().show(ui);
        });
    });
    ui.data_mut(|d| d.insert_temp(idz, z));
    subhead(ui, "Error · disabled");
    let ide = egui::Id::new("num_err");
    let mut e = ui.data(|d| d.get_temp::<f32>(ide).unwrap_or(-1.0));
    ui.horizontal(|ui| {
        ui.allocate_ui(vec2(120.0, core::CONTROL_MD), |ui| {
            NumericField::new(&mut e).error(true).show(ui);
        });
        ui.add_space(core::SPACE_2);
        let mut d0 = 0.0;
        ui.allocate_ui(vec2(120.0, core::CONTROL_MD), |ui| {
            NumericField::new(&mut d0).disabled().show(ui);
        });
    });
    ui.data_mut(|d| d.insert_temp(ide, e));
}

fn page_color_swatch(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Square + circle");
    ui.horizontal(|ui| {
        for c in [
            core::RED_500,
            core::GREEN_500,
            core::AMBER_500,
            core::BLUE_500,
        ] {
            ColorSwatch::new(c).show(ui);
            ui.add_space(core::SPACE_2);
        }
        ColorSwatch::new(core::BLUE_400).circle().show(ui);
    });
}

fn page_progress(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Continuous (rounded)");
    for f in [0.15_f32, 0.5, 0.85] {
        ui.allocate_ui(vec2(320.0, 8.0), |ui| {
            Progress::new(f).show(ui);
        });
        ui.add_space(core::SPACE_2);
    }
    subhead(ui, "Stepped");
    ui.allocate_ui(vec2(320.0, 8.0), |ui| {
        Progress::new(0.6).steps(5).show(ui);
    });
    subhead(ui, "Circular (determinate ring)");
    ui.horizontal(|ui| {
        for f in [0.25_f32, 0.6, 0.9] {
            Progress::new(f).circular().show(ui);
            ui.add_space(core::SPACE_4);
        }
    });
}

fn page_skeleton(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Loading placeholders (pulse)");
    ui.allocate_ui(vec2(280.0, 20.0), |ui| {
        Skeleton::new().height(core::SPACE_5).show(ui)
    });
    ui.add_space(core::SPACE_2);
    ui.allocate_ui(vec2(200.0, 16.0), |ui| {
        Skeleton::new().width(200.0).show(ui)
    });
    ui.add_space(core::SPACE_2);
    Skeleton::new().width(140.0).still().show(ui);
}

fn page_toggle(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Two-state button (on = accent)");
    let id = egui::Id::new("tg_demo");
    let mut bold = ui.data(|d| d.get_temp::<bool>(id).unwrap_or(true));
    ui.horizontal(|ui| {
        Toggle::new(&mut bold)
            .icon(light::TEXT_B)
            .label("Bold")
            .id_source("tg_b")
            .show(ui);
        ui.add_space(core::SPACE_2);
        let mut italic = false;
        Toggle::new(&mut italic)
            .icon(light::TEXT_ITALIC)
            .id_source("tg_i")
            .show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id, bold));
}

fn page_kbd(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Keyboard keys");
    ui.horizontal(|ui| {
        for k in ["Ctrl", "Shift", "K", "Esc"] {
            Kbd::new(k).show(ui);
            ui.add_space(core::SPACE_2);
        }
    });
}

fn page_textarea(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Multi-line · rows · error");
    let id = egui::Id::new("ta_demo");
    let mut s = ui.data(|d| d.get_temp::<String>(id).unwrap_or_default());
    ui.allocate_ui(vec2(360.0, 90.0), |ui| {
        Textarea::new(&mut s)
            .rows(3)
            .placeholder("Write a note…")
            .show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id, s));
    ui.add_space(core::SPACE_4);
    let mut e = String::from("too short");
    ui.allocate_ui(vec2(360.0, 70.0), |ui| {
        Textarea::new(&mut e).rows(2).error(true).show(ui);
    });
}

fn page_field(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Vertical (default) — label + control + hint/error");
    let id = egui::Id::new("fld_a");
    let mut s = ui.data(|d| d.get_temp::<String>(id).unwrap_or_default());
    ui.allocate_ui(vec2(360.0, 80.0), |ui| {
        Field::new("Email")
            .required()
            .hint("We never share it")
            .show(ui, |ui| {
                Input::new(&mut s).placeholder("you@example.com").show(ui)
            });
    });
    ui.data_mut(|d| d.insert_temp(id, s));
    let mut e = String::new();
    ui.allocate_ui(vec2(360.0, 80.0), |ui| {
        Field::new("Username")
            .error("Already taken")
            .show(ui, |ui| Input::new(&mut e).error(true).show(ui));
    });

    subhead(ui, "Horizontal (label ↔ control)");
    let id2 = egui::Id::new("fld_sw");
    let mut on = ui.data(|d| d.get_temp::<bool>(id2).unwrap_or(true));
    ui.allocate_ui(vec2(420.0, 40.0), |ui| {
        Field::new("Vsync")
            .horizontal()
            .show(ui, |ui| Switch::new(&mut on).show(ui));
    });
    ui.data_mut(|d| d.insert_temp(id2, on));

    subhead(ui, "FieldSet + legend, FieldSeparator");
    ui.allocate_ui(vec2(360.0, 120.0), |ui| {
        FieldSet::new().legend("Display").show(ui, |ui| {
            let id3 = egui::Id::new("fld_res");
            let mut sel = ui.data(|d| d.get_temp::<usize>(id3).unwrap_or(0));
            RadioGroup::new(&mut sel)
                .options(["Windowed", "Fullscreen"])
                .show(ui);
            ui.data_mut(|d| d.insert_temp(id3, sel));
        });
    });
    ui.add_space(core::SPACE_3);
    FieldSeparator::new().label("OR").show(ui);
}

fn page_radio_group(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Single-select");
    let id = egui::Id::new("rg_demo");
    let mut sel = ui.data(|d| d.get_temp::<usize>(id).unwrap_or(0));
    RadioGroup::new(&mut sel)
        .options(["Small", "Medium", "Large"])
        .show(ui);
    ui.data_mut(|d| d.insert_temp(id, sel));
}

fn page_card(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Header + action + content + footer");
    ui.allocate_ui(vec2(360.0, 220.0), |ui| {
        Card::new()
            .title("Project settings")
            .description("Manage your project preferences")
            .action(|ui| {
                Button::new("")
                    .icon_left(light::DOTS_THREE)
                    .icon_only()
                    .ghost()
                    .sm()
                    .id_source("card_menu")
                    .show(ui);
            })
            .footer(|ui| {
                ui.horizontal(|ui| {
                    Button::new("Save").id_source("card_save").show(ui);
                    ui.add_space(core::SPACE_2);
                    Button::new("Cancel")
                        .ghost()
                        .id_source("card_cancel")
                        .show(ui);
                });
            })
            .show(ui, |ui| {
                Text::new("Card body content goes here.").show(ui);
            });
    });
    subhead(ui, "size = sm");
    ui.allocate_ui(vec2(300.0, 90.0), |ui| {
        Card::new().sm().title("Compact").show(ui, |ui| {
            Text::new("Tighter spacing.").show(ui);
        });
    });
}

fn page_checkbox_card(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Selectable card (whole card toggles)");
    let id = egui::Id::new("cc_demo");
    let mut on = ui.data(|d| d.get_temp::<bool>(id).unwrap_or(true));
    ui.allocate_ui(vec2(360.0, 64.0), |ui| {
        CheckboxCard::new(&mut on, "Enable notifications")
            .description("Email + in-app alerts")
            .id_source("cc")
            .show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id, on));
}

fn page_radio_card(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Radio cards (single-select, consumer-managed)");
    let id = egui::Id::new("rc_demo");
    let mut sel = ui.data(|d| d.get_temp::<usize>(id).unwrap_or(0));
    for (i, (title, desc)) in [
        ("Starter", "Up to 10 projects"),
        ("Pro", "Unlimited projects"),
    ]
    .iter()
    .enumerate()
    {
        ui.allocate_ui(vec2(360.0, 56.0), |ui| {
            if RadioCard::new(sel == i, *title)
                .description(*desc)
                .id_source(("rc", i))
                .show(ui)
                .clicked()
            {
                sel = i;
            }
        });
        ui.add_space(core::SPACE_2);
    }
    ui.data_mut(|d| d.insert_temp(id, sel));
}

fn page_input_group(ui: &mut Ui, _theme: &Theme) {
    let row_h = core::CONTROL_MD + 2.0 * core::SPACE_2 + core::SPACE_2;
    caption(ui, "Icon addons (inline)");
    let id = egui::Id::new("ig_search");
    let mut s = ui.data(|d| d.get_temp::<String>(id).unwrap_or_default());
    ui.allocate_ui(vec2(360.0, row_h), |ui| {
        InputGroup::new(&mut s)
            .leading_icon(light::MAGNIFYING_GLASS)
            .button(Slot::TrailingInline, light::X, || {})
            .placeholder("Search…")
            .id_source("ig_search")
            .show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id, s));

    subhead(ui, "Text addon ($ / .com)");
    let id2 = egui::Id::new("ig_price");
    let mut p = ui.data(|d| d.get_temp::<String>(id2).unwrap_or_default());
    ui.allocate_ui(vec2(360.0, row_h), |ui| {
        InputGroup::new(&mut p)
            .leading_text("$")
            .text(Slot::TrailingInline, "USD")
            .placeholder("0.00")
            .id_source("ig_price")
            .show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id2, p));

    subhead(ui, "Block addon + multiline (textarea)");
    let id3 = egui::Id::new("ig_note");
    let mut n = ui.data(|d| d.get_temp::<String>(id3).unwrap_or_default());
    ui.allocate_ui(vec2(360.0, 110.0), |ui| {
        InputGroup::new(&mut n)
            .text(Slot::BlockStart, "Description")
            .multiline(3)
            .placeholder("Markdown supported…")
            .id_source("ig_note")
            .show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id3, n));
}

fn page_switch(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "States");
    let id = egui::Id::new("demo_switch");
    let mut on = ui.data(|d| d.get_temp::<bool>(id).unwrap_or(true));
    ui.horizontal(|ui| {
        Switch::new(&mut on).show(ui);
        ui.add_space(core::SPACE_2);
        Text::new(if on { "on" } else { "off" }).muted().show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id, on));
    ui.add_space(core::SPACE_3);
    let mut t = true;
    let mut f = false;
    ui.horizontal(|ui| {
        Switch::new(&mut t).disabled().show(ui);
        ui.add_space(core::SPACE_2);
        Switch::new(&mut f).disabled().show(ui);
        ui.add_space(core::SPACE_2);
        Text::new("disabled").muted().show(ui);
    });
    subhead(ui, "Sizes (Sm / Md / Lg)");
    ui.horizontal(|ui| {
        let mut s = true;
        Switch::new(&mut s).sm().id_source("sw_sm").show(ui);
        ui.add_space(core::SPACE_3);
        let mut m = true;
        Switch::new(&mut m).id_source("sw_md").show(ui);
        ui.add_space(core::SPACE_3);
        let mut l = true;
        Switch::new(&mut l).lg().id_source("sw_lg").show(ui);
    });
}

fn page_input(ui: &mut Ui, _theme: &Theme) {
    fn field(ui: &mut Ui, key: &str, render: impl FnOnce(&mut Ui, &mut String)) {
        let id = egui::Id::new(key);
        let mut s = ui.data(|d| d.get_temp::<String>(id).unwrap_or_default());
        ui.allocate_ui(vec2(320.0, core::CONTROL_MD), |ui| render(ui, &mut s));
        ui.data_mut(|d| d.insert_temp(id, s));
        ui.add_space(core::SPACE_3);
    }
    caption(ui, "Default / placeholder");
    field(ui, "in_a", |ui, s| {
        Input::new(s).placeholder("Type here…").show(ui);
    });
    caption(ui, "Error");
    field(ui, "in_b", |ui, s| {
        Input::new(s).placeholder("Required").error(true).show(ui);
    });
    caption(ui, "Disabled");
    field(ui, "in_c", |ui, s| {
        Input::new(s).placeholder("Disabled").disabled().show(ui);
    });
    subhead(ui, "Sizes (Sm / Md / Lg)");
    for (key, mk) in [
        ("in_sm", Size::Sm),
        ("in_md", Size::Md),
        ("in_lg", Size::Lg),
    ] {
        let id = egui::Id::new(key);
        let mut s = ui.data(|d| d.get_temp::<String>(id).unwrap_or_default());
        ui.allocate_ui(vec2(320.0, core::CONTROL_LG), |ui| {
            Input::new(&mut s).placeholder(key).size(mk).show(ui);
        });
        ui.data_mut(|d| d.insert_temp(id, s));
        ui.add_space(core::SPACE_2);
    }
}

fn page_badge(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "shadcn variants");
    ui.horizontal_wrapped(|ui| {
        for (name, v) in [
            ("Default", BadgeVariant::Default),
            ("Secondary", BadgeVariant::Secondary),
            ("Destructive", BadgeVariant::Destructive),
            ("Outline", BadgeVariant::Outline),
            ("Ghost", BadgeVariant::Ghost),
            ("Link", BadgeVariant::Link),
        ] {
            Badge::new(name).variant(v).show(ui);
            ui.add_space(core::SPACE_2);
        }
    });
    subhead(ui, "Status (domain) — with dot");
    ui.horizontal_wrapped(|ui| {
        for (name, v) in [
            ("Success", BadgeVariant::Success),
            ("Warning", BadgeVariant::Warning),
            ("Info", BadgeVariant::Info),
        ] {
            Badge::new(name).variant(v).dot().show(ui);
            ui.add_space(core::SPACE_2);
        }
    });
    subhead(ui, "Sizes (Sm / Md / Lg)");
    ui.horizontal(|ui| {
        Badge::new("Small").sm().show(ui);
        ui.add_space(core::SPACE_2);
        Badge::new("Medium").show(ui);
        ui.add_space(core::SPACE_2);
        Badge::new("Large").lg().show(ui);
    });
}

fn page_spinner(ui: &mut Ui, theme: &Theme) {
    caption(ui, "Sizes + color");
    ui.horizontal(|ui| {
        Spinner::new().sm().show(ui);
        ui.add_space(core::SPACE_4);
        Spinner::new().show(ui);
        ui.add_space(core::SPACE_4);
        Spinner::new().lg().show(ui);
        ui.add_space(core::SPACE_4);
        Spinner::new().lg().color(theme.primary).show(ui);
    });
}

fn page_avatar(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Sizes");
    ui.horizontal(|ui| {
        Avatar::new("ab").size(AvatarSize::Sm).show(ui);
        ui.add_space(core::SPACE_3);
        Avatar::new("cd").show(ui);
        ui.add_space(core::SPACE_3);
        Avatar::new("ef").size(AvatarSize::Lg).show(ui);
    });
}

fn page_tooltip(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Hover the button");
    let resp = Button::new("Hover me")
        .secondary()
        .id_source("tt_btn")
        .show(ui);
    Tooltip::new("A token-styled tooltip").show(resp);
}

fn page_checkbox(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "States");
    let id = egui::Id::new("demo_check_a");
    let mut a = ui.data(|d| d.get_temp::<bool>(id).unwrap_or(true));
    Checkbox::new(&mut a).label("Accept terms").show(ui);
    ui.data_mut(|d| d.insert_temp(id, a));
    ui.add_space(core::SPACE_2);
    let mut off = false;
    Checkbox::new(&mut off).label("Unchecked").show(ui);
    ui.add_space(core::SPACE_2);
    let mut on = true;
    Checkbox::new(&mut on)
        .label("Disabled checked")
        .disabled()
        .show(ui);
    let mut un = false;
    Checkbox::new(&mut un).label("Disabled").disabled().show(ui);
    subhead(ui, "Indeterminate (mixed)");
    let mut mixed = false;
    Checkbox::new(&mut mixed)
        .label("Some selected")
        .indeterminate(true)
        .id_source("cb_ind")
        .show(ui);
    subhead(ui, "Sizes (Sm / Md / Lg)");
    ui.horizontal(|ui| {
        let mut s = true;
        Checkbox::new(&mut s).sm().id_source("cb_sm").show(ui);
        ui.add_space(core::SPACE_3);
        let mut m = true;
        Checkbox::new(&mut m).id_source("cb_md").show(ui);
        ui.add_space(core::SPACE_3);
        let mut l = true;
        Checkbox::new(&mut l).lg().id_source("cb_lg").show(ui);
    });
}

fn page_radio(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Single-select group (consumer-managed)");
    let id = egui::Id::new("demo_radio");
    let mut sel = ui.data(|d| d.get_temp::<usize>(id).unwrap_or(0));
    for (i, label) in ["Option A", "Option B", "Option C"].iter().enumerate() {
        if Radio::new(sel == i)
            .label(*label)
            .id_source(("r", i))
            .show(ui)
            .clicked()
        {
            sel = i;
        }
        ui.add_space(core::SPACE_1);
    }
    ui.data_mut(|d| d.insert_temp(id, sel));
    ui.add_space(core::SPACE_2);
    Radio::new(true)
        .label("Disabled selected")
        .disabled()
        .show(ui);
    Radio::new(false).label("Disabled").disabled().show(ui);
    subhead(ui, "Sizes (Sm / Md / Lg)");
    ui.horizontal(|ui| {
        Radio::new(true).sm().id_source("rd_sm").show(ui);
        ui.add_space(core::SPACE_3);
        Radio::new(true).id_source("rd_md").show(ui);
        ui.add_space(core::SPACE_3);
        Radio::new(true).lg().id_source("rd_lg").show(ui);
    });
}

fn page_colors(ui: &mut Ui, theme: &Theme) {
    caption(ui, "Semantic — surfaces & text");
    swatch_row(
        ui,
        &[
            ("background", theme.background),
            ("card", theme.card),
            ("popover", theme.popover),
            ("muted", theme.muted),
            ("border", theme.border),
            ("border_strong", theme.border_strong),
        ],
        theme,
    );
    swatch_row(
        ui,
        &[
            ("foreground", theme.foreground),
            ("muted_foreground", theme.muted_foreground),
            ("disabled_fg", theme.disabled_foreground),
            ("primary", theme.primary),
            ("ring", theme.ring),
        ],
        theme,
    );
    subhead(ui, "Core — zinc ramp");
    swatch_row(
        ui,
        &[
            ("50", core::ZINC_50),
            ("100", core::ZINC_100),
            ("200", core::ZINC_200),
            ("300", core::ZINC_300),
            ("400", core::ZINC_400),
            ("500", core::ZINC_500),
            ("600", core::ZINC_600),
            ("700", core::ZINC_700),
            ("800", core::ZINC_800),
            ("900", core::ZINC_900),
            ("950", core::ZINC_950),
        ],
        theme,
    );
    subhead(ui, "Status");
    swatch_row(
        ui,
        &[
            ("success", theme.success),
            ("warning", theme.warning),
            ("error", theme.error),
            ("info", theme.info),
            ("neutral", theme.neutral),
        ],
        theme,
    );
}

fn page_typography(ui: &mut Ui, _theme: &Theme) {
    const SAMPLE: &str = "Ouroboros — the serpent renews";
    type_row(ui, "display", |ui| {
        Heading::new(SAMPLE).display().show(ui);
    });
    type_row(ui, "h1", |ui| {
        Heading::new(SAMPLE).h1().show(ui);
    });
    type_row(ui, "h2", |ui| {
        Heading::new(SAMPLE).h2().show(ui);
    });
    type_row(ui, "heading", |ui| {
        Heading::new(SAMPLE).heading().show(ui);
    });
    for (name, role) in [
        ("body", TextRole::Body),
        ("body_strong", TextRole::BodyStrong),
        ("label", TextRole::Label),
        ("caption", TextRole::Caption),
        ("code", TextRole::Code),
        ("kbd", TextRole::Kbd),
    ] {
        type_row(ui, name, move |ui| {
            Text::new(SAMPLE).role(role).show(ui);
        });
    }
}

fn page_spacing(ui: &mut Ui, theme: &Theme) {
    for (name, w) in [
        ("SPACE_1", core::SPACE_1),
        ("SPACE_2", core::SPACE_2),
        ("SPACE_3", core::SPACE_3),
        ("SPACE_4", core::SPACE_4),
        ("SPACE_5", core::SPACE_5),
        ("SPACE_6", core::SPACE_6),
        ("SPACE_8", core::SPACE_8),
        ("SPACE_10", core::SPACE_10),
        ("SPACE_12", core::SPACE_12),
    ] {
        ui.horizontal(|ui| {
            name_cell(ui, name);
            let (rect, _) = ui.allocate_exact_size(vec2(w, core::SPACE_3), Sense::hover());
            ui.painter()
                .rect_filled(rect, CornerRadius::same(2), theme.primary);
            Text::new(format!("{w}")).caption().muted().show(ui);
        });
    }
}

fn page_radius(ui: &mut Ui, theme: &Theme) {
    ui.horizontal(|ui| {
        for (name, r) in [
            ("SM 4", core::RADIUS_SM),
            ("MD 6", core::RADIUS_MD),
            ("LG 8", core::RADIUS_LG),
            ("XL 12", core::RADIUS_XL),
        ] {
            ui.vertical(|ui| {
                let (rect, _) = ui.allocate_exact_size(vec2(56.0, 56.0), Sense::hover());
                ui.painter()
                    .rect_filled(rect, CornerRadius::same(r as u8), theme.muted);
                ui.painter().rect_stroke(
                    rect,
                    CornerRadius::same(r as u8),
                    Stroke::new(core::BORDER_THIN, theme.border_strong),
                    StrokeKind::Inside,
                );
                ui.add_space(core::SPACE_1);
                Text::new(name).caption().muted().show(ui);
            });
            ui.add_space(core::SPACE_4);
        }
    });
}

fn page_shadows(ui: &mut Ui, _theme: &Theme) {
    caption(
        ui,
        "shown on a light panel — dark-theme shadows are subtle by design",
    );
    ui.add_space(core::SPACE_2);
    egui::Frame::default()
        .fill(core::ZINC_200)
        .corner_radius(CornerRadius::same(core::RADIUS_LG as u8))
        .inner_margin(core::SPACE_6)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                for (name, shadow) in [
                    ("sm", core::SHADOW_SM),
                    ("md", core::SHADOW_MD),
                    ("lg", core::SHADOW_LG),
                    // Parameterized builder — custom offset/blur/spread beyond the fixed scale.
                    (
                        "custom",
                        core::shadow([4, 6], 16, 2, egui::Color32::from_black_alpha(110)),
                    ),
                ] {
                    egui::Frame::default()
                        .fill(core::ZINC_50)
                        .corner_radius(CornerRadius::same(core::RADIUS_LG as u8))
                        .shadow(shadow)
                        .inner_margin(core::SPACE_4)
                        .show(ui, |ui| {
                            Text::new(format!("shadow {name}"))
                                .label()
                                .color(core::ZINC_900)
                                .show(ui);
                        });
                    ui.add_space(core::SPACE_8);
                }
            });
        });
}

fn page_sizing(ui: &mut Ui, theme: &Theme) {
    caption(ui, "Control heights");
    ui.horizontal(|ui| {
        for (name, h) in [
            ("SM 26", core::CONTROL_SM),
            ("MD 32", core::CONTROL_MD),
            ("LG 38", core::CONTROL_LG),
        ] {
            let (rect, _) = ui.allocate_exact_size(vec2(96.0, h), Sense::hover());
            ui.painter()
                .rect_filled(rect, CornerRadius::same(core::RADIUS_MD as u8), theme.muted);
            ui.painter().rect_stroke(
                rect,
                CornerRadius::same(core::RADIUS_MD as u8),
                Stroke::new(core::BORDER_THIN, theme.border_strong),
                StrokeKind::Inside,
            );
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                name,
                typography::caption().font_id(),
                theme.muted_foreground,
            );
            ui.add_space(core::SPACE_3);
        }
    });
    subhead(ui, "Icon sizes");
    ui.horizontal(|ui| {
        for (sz, lbl) in [
            (core::ICON_SM, "14"),
            (core::ICON_MD, "16"),
            (core::ICON_LG, "20"),
            (core::ICON_XL, "24"),
        ] {
            ui.vertical(|ui| {
                Icon::new(light::CUBE).size(sz).show(ui);
                Text::new(lbl).caption().muted().show(ui);
            });
            ui.add_space(core::SPACE_4);
        }
    });
}

fn page_opacity(ui: &mut Ui, theme: &Theme) {
    ui.horizontal(|ui| {
        Text::new("enabled").show(ui);
        ui.add_space(core::SPACE_4);
        Text::new("disabled 0.5")
            .color(theme.foreground.gamma_multiply(core::OPACITY_DISABLED))
            .show(ui);
    });
    ui.add_space(core::SPACE_3);
    ui.horizontal(|ui| {
        overlay_tile(ui, "scrim 0.6", core::ZINC_200, core::SCRIM);
        ui.add_space(core::SPACE_4);
        overlay_tile(ui, "hover", theme.muted, theme.hover_overlay);
        ui.add_space(core::SPACE_4);
        overlay_tile(ui, "press", theme.muted, theme.press_overlay);
    });
}

fn page_motion(ui: &mut Ui, theme: &Theme) {
    let t = ui.input(|i| i.time) as f32;
    for (name, dur, easing) in [
        (
            "fast 0.10  EaseOut",
            core::DURATION_FAST,
            core::Easing::EaseOut,
        ),
        (
            "normal 0.18  EaseOut",
            core::DURATION_NORMAL,
            core::Easing::EaseOut,
        ),
        (
            "slow 0.30  EaseInOut",
            core::DURATION_SLOW,
            core::Easing::EaseInOut,
        ),
        (
            "slow 0.30  Spring",
            core::DURATION_SLOW,
            core::Easing::Spring,
        ),
        (
            "slow 0.30  Bounce",
            core::DURATION_SLOW,
            core::Easing::Bounce,
        ),
    ] {
        let period = dur * 2.0 + 0.6;
        let phase = (t % period) / period;
        let leg = if phase < 0.5 {
            phase * 2.0
        } else {
            1.0 - (phase - 0.5) * 2.0
        };
        let e = easing.apply(leg.clamp(0.0, 1.0));
        ui.horizontal(|ui| {
            ui.allocate_ui(vec2(180.0, 16.0), |ui| {
                Text::new(name).code().muted().show(ui);
            });
            let (track, _) = ui.allocate_exact_size(vec2(240.0, 16.0), Sense::hover());
            ui.painter()
                .rect_filled(track, CornerRadius::same(8), theme.muted);
            let r = 6.0;
            let x = track.left() + r + e * (track.width() - 2.0 * r);
            ui.painter()
                .circle_filled(egui::pos2(x, track.center().y), r, theme.primary);
        });
    }
    ui.ctx().request_repaint();
}

fn page_layout_tokens(ui: &mut Ui, theme: &Theme) {
    caption(ui, "Panel widths");
    for (name, w) in [
        ("SIDEBAR 240", layout::SIDEBAR_WIDTH),
        ("INSPECTOR 300", layout::INSPECTOR_WIDTH),
        ("PANEL_MIN 180", layout::PANEL_MIN),
        ("PANEL_MAX 480", layout::PANEL_MAX),
    ] {
        ui.horizontal(|ui| {
            let (rect, _) = ui.allocate_exact_size(vec2(w, core::SPACE_3), Sense::hover());
            ui.painter()
                .rect_filled(rect, CornerRadius::same(2), theme.border_strong);
            Text::new(name).code().muted().show(ui);
        });
    }
    subhead(ui, "Breakpoints");
    Text::new("compact <720  ·  normal <1024  ·  wide ≥1440")
        .code()
        .muted()
        .show(ui);
}

fn page_auto_layout(ui: &mut Ui, theme: &Theme) {
    let (p, pf) = (theme.primary, theme.primary_foreground);
    let (mu, fg) = (theme.muted, theme.foreground);

    caption(ui, "gap Auto — space-between");
    al_box(ui, theme, |ui| {
        AutoLayout::horizontal()
            .gap_auto()
            .pad(core::SPACE_2)
            .cross_align(CrossAlign::Center)
            .hug(|ui| chip(ui, "A", p, pf))
            .hug(|ui| chip(ui, "B", mu, fg))
            .hug(|ui| chip(ui, "C", mu, fg))
            .show(ui);
    });
    caption(ui, "Fill spacer — left group · spacer · right action");
    al_box(ui, theme, |ui| {
        AutoLayout::horizontal()
            .gap(core::SPACE_2)
            .pad(core::SPACE_2)
            .cross_align(CrossAlign::Center)
            .hug(|ui| chip(ui, "File", mu, fg))
            .hug(|ui| chip(ui, "Edit", mu, fg))
            .fill(|_ui| {})
            .hug(|ui| chip(ui, "Run", p, pf))
            .show(ui);
    });
    caption(ui, "main_align Center");
    al_box(ui, theme, |ui| {
        AutoLayout::horizontal()
            .gap(core::SPACE_2)
            .pad(core::SPACE_2)
            .main_align(MainAlign::Center)
            .cross_align(CrossAlign::Center)
            .hug(|ui| chip(ui, "ok", p, pf))
            .hug(|ui| chip(ui, "cancel", mu, fg))
            .show(ui);
    });
    caption(ui, "vertical stack — gap + padding");
    al_box(ui, theme, |ui| {
        AutoLayout::vertical()
            .gap(core::SPACE_2)
            .pad(core::SPACE_3)
            .cross_align(CrossAlign::Start)
            .hug(|ui| chip(ui, "row one", mu, fg))
            .hug(|ui| chip(ui, "row two", mu, fg))
            .hug(|ui| chip(ui, "row three", p, pf))
            .show(ui);
    });
    caption(
        ui,
        "Fill with min/max — fill_clamped(80, 160) · fill_min(120)",
    );
    al_box(ui, theme, |ui| {
        AutoLayout::horizontal()
            .gap(core::SPACE_2)
            .pad(core::SPACE_2)
            .cross_align(CrossAlign::Center)
            .fill_clamped(80.0, 160.0, |ui| fill_chip(ui, "80–160", mu, fg))
            .fill_min(120.0, |ui| fill_chip(ui, "min 120", p, pf))
            .show(ui);
    });
    caption(
        ui,
        "wrap — 6 × fill_min(72) reflow: one row when wide, 2–3 when narrow",
    );
    al_box(ui, theme, |ui| {
        let mut grid = AutoLayout::horizontal()
            .wrap()
            .gap(core::SPACE_2)
            .pad(core::SPACE_2);
        for label in ["STR", "AGI", "VIT", "INT", "DEX", "LUK"] {
            grid = grid.fill_min(72.0, move |ui| fill_chip(ui, label, mu, fg));
        }
        grid.show(ui);
    });
}

fn page_resize_lab(ui: &mut Ui, _theme: &Theme) {
    Text::new(
        "Responsive layout stress area — drag the divider and watch the left panel reflow \
         without overlap, clipping ratchets, or collapsed columns.",
    )
    .muted()
    .wrap()
    .show(ui);
    // Live width readouts so failure ranges can be reported precisely ("breaks below N px").
    Text::new(format!("window: {:.0} px", ui.ctx().content_rect().width()))
        .muted()
        .show(ui);
    ui.add_space(core::SPACE_3);

    // Interactive state lives in egui temp memory (same pattern as the other pages).
    let id_name = egui::Id::new("rlab_name");
    let id_mass = egui::Id::new("rlab_mass");
    let id_blend = egui::Id::new("rlab_blend");
    let id_drag = egui::Id::new("rlab_drag");
    let id_stats = egui::Id::new("rlab_stats");
    let id_cols = egui::Id::new("rlab_cols");
    let mut name = ui.data(|d| d.get_temp::<String>(id_name).unwrap_or_default());
    let mut mass = ui.data(|d| d.get_temp::<f32>(id_mass).unwrap_or(72.5));
    let mut blend = ui.data(|d| d.get_temp::<usize>(id_blend).unwrap_or(0));
    let mut drag = ui.data(|d| d.get_temp::<f32>(id_drag).unwrap_or(0.05));
    let mut stats = ui.data(|d| {
        d.get_temp::<[f32; 6]>(id_stats)
            .unwrap_or([12.0, 9.0, 14.0, 7.0, 11.0, 5.0])
    });
    let mut cols = ui.data(|d| {
        d.get_temp::<[f32; 4]>(id_cols)
            .unwrap_or([0.6, 0.25, 0.4, 0.8])
    });

    ui.allocate_ui(vec2(ui.available_width(), 460.0), |ui| {
        Splitter::horizontal()
            .id_source("rlab_split")
            .panel(PanelSpec::new().size(0.35).min(180.0).max(520.0), |ui| {
                let panel_w = ui.available_width();
                Text::new(format!("◂ panel: {panel_w:.0} px ▸")).show(ui);
                ui.add_space(core::SPACE_1);
                egui::ScrollArea::vertical()
                    .id_salt("rlab_scroll")
                    .auto_shrink([true, false])
                    .show(ui, |ui| {
                        resize_lab_panel(
                            ui, &mut name, &mut mass, &mut blend, &mut drag, &mut stats, &mut cols,
                        );
                    });
            })
            .panel(PanelSpec::flex(), |ui| {
                let flex_w = ui.available_width();
                Surface::new().muted().show(ui, |ui| {
                    ui.set_min_size(ui.available_size());
                    Text::new(format!("flex area: {flex_w:.0} px")).show(ui);
                    Text::new("shrink the window to stress the left panel")
                        .muted()
                        .wrap()
                        .show(ui);
                });
            })
            .show(ui);
    });

    ui.data_mut(|d| {
        d.insert_temp(id_name, name);
        d.insert_temp(id_mass, mass);
        d.insert_temp(id_blend, blend);
        d.insert_temp(id_drag, drag);
        d.insert_temp(id_stats, stats);
        d.insert_temp(id_cols, cols);
    });
}

/// The torture stack inside the Resize Lab's left panel — every responsive failure mode
/// in one column: stretching controls, a wrapping status band, a reflowing stat grid,
/// wrapping prose, and two side-by-side columns that must not collapse.
fn resize_lab_panel(
    ui: &mut Ui,
    name: &mut String,
    mass: &mut f32,
    blend: &mut usize,
    drag: &mut f32,
    stats: &mut [f32; 6],
    cols: &mut [f32; 4],
) {
    ui.add_space(core::SPACE_2);

    // a) Inspector rows — fixed label column, controls absorb the remaining width.
    Text::new("PROPERTIES").caption().muted().show(ui);
    ui.add_space(core::SPACE_1);
    PropertyRow::new("Name").show(ui, |ui| {
        Input::new(name).placeholder("Entity name…").show(ui)
    });
    PropertyRow::new("Mass").show(ui, |ui| {
        NumericField::new(mass).speed(0.05).suffix(" kg").show(ui)
    });
    PropertyRow::new("Blend").show(ui, |ui| {
        Select::new(blend)
            .options(["Opaque", "Cutout", "Transparent", "Additive"])
            .show(ui)
    });
    PropertyRow::new("Drag").show(ui, |ui| {
        NumericField::new(drag).speed(0.01).decimals(2).show(ui)
    });

    // b) Status band — the alert fills (and wraps its long message), the action hugs.
    subhead(ui, "Status band — Fill alert · Hug action");
    AutoLayout::horizontal()
        .gap(core::SPACE_2)
        .cross_align(CrossAlign::Center)
        .fill(|ui| {
            Alert::new(
                "Autosave recovered three unsaved changes from the previous session — \
                 review them before publishing; a narrow panel must wrap this message \
                 instead of pushing the action button out of view.",
            )
            .variant(AlertVariant::Warning)
            .show(ui);
        })
        .hug(|ui| {
            Button::new("Action")
                .sm()
                .id_source("rlab_band_btn")
                .show(ui);
        })
        .show(ui);

    // c) Stat grid — wrap + fill_min: one row when wide, reflows to 2–3 lines when narrow.
    subhead(ui, "Stat grid — wrap + fill_min(72)");
    let mut grid = AutoLayout::horizontal()
        .wrap()
        .gap(core::SPACE_2)
        .gap_cross(core::SPACE_2);
    for (label, value) in ["STR", "AGI", "VIT", "INT", "DEX", "LUK"]
        .into_iter()
        .zip(stats.iter_mut())
    {
        grid = grid.fill_min(72.0, move |ui| {
            Surface::new().muted().show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                Text::new(label).caption().muted().show(ui);
                NumericField::new(&mut *value).speed(0.1).show(ui);
            });
        });
    }
    grid.show(ui);

    // d) Long prose — text wrap under a shrinking budget.
    subhead(ui, "Long text");
    Text::new(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor \
         incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis \
         nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. \
         Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu \
         fugiat nulla pariatur.",
    )
    .muted()
    .wrap()
    .show(ui);

    // e) Two responsive columns — each floors at 140px instead of collapsing.
    subhead(ui, "Two columns — fill_min(140) each");
    let (ca, cb) = cols.split_at_mut(2);
    AutoLayout::horizontal()
        .gap(core::SPACE_4)
        .fill_min(140.0, move |ui| {
            PropertyRow::new("Bounce")
                .show(ui, |ui| NumericField::new(&mut ca[0]).speed(0.01).show(ui));
            PropertyRow::new("Friction")
                .show(ui, |ui| NumericField::new(&mut ca[1]).speed(0.01).show(ui));
        })
        .fill_min(140.0, move |ui| {
            PropertyRow::new("Damping")
                .show(ui, |ui| NumericField::new(&mut cb[0]).speed(0.01).show(ui));
            PropertyRow::new("Restitution")
                .show(ui, |ui| NumericField::new(&mut cb[1]).speed(0.01).show(ui));
        })
        .show(ui);
    ui.add_space(core::SPACE_4);
}

fn page_text(ui: &mut Ui, theme: &Theme) {
    for (name, role) in [
        ("body", TextRole::Body),
        ("body_strong", TextRole::BodyStrong),
        ("label", TextRole::Label),
        ("caption", TextRole::Caption),
        ("code", TextRole::Code),
        ("kbd", TextRole::Kbd),
    ] {
        ui.horizontal(|ui| {
            name_cell(ui, name);
            Text::new("Ouroboros — the serpent renews")
                .role(role)
                .show(ui);
        });
    }
    subhead(ui, "Color & decoration");
    ui.horizontal(|ui| {
        Text::new("muted").muted().show(ui);
        ui.add_space(core::SPACE_4);
        Text::new("success token").color(theme.success).show(ui);
        ui.add_space(core::SPACE_4);
        Text::new("underlined").underline().show(ui);
        ui.add_space(core::SPACE_4);
        Text::new("italic").italic().show(ui);
        ui.add_space(core::SPACE_4);
        Text::new("muted italic aside").muted().italic().show(ui);
    });
}

fn page_heading(ui: &mut Ui, _theme: &Theme) {
    Heading::new("Display").display().show(ui);
    ui.add_space(core::SPACE_2);
    Heading::new("Heading 1").h1().show(ui);
    ui.add_space(core::SPACE_2);
    Heading::new("Heading 2").h2().show(ui);
    ui.add_space(core::SPACE_2);
    Heading::new("Heading").heading().show(ui);
}

fn page_icon(ui: &mut Ui, theme: &Theme) {
    caption(ui, "Sizes (sm/md/lg/xl) + color");
    ui.horizontal(|ui| {
        Icon::new(light::CUBE).sm().show(ui);
        ui.add_space(core::SPACE_3);
        Icon::new(light::CUBE).md().show(ui);
        ui.add_space(core::SPACE_3);
        Icon::new(light::CUBE).lg().show(ui);
        ui.add_space(core::SPACE_3);
        Icon::new(light::CUBE).xl().show(ui);
        ui.add_space(core::SPACE_5);
        Icon::new(light::CHECK).lg().color(theme.success).show(ui);
        ui.add_space(core::SPACE_3);
        Icon::new(light::WARNING).lg().color(theme.warning).show(ui);
        ui.add_space(core::SPACE_3);
        Icon::new(light::HEART).lg().color(theme.error).show(ui);
    });
    subhead(ui, "Phosphor Light set (sample)");
    ui.horizontal_wrapped(|ui| {
        for g in [
            light::GRID_FOUR,
            light::SQUARES_FOUR,
            light::CUBE,
            light::TERMINAL,
            light::GEAR,
            light::PALETTE,
            light::MAGNIFYING_GLASS,
            light::STAR,
            light::CHECK,
            light::WARNING,
            light::INFO,
            light::LIGHTBULB,
        ] {
            Icon::new(g).lg().show(ui);
            ui.add_space(core::SPACE_3);
        }
    });
}

fn page_divider(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Horizontal");
    Divider::horizontal().show(ui);
    ui.add_space(core::SPACE_3);
    caption(ui, "Destructive (red rule)");
    Divider::horizontal().destructive().show(ui);
    ui.add_space(core::SPACE_3);
    caption(ui, "Dotted");
    Divider::horizontal().dotted().show(ui);
    ui.add_space(core::SPACE_3);
    caption(ui, "Vertical (between content)");
    ui.allocate_ui(vec2(ui.available_width(), 22.0), |ui| {
        ui.horizontal(|ui| {
            Text::new("File").show(ui);
            ui.add_space(core::SPACE_3);
            Divider::vertical().show(ui);
            ui.add_space(core::SPACE_3);
            Text::new("Edit").show(ui);
        });
    });
}

fn page_button(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Variants × sizes (Sm/Md/Lg)");
    for (name, variant) in [
        ("Default", ButtonVariant::Default),
        ("Secondary", ButtonVariant::Secondary),
        ("Destructive", ButtonVariant::Destructive),
        ("Outline", ButtonVariant::Outline),
        ("Ghost", ButtonVariant::Ghost),
        ("Link", ButtonVariant::Link),
    ] {
        ui.horizontal(|ui| {
            name_cell(ui, name);
            Button::new("Button")
                .variant(variant)
                .sm()
                .id_source((name, "sm"))
                .show(ui);
            ui.add_space(core::SPACE_2);
            Button::new("Button")
                .variant(variant)
                .id_source((name, "md"))
                .show(ui);
            ui.add_space(core::SPACE_2);
            Button::new("Button")
                .variant(variant)
                .lg()
                .id_source((name, "lg"))
                .show(ui);
        });
        ui.add_space(core::SPACE_1);
    }
    subhead(ui, "Icons · icon-only · disabled");
    ui.horizontal(|ui| {
        Button::new("New")
            .icon_left(light::CUBE)
            .id_source("b_new")
            .show(ui);
        ui.add_space(core::SPACE_2);
        Button::new("Next")
            .icon_right(light::CHECK)
            .secondary()
            .id_source("b_next")
            .show(ui);
        ui.add_space(core::SPACE_2);
        Button::new("")
            .icon_only()
            .icon_left(light::GEAR)
            .outline()
            .id_source("b_gear")
            .show(ui);
        ui.add_space(core::SPACE_2);
        Button::new("Delete")
            .destructive()
            .icon_left(light::WARNING)
            .id_source("b_del")
            .show(ui);
        ui.add_space(core::SPACE_2);
        Button::new("Disabled")
            .disabled()
            .id_source("b_dis")
            .show(ui);
    });
    subhead(ui, "Loading (width preserved, clicks ignored)");
    ui.horizontal(|ui| {
        Button::new("Saving")
            .loading(true)
            .id_source("b_load_primary")
            .show(ui);
        ui.add_space(core::SPACE_2);
        Button::new("Loading")
            .secondary()
            .loading(true)
            .id_source("b_load_secondary")
            .show(ui);
        ui.add_space(core::SPACE_2);
        Button::new("")
            .icon_only()
            .icon_left(light::GEAR)
            .outline()
            .loading(true)
            .id_source("b_load_icon")
            .show(ui);
    });
}

// ── Helpers (text via atoms) ──────────────────────────────────────────────────

/// A muted caption line.
fn caption(ui: &mut Ui, text: &str) {
    Text::new(text).caption().muted().show(ui);
}

/// A subsection label with space above.
fn subhead(ui: &mut Ui, text: &str) {
    ui.add_space(core::SPACE_5);
    Text::new(text).label().show(ui);
    ui.add_space(core::SPACE_2);
}

/// A fixed-width mono name column (for `name | demo` rows).
fn name_cell(ui: &mut Ui, name: &str) {
    ui.allocate_ui(vec2(112.0, core::TEXT_LG), |ui| {
        Text::new(name).code().muted().show(ui);
    });
}

/// A `name | rendered` typography row.
fn type_row(ui: &mut Ui, name: &str, render: impl FnOnce(&mut Ui)) {
    ui.horizontal(|ui| {
        ui.allocate_ui(vec2(120.0, core::TEXT_3XL), |ui| {
            Text::new(name).code().muted().show(ui);
        });
        render(ui);
    });
    ui.add_space(core::SPACE_1);
}

fn al_box(ui: &mut Ui, theme: &Theme, add: impl FnOnce(&mut Ui)) {
    egui::Frame::default()
        .stroke(Stroke::new(core::BORDER_THIN, theme.border))
        .corner_radius(CornerRadius::same(core::RADIUS_MD as u8))
        .inner_margin(core::SPACE_1)
        .show(ui, |ui| {
            ui.set_width(ui.available_width().min(420.0));
            add(ui);
        });
    ui.add_space(core::SPACE_3);
}

/// A pill that spans the cell's width — visualizes the *resolved* size of `Fill` cells
/// in the auto-layout demos (a hugging [`chip`] would hide the distribution).
fn fill_chip(ui: &mut Ui, label: &str, fill: Color32, fg: Color32) {
    let pad = vec2(core::SPACE_3, core::SPACE_2);
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_owned(), typography::label().font_id(), fg);
    let size = vec2(ui.available_width(), galley.size().y + pad.y * 2.0);
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter()
        .rect_filled(rect, CornerRadius::same(core::RADIUS_MD as u8), fill);
    let text_pos = egui::pos2(rect.center().x - galley.size().x * 0.5, rect.min.y + pad.y);
    ui.painter().galley(text_pos, galley, fg);
}

/// A content-sized pill — a stand-in child for the auto-layout demos.
fn chip(ui: &mut Ui, label: &str, fill: Color32, fg: Color32) {
    let pad = vec2(core::SPACE_3, core::SPACE_2);
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_owned(), typography::label().font_id(), fg);
    let size = galley.size() + pad * 2.0;
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter()
        .rect_filled(rect, CornerRadius::same(core::RADIUS_MD as u8), fill);
    ui.painter().galley(rect.min + pad, galley, fg);
}

fn hex(c: Color32) -> String {
    format!("#{:02x}{:02x}{:02x}", c.r(), c.g(), c.b())
}

fn overlay_tile(ui: &mut Ui, label: &str, base: Color32, overlay: Color32) {
    ui.vertical(|ui| {
        let (rect, _) = ui.allocate_exact_size(vec2(104.0, 44.0), Sense::hover());
        let cr = CornerRadius::same(core::RADIUS_MD as u8);
        ui.painter().rect_filled(rect, cr, base);
        ui.painter().rect_filled(rect, cr, overlay);
        ui.add_space(core::SPACE_1);
        Text::new(label).caption().color(core::ZINC_400).show(ui);
    });
}

fn swatch_row(ui: &mut Ui, items: &[(&str, Color32)], theme: &Theme) {
    ui.horizontal_wrapped(|ui| {
        for (name, color) in items {
            ui.vertical(|ui| {
                let (rect, _) = ui.allocate_exact_size(vec2(104.0, 44.0), Sense::hover());
                ui.painter()
                    .rect_filled(rect, CornerRadius::same(core::RADIUS_MD as u8), *color);
                ui.painter().rect_stroke(
                    rect,
                    CornerRadius::same(core::RADIUS_MD as u8),
                    Stroke::new(core::BORDER_THIN, theme.border),
                    StrokeKind::Inside,
                );
                ui.add_space(core::SPACE_1);
                Text::new(*name).caption().show(ui);
                Text::new(hex(*color)).code().muted().show(ui);
            });
            ui.add_space(core::SPACE_3);
        }
    });
}
