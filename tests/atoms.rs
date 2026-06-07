//! Smoke / interaction tests for atoms (egui_kittest).
//!
//! The shared [`rendered`] helper installs the theme fonts first — the named Iosevka
//! families must exist before any atom (which uses a type style) paints, so we run one
//! install frame and then a paint frame.

use egui::Ui;
use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;
use ouroboros_ui::atoms::{
    Avatar, Badge, BadgeVariant, Button, Checkbox, ColorSwatch, Divider, Heading, HeadingLevel,
    Icon, Input, Kbd, NumericField, Progress, Radio, Skeleton, Slider, Spinner, Surface, Switch,
    Text, TextRole, Textarea, Toggle, Tooltip,
};
use ouroboros_ui::cells::{
    ListItem, MenuItem, PropertyRow, TableCell, TableRow, ToolbarButton, TreeNode,
};
use ouroboros_ui::egui_phosphor::light;
use ouroboros_ui::molecules::{
    Alert, Breadcrumb, Card, CheckboxCard, Collapsible, ColorField, Field, FieldSeparator,
    FieldSet, InputGroup, RadioGroup, SearchField, Slot, Tabs, ToggleGroup, VectorField,
};
use ouroboros_ui::organisms::{
    Accordion, Column, Menubar, PanelSpec, Select, Sidebar, Splitter, TabView, Table, Toolbar,
    TreeItem, TreeView,
};
use ouroboros_ui::tokens::core;
use ouroboros_ui::{Mode, Size, Theme};
use std::cell::Cell;
use std::rc::Rc;

/// Render `content` in a harness with the theme/fonts installed.
fn rendered(mut content: impl FnMut(&mut Ui) + 'static) {
    let mut installed = false;
    let mut harness = Harness::new_ui(move |ui| {
        if !installed {
            Theme::install(ui.ctx(), Mode::Dark);
            installed = true;
            return;
        }
        content(ui);
    });
    harness.run(); // install fonts (skips render)
    harness.run(); // paint with fonts available
}

/// Like [`rendered`] but uses fixed `step()`s — for content that repaints every frame (e.g. a
/// loading spinner), where `run()` would exceed max_steps waiting for stability.
fn rendered_stepped(mut content: impl FnMut(&mut Ui) + 'static) {
    let mut installed = false;
    let mut harness = Harness::new_ui(move |ui| {
        if !installed {
            Theme::install(ui.ctx(), Mode::Dark);
            installed = true;
            return;
        }
        content(ui);
    });
    harness.step(); // install frame
    harness.step(); // render frame
    harness.step();
}

#[test]
fn text_renders_all_roles() {
    rendered(|ui| {
        for role in [
            TextRole::Body,
            TextRole::BodyStrong,
            TextRole::Label,
            TextRole::Caption,
            TextRole::Code,
            TextRole::Kbd,
        ] {
            Text::new("sample").role(role).show(ui);
        }
        Text::new("muted").muted().show(ui);
    });
}

#[test]
fn heading_renders_all_levels() {
    rendered(|ui| {
        for level in [
            HeadingLevel::Display,
            HeadingLevel::H1,
            HeadingLevel::H2,
            HeadingLevel::Heading,
        ] {
            Heading::new("Title").level(level).show(ui);
        }
    });
}

#[test]
fn icon_renders_at_sizes() {
    rendered(|ui| {
        Icon::new(light::GEAR).sm().show(ui);
        Icon::new(light::CUBE).md().show(ui);
        Icon::new(light::STAR).lg().show(ui);
        Icon::new(light::HEART).xl().show(ui);
    });
}

#[test]
fn divider_renders_both_axes() {
    rendered(|ui| {
        Divider::horizontal().show(ui);
        Divider::horizontal().destructive().show(ui);
        ui.horizontal(|ui| {
            Divider::vertical().show(ui);
        });
    });
}

#[test]
fn button_click_fires() {
    let clicked = Rc::new(Cell::new(false));
    let sink = clicked.clone();
    let mut installed = false;
    let mut harness = Harness::new_ui(move |ui| {
        if !installed {
            Theme::install(ui.ctx(), Mode::Dark);
            installed = true;
            return;
        }
        if Button::new("Save").show(ui).clicked() {
            sink.set(true);
        }
    });
    harness.run();
    harness.run();
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "Save")
        .click_accesskit();
    harness.run();
    assert!(clicked.get(), "enabled button click should fire");
}

#[test]
fn checkbox_toggles() {
    let state = Rc::new(Cell::new(false));
    let sink = state.clone();
    let mut installed = false;
    let mut harness = Harness::new_ui(move |ui| {
        if !installed {
            Theme::install(ui.ctx(), Mode::Dark);
            installed = true;
            return;
        }
        let mut v = sink.get();
        Checkbox::new(&mut v).label("Accept").show(ui);
        sink.set(v);
    });
    harness.run();
    harness.run();
    harness
        .get_by_role_and_label(egui::accesskit::Role::CheckBox, "Accept")
        .click_accesskit();
    harness.run();
    assert!(state.get(), "checkbox should toggle on");
}

#[test]
fn radio_click_fires() {
    let clicked = Rc::new(Cell::new(false));
    let sink = clicked.clone();
    let mut installed = false;
    let mut harness = Harness::new_ui(move |ui| {
        if !installed {
            Theme::install(ui.ctx(), Mode::Dark);
            installed = true;
            return;
        }
        if Radio::new(false).label("Option A").show(ui).clicked() {
            sink.set(true);
        }
    });
    harness.run();
    harness.run();
    harness
        .get_by_role_and_label(egui::accesskit::Role::RadioButton, "Option A")
        .click_accesskit();
    harness.run();
    assert!(clicked.get(), "radio click should fire");
}

#[test]
fn switch_toggles() {
    let state = Rc::new(Cell::new(false));
    let sink = state.clone();
    let mut installed = false;
    let mut harness = Harness::new_ui(move |ui| {
        if !installed {
            Theme::install(ui.ctx(), Mode::Dark);
            installed = true;
            return;
        }
        let mut v = sink.get();
        Switch::new(&mut v).show(ui);
        sink.set(v);
    });
    harness.run();
    harness.run();
    harness
        .get_by_role(egui::accesskit::Role::CheckBox)
        .click_accesskit();
    harness.run();
    assert!(state.get(), "switch should toggle on");
}

#[test]
fn input_renders() {
    rendered(|ui| {
        let mut s = String::from("hello");
        Input::new(&mut s).placeholder("type…").show(ui);
        let mut e = String::new();
        Input::new(&mut e).error(true).show(ui);
    });
}

#[test]
fn badge_renders_all_variants() {
    rendered(|ui| {
        for v in [
            BadgeVariant::Default,
            BadgeVariant::Secondary,
            BadgeVariant::Destructive,
            BadgeVariant::Outline,
            BadgeVariant::Ghost,
            BadgeVariant::Link,
            BadgeVariant::Success,
            BadgeVariant::Warning,
            BadgeVariant::Info,
        ] {
            Badge::new("badge").variant(v).dot().show(ui);
        }
    });
}

#[test]
fn spinner_and_avatar_render() {
    // Spinner repaints every frame, so `run()` (which waits for stability) would exceed
    // max_steps — use `step()` for a single frame.
    let mut installed = false;
    let mut harness = Harness::new_ui(move |ui| {
        if !installed {
            Theme::install(ui.ctx(), Mode::Dark);
            installed = true;
            return;
        }
        Spinner::new().lg().show(ui);
        Avatar::new("ab").show(ui);
    });
    harness.step(); // install frame
    harness.step(); // render frame (single steps; spinner keeps repainting)
    harness.step();
}

#[test]
fn tooltip_attaches() {
    rendered(|ui| {
        let resp = Button::new("hover").show(ui);
        Tooltip::new("tip").show(resp);
    });
}

#[test]
fn engine_atoms_render() {
    rendered(|ui| {
        let mut v = 0.5_f32;
        Slider::new(&mut v).range(0.0, 1.0).step(0.1).show(ui);
        let mut n = 1.0_f32;
        NumericField::new(&mut n).speed(0.1).suffix(" m").show(ui);
        ColorSwatch::new(core::RED_500).show(ui);
        ColorSwatch::new(core::GREEN_500).circle().show(ui);
        Progress::new(0.4).show(ui);
        Kbd::new("Ctrl").show(ui);
    });
}

#[test]
fn skeleton_renders() {
    // Pulse repaints, so use step() (like spinner).
    let mut installed = false;
    let mut harness = Harness::new_ui(move |ui| {
        if !installed {
            Theme::install(ui.ctx(), Mode::Dark);
            installed = true;
            return;
        }
        Skeleton::new().width(120.0).show(ui);
    });
    harness.step();
    harness.step();
    harness.step();
}

#[test]
fn toggle_toggles() {
    let state = Rc::new(Cell::new(false));
    let sink = state.clone();
    let mut installed = false;
    let mut harness = Harness::new_ui(move |ui| {
        if !installed {
            Theme::install(ui.ctx(), Mode::Dark);
            installed = true;
            return;
        }
        let mut v = sink.get();
        Toggle::new(&mut v).label("Bold").show(ui);
        sink.set(v);
    });
    harness.run();
    harness.run();
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "Bold")
        .click_accesskit();
    harness.run();
    assert!(state.get(), "toggle should turn on");
}

#[test]
fn molecules_engine_render() {
    rendered(|ui| {
        let mut t = 0;
        Tabs::new(&mut t).tabs(["A", "B"]).show(ui);
        Collapsible::new("Sec").default_open(true).show(ui, |ui| {
            Text::new("body").show(ui);
        });
        Alert::new("msg").warning().title("Heads up").show(ui);
        let mut g = 0;
        ToggleGroup::new(&mut g)
            .options(["Local", "World"])
            .show(ui);
        Breadcrumb::new().items(["A", "B", "C"]).show(ui);
        let mut v = [1.0_f32, 2.0, 3.0];
        VectorField::new(&mut v).show(ui);
        let mut col = core::BLUE_500;
        ColorField::new(&mut col).show(ui);
        let mut s = String::new();
        SearchField::new(&mut s).placeholder("x").show(ui);
    });
}

#[test]
fn tabs_selects() {
    let selected = Rc::new(Cell::new(0usize));
    let sink = selected.clone();
    let mut installed = false;
    let mut harness = Harness::new_ui(move |ui| {
        if !installed {
            Theme::install(ui.ctx(), Mode::Dark);
            installed = true;
            return;
        }
        let mut v = sink.get();
        Tabs::new(&mut v).tabs(["Scene", "Game"]).show(ui);
        sink.set(v);
    });
    harness.run();
    harness.run();
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "Game")
        .click_accesskit();
    harness.run();
    assert_eq!(selected.get(), 1, "clicking the Game tab should select it");
}

#[test]
fn cells_render() {
    rendered(|ui| {
        let mut m = 1.0_f32;
        PropertyRow::new("Mass").show(ui, |ui| NumericField::new(&mut m).show(ui));
        ListItem::new("Cube")
            .icon(light::CUBE)
            .subtitle("Mesh")
            .show(ui);
        MenuItem::new("Copy")
            .icon(light::COPY)
            .shortcut("Ctrl C")
            .show(ui);
        TreeNode::new("Player")
            .depth(1)
            .icon(light::CUBE)
            .expandable(true)
            .show(ui);
        let mut active = true;
        ToolbarButton::new(&mut active, light::CURSOR)
            .tooltip("Select")
            .show(ui);
        TableCell::text("A").header().show(ui);
        TableCell::text("B").status(egui::Color32::RED).show(ui);
    });
}

#[test]
fn list_item_selects() {
    let clicked = Rc::new(Cell::new(false));
    let sink = clicked.clone();
    let mut installed = false;
    let mut harness = Harness::new_ui(move |ui| {
        if !installed {
            Theme::install(ui.ctx(), Mode::Dark);
            installed = true;
            return;
        }
        if ListItem::new("Cube").show(ui).clicked() {
            sink.set(true);
        }
    });
    harness.run();
    harness.run();
    harness.get_by_label("Cube").click();
    harness.run();
    assert!(clicked.get(), "clicking the list row should select it");
}

#[test]
fn organisms_render() {
    rendered(|ui| {
        Toolbar::new().show(ui, |ui| {
            let mut a = true;
            ToolbarButton::new(&mut a, light::CURSOR).show(ui);
        });
        let mut t = 0;
        TabView::new(&mut t).tabs(["A", "B"]).show(ui, |ui, i| {
            Text::new(format!("panel {i}")).show(ui);
        });
        Table::new()
            .columns([Column::new("N"), Column::new("T").end()])
            .rows([
                TableRow::new([TableCell::text("a"), TableCell::text("b").end()]),
                TableRow::new([TableCell::text("c"), TableCell::text("d").end()]),
            ])
            .striped(true)
            .border(true)
            .selectable(true)
            .show(ui);
        let mut s = 0;
        TreeView::new(&mut s)
            .items([
                TreeItem::new("Root").expanded(true),
                TreeItem::new("Child").depth(1),
            ])
            .show(ui);
        let mut nav = 0;
        Sidebar::new(&mut nav)
            .item(light::HOUSE, "Home")
            .text_item("Other")
            .show(ui);
    });
}

#[test]
fn new_states_render() {
    // Button loading repaints (spinner), so step instead of run.
    rendered_stepped(|ui| {
        // Button: loading + sizes + icon-only.
        Button::new("Saving").loading(true).show(ui);
        Button::new("S").size(Size::Sm).show(ui);
        Button::new("L").size(Size::Lg).show(ui);
        Button::new("").icon_only().icon_left(light::GEAR).show(ui);
        // Checkbox: indeterminate + sizes.
        let mut c = false;
        Checkbox::new(&mut c).indeterminate(true).show(ui);
        let mut c2 = true;
        Checkbox::new(&mut c2).sm().show(ui);
        // Radio + switch sizes.
        Radio::new(true).lg().show(ui);
        let mut sw = true;
        Switch::new(&mut sw).lg().show(ui);
        // Slider: disabled + size.
        let mut v = 0.5;
        Slider::new(&mut v).disabled().show(ui);
        let mut v2 = 0.5;
        Slider::new(&mut v2).sm().show(ui);
        // Numeric: error + size.
        let mut n = 1.0;
        NumericField::new(&mut n).error(true).show(ui);
        let mut n2 = 2.0;
        NumericField::new(&mut n2).lg().show(ui);
        // Input size.
        let mut s = String::new();
        Input::new(&mut s).size(Size::Sm).show(ui);
    });
}

#[test]
fn splitter_renders() {
    rendered(|ui| {
        ui.allocate_ui(egui::vec2(400.0, 240.0), |ui| {
            Splitter::horizontal()
                .id_source("test_split")
                .panel(PanelSpec::new().min(80.0).max(200.0), |ui| {
                    Text::new("left").show(ui);
                })
                .panel(PanelSpec::new(), |ui| {
                    Splitter::vertical()
                        .id_source("test_split_nested")
                        .panel(PanelSpec::new(), |ui| {
                            Text::new("top").show(ui);
                        })
                        .panel(PanelSpec::new().collapsible(true), |ui| {
                            Text::new("bottom").show(ui);
                        })
                        .show(ui);
                })
                .show(ui);
        });
    });
}

#[test]
fn organisms_forms_render() {
    rendered(|ui| {
        let mut s = 0;
        Select::new(&mut s).options(["A", "B", "C"]).show(ui);
        Accordion::new().show(ui, |acc| {
            acc.section("S1", |ui| {
                Text::new("x").show(ui);
            });
            acc.section("S2", |ui| {
                Text::new("y").show(ui);
            });
        });
        Menubar::new()
            .menu("File", ["New", "Open"])
            .menu("Edit", ["Undo"])
            .show(ui);
    });
}

#[test]
fn surface_and_field_render() {
    rendered(|ui| {
        Surface::new().elevated().show(ui, |ui| {
            Text::new("surface").show(ui);
        });
        let mut s = String::new();
        Field::new("Label")
            .required()
            .hint("hint")
            .show(ui, |ui| Input::new(&mut s).show(ui));
    });
}

#[test]
fn card_and_input_group_render() {
    rendered(|ui| {
        Card::new()
            .title("T")
            .description("d")
            .action(|ui| {
                Button::new("x").show(ui);
            })
            .sm()
            .show(ui, |ui| {
                Text::new("body").show(ui);
            });
        let mut s = String::new();
        InputGroup::new(&mut s)
            .leading_icon(light::MAGNIFYING_GLASS)
            .leading_text("$")
            .button(Slot::TrailingInline, light::X, || {})
            .placeholder("search")
            .show(ui);
        let mut n = String::new();
        InputGroup::new(&mut n)
            .text(Slot::BlockStart, "Note")
            .multiline(2)
            .show(ui);
    });
}

#[test]
fn textarea_and_field_orientations_render() {
    rendered(|ui| {
        let mut s = String::from("multi\nline");
        Textarea::new(&mut s).rows(3).placeholder("note").show(ui);
        let mut name = String::new();
        Field::new("Name")
            .required()
            .hint("h")
            .show(ui, |ui| Input::new(&mut name).show(ui));
        let mut on = true;
        Field::new("Vsync")
            .horizontal()
            .show(ui, |ui| Switch::new(&mut on).show(ui));
        FieldSet::new().legend("Group").show(ui, |ui| {
            Text::new("inside").show(ui);
        });
        FieldSeparator::new().label("OR").show(ui);
    });
}

#[test]
fn radio_group_selects() {
    let selected = Rc::new(Cell::new(0usize));
    let sink = selected.clone();
    let mut installed = false;
    let mut harness = Harness::new_ui(move |ui| {
        if !installed {
            Theme::install(ui.ctx(), Mode::Dark);
            installed = true;
            return;
        }
        let mut v = sink.get();
        RadioGroup::new(&mut v).options(["A", "B", "C"]).show(ui);
        sink.set(v);
    });
    harness.run();
    harness.run();
    harness
        .get_by_role_and_label(egui::accesskit::Role::RadioButton, "C")
        .click_accesskit();
    harness.run();
    assert_eq!(selected.get(), 2, "radio group should select option C");
}

#[test]
fn checkbox_card_toggles() {
    let state = Rc::new(Cell::new(false));
    let sink = state.clone();
    let mut installed = false;
    let mut harness = Harness::new_ui(move |ui| {
        if !installed {
            Theme::install(ui.ctx(), Mode::Dark);
            installed = true;
            return;
        }
        let mut v = sink.get();
        CheckboxCard::new(&mut v, "Enable").show(ui);
        sink.set(v);
    });
    harness.run();
    harness.run();
    // The whole card is the click target; click the label text position.
    harness.get_by_label("Enable").click();
    harness.run();
    assert!(state.get(), "clicking the card should toggle it");
}

#[test]
fn disabled_button_does_not_click() {
    let clicked = Rc::new(Cell::new(false));
    let sink = clicked.clone();
    let mut installed = false;
    let mut harness = Harness::new_ui(move |ui| {
        if !installed {
            Theme::install(ui.ctx(), Mode::Dark);
            installed = true;
            return;
        }
        if Button::new("Save").disabled().show(ui).clicked() {
            sink.set(true);
        }
    });
    harness.run();
    harness.run();
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "Save")
        .click_accesskit();
    harness.run();
    assert!(!clicked.get(), "disabled button must not fire a click");
}

#[test]
fn splitter_fixed_bands_hold_px() {
    // A `[fixed · flex · fixed]` vertical splitter must give the fixed bands their exact px and
    // hand the remainder to the flex panel — the header/footer chrome pattern.
    let header_h = Rc::new(Cell::new(0.0f32));
    let body_h = Rc::new(Cell::new(0.0f32));
    let footer_h = Rc::new(Cell::new(0.0f32));
    let (h, b, f) = (header_h.clone(), body_h.clone(), footer_h.clone());
    rendered(move |ui| {
        Splitter::vertical()
            .id_source("test_fixed_bands")
            .panel(PanelSpec::fixed(40.0), |ui| h.set(ui.max_rect().height()))
            .panel(PanelSpec::flex(), |ui| b.set(ui.max_rect().height()))
            .panel(PanelSpec::fixed(24.0), |ui| f.set(ui.max_rect().height()))
            .show(ui);
    });
    assert!(
        (header_h.get() - 40.0).abs() < 0.5,
        "fixed header should be 40px, got {}",
        header_h.get()
    );
    assert!(
        (footer_h.get() - 24.0).abs() < 0.5,
        "fixed footer should be 24px, got {}",
        footer_h.get()
    );
    assert!(
        body_h.get() > 40.0,
        "flex body should take the remainder, got {}",
        body_h.get()
    );
}
