//! Smoke / interaction tests for atoms (egui_kittest).
//!
//! The shared [`rendered`] helper installs the theme fonts first — the named Iosevka
//! families must exist before any atom (which uses a type style) paints, so we run one
//! install frame and then a paint frame.

use egui::Ui;
use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;
use ouroboros_ui::atoms::{
    Avatar, Badge, BadgeVariant, Button, Checkbox, Divider, Heading, HeadingLevel, Icon, Input,
    Radio, Spinner, Surface, Switch, Text, TextRole, Tooltip,
};
use ouroboros_ui::egui_phosphor::light;
use ouroboros_ui::molecules::{Card, CheckboxCard, Field, InputGroup, RadioGroup};
use ouroboros_ui::{Mode, Theme};
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
        Card::new().title("T").description("d").show(ui, |ui| {
            Text::new("body").show(ui);
        });
        let mut s = String::new();
        InputGroup::new(&mut s)
            .leading(light::MAGNIFYING_GLASS)
            .placeholder("search")
            .show(ui);
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
