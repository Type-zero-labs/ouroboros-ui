//! Smoke / interaction tests for atoms (egui_kittest).
//!
//! The shared [`rendered`] helper installs the theme fonts first — the named Iosevka
//! families must exist before any atom (which uses a type style) paints, so we run one
//! install frame and then a paint frame.

use egui::Ui;
use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;
use ouroboros_ui::atoms::{Button, Divider, Heading, HeadingLevel, Icon, Text, TextRole};
use ouroboros_ui::egui_phosphor::light;
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
