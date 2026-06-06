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
    ListItem, MenuItem, PropertyRow, TableCell, TableRow, ToolbarButton, TreeNode,
};
use ouroboros_ui::molecules::{
    Alert, AlertVariant, Breadcrumb, Card, CheckboxCard, Collapsible, ColorField, Field,
    FieldSeparator, FieldSet, InputGroup, RadioCard, RadioGroup, SearchField, Slot, Tabs,
    ToggleGroup, VectorField,
};
use ouroboros_ui::organisms::{
    Accordion, Dialog, DropdownMenu, Menubar, Popover, Select, Sidebar, TabView, Table, Toast,
    Toolbar, TreeItem, TreeView,
};
use ouroboros_ui::theme::typography;
use ouroboros_ui::tokens::{core, layout};
use ouroboros_ui::{Mode, Theme};

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
    ("LAYOUT", &[Page::LayoutTokens, Page::AutoLayoutDemo]),
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
    }
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
        "Framed table — muted header + zebra rows (legacy style)",
    );
    ui.allocate_ui(vec2(380.0, 200.0), |ui| {
        Table::new()
            .headers(["Name", "Type", "Size"])
            .row(["hero.fbx", "Mesh", "2.1 MB"])
            .row(["grass.png", "Texture", "512 KB"])
            .row(["main.rs", "Script", "8 KB"])
            .row(["sky.hdr", "Texture", "4.0 MB"])
            .show(ui);
    });
    subhead(ui, "Rich cells — TableCell with a status dot");
    let mut row = |a: &str, b: &str, status: Option<egui::Color32>, header: bool| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            let mut c0 = TableCell::new(a);
            let mut c1 = TableCell::new(b);
            if header {
                c0 = c0.header();
                c1 = c1.header();
            }
            if let Some(s) = status {
                c1 = c1.status(s);
            }
            c0.show(ui, 120.0);
            c1.show(ui, 110.0);
        });
    };
    row("Creature", "State", None, true);
    row("Goblin", "Alive", Some(theme.success), false);
    row("Skeleton", "Broken ref", Some(theme.error), false);
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
    if show {
        Toast::new("Build finished in 2.3s")
            .success()
            .show(ui.ctx());
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

fn page_table_row(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Table rows (fixed columns)");
    let widths = [120.0_f32, 80.0, 80.0];
    ui.allocate_ui(vec2(300.0, 140.0), |ui| {
        TableRow::new(["Name", "Type", "Size"])
            .header()
            .show(ui, &widths);
        ui.add_space(core::SPACE_1);
        Divider::horizontal().show(ui);
        ui.add_space(core::SPACE_1);
        for row in [
            ["hero.fbx", "Mesh", "2.1 MB"],
            ["grass.png", "Texture", "512 KB"],
            ["main.rs", "Script", "8 KB"],
        ] {
            TableRow::new(row).show(ui, &widths);
        }
    });
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
    caption(ui, "Swatch + hex (picker popover later)");
    ui.horizontal(|ui| {
        ColorField::new(core::BLUE_500).show(ui);
    });
    ui.add_space(core::SPACE_2);
    ColorField::new(core::AMBER_500).show(ui);
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
