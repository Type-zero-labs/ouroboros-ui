//! The shell pieces of the Oct 2026 redesign: rail, side panel, splitter collapse API,
//! window resize grips, drawer, toast actions.

use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;
use ouroboros_ui::atoms::{RailButton, WindowResizeBorder};
use ouroboros_ui::egui_phosphor::light;
use ouroboros_ui::organisms::{
    is_collapsed, set_collapsed, Drawer, PanelSpec, Rail, RailItem, SidePanel, Splitter, Toast,
};
use ouroboros_ui::tokens::layout;
use ouroboros_ui::{Mode, Theme};
use std::cell::Cell;
use std::rc::Rc;

fn harness(mut content: impl FnMut(&mut egui::Ui) + 'static) -> Harness<'static> {
    let mut installed = false;
    let mut h = Harness::new_ui(move |ui| {
        if !installed {
            Theme::install(ui.ctx(), Mode::Dark);
            installed = true;
            return;
        }
        content(ui);
    });
    h.run();
    h.run();
    h
}

/// Clicking a module switches; clicking the active one again reports `reclicked` (the shell
/// collapses the side panel with it).
#[test]
fn rail_switches_and_reports_reclick() {
    let active = Rc::new(Cell::new(0usize));
    let reclicks = Rc::new(Cell::new(0u32));
    let (a, r) = (active.clone(), reclicks.clone());
    let mut h = harness(move |ui| {
        let mut idx = a.get();
        let resp = Rail::new(&mut idx)
            .item(RailItem::new(light::DATABASE, "Database"))
            .item(RailItem::new(light::GLOBE, "World"))
            .footer(RailItem::new(light::TERMINAL_WINDOW, "Console").badge(Some(2)))
            .show(ui);
        a.set(idx);
        if resp.reclicked {
            r.set(r.get() + 1);
        }
    });
    h.get_by_label("World").click();
    h.run();
    assert_eq!(active.get(), 1, "switched to World");
    h.get_by_label("World").click();
    h.run();
    assert_eq!(active.get(), 1);
    assert_eq!(reclicks.get(), 1, "clicking the active module is a reclick");
}

/// The rail button renders at its token size.
#[test]
fn rail_button_has_token_size() {
    let size = Rc::new(Cell::new(egui::Vec2::ZERO));
    let s = size.clone();
    let _h = harness(move |ui| {
        let r = RailButton::new(light::GLOBE)
            .active(true)
            .badge(Some(5))
            .show(ui);
        s.set(r.rect.size());
    });
    assert_eq!(
        size.get(),
        egui::vec2(layout::RAIL_BUTTON, layout::RAIL_BUTTON)
    );
}

/// The side panel header holds the title and the search; the body follows.
#[test]
fn side_panel_renders_title_and_search() {
    let h = harness(move |ui| {
        let mut search = String::new();
        let mut collapsed = false;
        let mut width = layout::SIDE_PANEL_W;
        SidePanel::new("p")
            .title("Database")
            .search(&mut search, "Search")
            .collapsible(&mut collapsed)
            .resizable(&mut width)
            .show(ui, |ui| {
                ouroboros_ui::atoms::Text::new("Player").show(ui);
            });
    });
    h.get_by_label("database");
    h.get_by_label("Player");
}

/// Collapse from code (the rail click) before and after the splitter's first frame.
#[test]
fn splitter_collapses_from_code() {
    let ctx = egui::Context::default();
    set_collapsed(&ctx, "split", 1, true);
    assert!(is_collapsed(&ctx, "split", 1));
    let mut h = Harness::new_ui(move |ui| {
        Splitter::horizontal()
            .id_source("split")
            .panel(PanelSpec::flex(), |_| {})
            .panel(PanelSpec::new().collapsible(true), |_| {})
            .show(ui);
    });
    h.run();
    set_collapsed(&h.ctx, "split", 1, true);
    h.run();
    assert!(is_collapsed(&h.ctx, "split", 1));
    set_collapsed(&h.ctx, "split", 1, false);
    assert!(!is_collapsed(&h.ctx, "split", 1));
}

/// No drag, no resize.
#[test]
fn window_border_idle_returns_none() {
    let grabbed = Rc::new(Cell::new(false));
    let g = grabbed.clone();
    let _h = harness(move |ui| {
        let rect = ui.max_rect();
        g.set(WindowResizeBorder::new().show(ui, rect).is_some());
    });
    assert!(!grabbed.get());
}

/// The drawer draws only when open, and clamps its height.
#[test]
fn drawer_clamps_height() {
    let height = Rc::new(Cell::new(5000.0_f32));
    let hh = height.clone();
    let _h = harness(move |ui| {
        let mut open = true;
        let mut h = hh.get();
        let canvas = ui.max_rect();
        Drawer::new("d", &mut open, &mut h).show(ui.ctx(), canvas, |ui| {
            ouroboros_ui::atoms::Text::new("log").show(ui);
        });
        hh.set(h);
    });
    assert!(height.get() < 5000.0, "clamped to 60% of the canvas");
}

/// A toast with actions renders its buttons.
#[test]
fn toast_actions_render() {
    let h = harness(move |ui| {
        Toast::new("Exported")
            .action("Reveal")
            .action("Dismiss")
            .bottom()
            .show_with_actions(ui.ctx());
    });
    h.get_by_label("Reveal");
    h.get_by_label("Exported");
}
