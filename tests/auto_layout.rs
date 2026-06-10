//! AutoLayout layout-contract tests (egui_kittest).
//!
//! Ports the core scenarios of the pure-solver suite of `ouroboros-hud/src/layout.rs`
//! (stacking, gaps, fill distribution, min/max clamps, space-between, cross-align,
//! padding) plus the responsive contract introduced by `ds-responsive-autolayout`:
//! bounded measurement, container clamping, wrap, and resize idempotence (anti-ratchet).
//!
//! Cells are captured from inside child closures via `ui.max_rect()` — inside a cell
//! closure that *is* the cell rect.

use egui::{vec2, Rect, Ui, Vec2};
use egui_kittest::Harness;
use ouroboros_ui::{AutoLayout, CrossAlign, Mode, SizeMode, Sizing, Theme};
use std::cell::Cell;
use std::rc::Rc;

/// Render `content` once in a harness of exactly `size`, with theme/fonts installed.
fn rendered_sized(size: Vec2, mut content: impl FnMut(&mut Ui) + 'static) {
    let mut installed = false;
    let mut harness = Harness::builder().with_size(size).build_ui(move |ui| {
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

fn cell_rect() -> (Rc<Cell<Rect>>, Rc<Cell<Rect>>) {
    let r = Rc::new(Cell::new(Rect::NOTHING));
    (r.clone(), r)
}

// ── Ported from the HUD solver suite (hud/src/layout.rs) ─────────────────────

#[test]
fn vertical_gap_stacks_children() {
    let (a, a2) = cell_rect();
    let (b, b2) = cell_rect();
    let (c, c2) = cell_rect();
    rendered_sized(vec2(200.0, 200.0), move |ui| {
        AutoLayout::vertical()
            .gap(10.0)
            .fixed(40.0, {
                let a2 = a2.clone();
                move |ui| {
                    a2.set(ui.max_rect());
                    ui.allocate_space(vec2(100.0, 40.0));
                }
            })
            .fixed(40.0, {
                let b2 = b2.clone();
                move |ui| {
                    b2.set(ui.max_rect());
                    ui.allocate_space(vec2(100.0, 40.0));
                }
            })
            .fixed(40.0, {
                let c2 = c2.clone();
                move |ui| {
                    c2.set(ui.max_rect());
                    ui.allocate_space(vec2(100.0, 40.0));
                }
            })
            .show(ui);
    });
    let (ra, rb, rc) = (a.get(), b.get(), c.get());
    assert!(
        (ra.height() - 40.0).abs() < 0.5,
        "child height 40, got {}",
        ra.height()
    );
    assert!(
        (rb.top() - ra.top() - 50.0).abs() < 0.5,
        "2nd at +50 (40+gap10), got {}",
        rb.top() - ra.top()
    );
    assert!(
        (rc.top() - ra.top() - 100.0).abs() < 0.5,
        "3rd at +100, got {}",
        rc.top() - ra.top()
    );
    assert!(
        (rb.left() - ra.left()).abs() < 0.5,
        "cross Start aligns lefts"
    );
}

#[test]
fn horizontal_gap_rows_children() {
    let (a, a2) = cell_rect();
    let (b, b2) = cell_rect();
    rendered_sized(vec2(200.0, 200.0), move |ui| {
        AutoLayout::horizontal()
            .gap(10.0)
            .fixed(50.0, {
                let a2 = a2.clone();
                move |ui| {
                    a2.set(ui.max_rect());
                    ui.allocate_space(vec2(50.0, 30.0));
                }
            })
            .fixed(50.0, {
                let b2 = b2.clone();
                move |ui| {
                    b2.set(ui.max_rect());
                    ui.allocate_space(vec2(50.0, 30.0));
                }
            })
            .show(ui);
    });
    let (ra, rb) = (a.get(), b.get());
    assert!((ra.width() - 50.0).abs() < 0.5);
    assert!(
        (rb.left() - ra.left() - 60.0).abs() < 0.5,
        "2nd at +60 (50+gap10), got {}",
        rb.left() - ra.left()
    );
}

#[test]
fn one_fill_takes_remaining() {
    let avail = Rc::new(Cell::new(0.0f32));
    let (f, f2) = cell_rect();
    let av = avail.clone();
    rendered_sized(vec2(200.0, 200.0), move |ui| {
        av.set(ui.available_height());
        AutoLayout::vertical()
            .fixed(40.0, |ui| {
                ui.allocate_space(vec2(100.0, 40.0));
            })
            .fill({
                let f2 = f2.clone();
                move |ui| {
                    f2.set(ui.max_rect());
                    ui.allocate_space(vec2(100.0, 10.0));
                }
            })
            .show(ui);
    });
    let expected = avail.get() - 40.0;
    assert!(
        (f.get().height() - expected).abs() < 0.5,
        "fill takes remaining {expected}, got {}",
        f.get().height()
    );
}

#[test]
fn two_fill_split_evenly() {
    let avail = Rc::new(Cell::new(0.0f32));
    let (a, a2) = cell_rect();
    let (b, b2) = cell_rect();
    let av = avail.clone();
    rendered_sized(vec2(200.0, 200.0), move |ui| {
        av.set(ui.available_height());
        AutoLayout::vertical()
            .fill({
                let a2 = a2.clone();
                move |ui| {
                    a2.set(ui.max_rect());
                    ui.allocate_space(vec2(100.0, 10.0));
                }
            })
            .fill({
                let b2 = b2.clone();
                move |ui| {
                    b2.set(ui.max_rect());
                    ui.allocate_space(vec2(100.0, 10.0));
                }
            })
            .show(ui);
    });
    let half = avail.get() / 2.0;
    assert!(
        (a.get().height() - half).abs() < 0.5,
        "first half {half}, got {}",
        a.get().height()
    );
    assert!(
        (b.get().height() - half).abs() < 0.5,
        "second half {half}, got {}",
        b.get().height()
    );
}

#[test]
fn two_fill_with_max_redistributes() {
    // Fill A caps at 60 — B takes the rest (HUD `two_fill_with_max_redistributes`).
    let avail = Rc::new(Cell::new(0.0f32));
    let (a, a2) = cell_rect();
    let (b, b2) = cell_rect();
    let av = avail.clone();
    rendered_sized(vec2(200.0, 200.0), move |ui| {
        av.set(ui.available_height());
        AutoLayout::vertical()
            .child(Sizing::fill().max(60.0), {
                let a2 = a2.clone();
                move |ui| {
                    a2.set(ui.max_rect());
                    ui.allocate_space(vec2(100.0, 10.0));
                }
            })
            .fill({
                let b2 = b2.clone();
                move |ui| {
                    b2.set(ui.max_rect());
                    ui.allocate_space(vec2(100.0, 10.0));
                }
            })
            .show(ui);
    });
    assert!(
        (a.get().height() - 60.0).abs() < 0.5,
        "capped at 60, got {}",
        a.get().height()
    );
    let rest = avail.get() - 60.0;
    assert!(
        (b.get().height() - rest).abs() < 0.5,
        "B takes the rest {rest}, got {}",
        b.get().height()
    );
}

#[test]
fn min_max_clamps_fixed() {
    let (a, a2) = cell_rect();
    rendered_sized(vec2(400.0, 400.0), move |ui| {
        AutoLayout::vertical()
            .child(Sizing::fixed(500.0).max(200.0), {
                let a2 = a2.clone();
                move |ui| {
                    a2.set(ui.max_rect());
                    ui.allocate_space(vec2(100.0, 10.0));
                }
            })
            .show(ui);
    });
    assert!(
        (a.get().height() - 200.0).abs() < 0.5,
        "fixed 500 clamped to 200, got {}",
        a.get().height()
    );
}

#[test]
fn gap_auto_is_space_between() {
    let avail = Rc::new(Cell::new(0.0f32));
    let (a, a2) = cell_rect();
    let (b, b2) = cell_rect();
    let (c, c2) = cell_rect();
    let av = avail.clone();
    rendered_sized(vec2(200.0, 200.0), move |ui| {
        av.set(ui.available_height());
        AutoLayout::vertical()
            .gap_auto()
            .fixed(40.0, {
                let a2 = a2.clone();
                move |ui| {
                    a2.set(ui.max_rect());
                    ui.allocate_space(vec2(100.0, 40.0));
                }
            })
            .fixed(40.0, {
                let b2 = b2.clone();
                move |ui| {
                    b2.set(ui.max_rect());
                    ui.allocate_space(vec2(100.0, 40.0));
                }
            })
            .fixed(40.0, {
                let c2 = c2.clone();
                move |ui| {
                    c2.set(ui.max_rect());
                    ui.allocate_space(vec2(100.0, 40.0));
                }
            })
            .show(ui);
    });
    // used = 120, leftover = H-120, gap = leftover/2 → child 3 ends at the container end.
    let g = (avail.get() - 120.0) / 2.0;
    let (ra, rb, rc) = (a.get(), b.get(), c.get());
    assert!(
        (rb.top() - ra.top() - (40.0 + g)).abs() < 0.5,
        "2nd at 40+{g}, got {}",
        rb.top() - ra.top()
    );
    assert!(
        (rc.bottom() - ra.top() - avail.get()).abs() < 0.5,
        "3rd ends at container end"
    );
}

#[test]
fn cross_align_center_centers_narrow_child() {
    // Container cross hugs the widest child (100); the narrow one (50) centers in it.
    let (a, a2) = cell_rect();
    let (b, b2) = cell_rect();
    rendered_sized(vec2(200.0, 200.0), move |ui| {
        AutoLayout::vertical()
            .cross_align(CrossAlign::Center)
            .fixed(40.0, {
                let a2 = a2.clone();
                move |ui| {
                    a2.set(ui.max_rect());
                    ui.allocate_space(vec2(100.0, 40.0));
                }
            })
            .fixed(40.0, {
                let b2 = b2.clone();
                move |ui| {
                    b2.set(ui.max_rect());
                    ui.allocate_space(vec2(50.0, 40.0));
                }
            })
            .show(ui);
    });
    let off = b.get().left() - a.get().left();
    assert!(
        (off - 25.0).abs() < 0.5,
        "narrow child centered (+25), got {off}"
    );
}

// ── Responsive contract (new in ds-responsive-autolayout) ───────────────────

#[test]
fn bounded_measure_nao_excede_budget() {
    // A greedy Hug child (expands to available_width) measures as the budget — the
    // frame and the cell never exceed the panel. Pre-change this measured 100_000px.
    let avail = Rc::new(Cell::new(0.0f32));
    let frame_w = Rc::new(Cell::new(0.0f32));
    let (cell, cell2) = cell_rect();
    let (av, fw) = (avail.clone(), frame_w.clone());
    rendered_sized(vec2(300.0, 200.0), move |ui| {
        av.set(ui.available_width());
        let resp = AutoLayout::horizontal()
            .hug({
                let cell2 = cell2.clone();
                move |ui| {
                    cell2.set(ui.max_rect());
                    let w = ui.available_width();
                    ui.set_min_width(w); // greedy: expands to whatever it is given
                    ui.allocate_space(vec2(w, 20.0));
                }
            })
            .show(ui);
        fw.set(resp.rect.width());
    });
    assert!(
        frame_w.get() <= avail.get() + 0.5,
        "frame ({}) must not exceed the budget ({})",
        frame_w.get(),
        avail.get()
    );
    assert!(
        cell.get().width() <= avail.get() + 0.5,
        "cell ({}) must not exceed the budget ({})",
        cell.get().width(),
        avail.get()
    );
}

#[test]
fn container_clamps_to_available() {
    // Two Fixed(300) in a ~400px budget: the frame clamps to the budget instead of
    // allocating 610px and painting over siblings (cells overflow but are clipped).
    let avail = Rc::new(Cell::new(0.0f32));
    let frame_w = Rc::new(Cell::new(0.0f32));
    let (av, fw) = (avail.clone(), frame_w.clone());
    rendered_sized(vec2(400.0, 200.0), move |ui| {
        av.set(ui.available_width());
        let resp = AutoLayout::horizontal()
            .gap(10.0)
            .fixed(300.0, |ui| {
                ui.allocate_space(vec2(300.0, 20.0));
            })
            .fixed(300.0, |ui| {
                ui.allocate_space(vec2(300.0, 20.0));
            })
            .show(ui);
        fw.set(resp.rect.width());
    });
    assert!(
        frame_w.get() <= avail.get() + 0.5,
        "frame ({}) clamps to budget ({})",
        frame_w.get(),
        avail.get()
    );
}

#[test]
fn fill_min_floors_at_min() {
    // Fixed(350) + fill_min(80) in ~400px: free ≈ 50 < 80 — the fill floors at 80.
    let (f, f2) = cell_rect();
    rendered_sized(vec2(400.0, 200.0), move |ui| {
        AutoLayout::horizontal()
            .fixed(350.0, |ui| {
                ui.allocate_space(vec2(350.0, 20.0));
            })
            .fill_min(80.0, {
                let f2 = f2.clone();
                move |ui| {
                    f2.set(ui.max_rect());
                    ui.allocate_space(vec2(10.0, 20.0));
                }
            })
            .show(ui);
    });
    assert!(
        (f.get().width() - 80.0).abs() < 0.5,
        "fill floors at its min 80, got {}",
        f.get().width()
    );
}

#[test]
fn wrap_quebra_linha() {
    // 3×Fixed(100), gap 10, ~250px budget → lines [1,2][3]: the 3rd child starts a new
    // line below (cross offset = line height 30 + line gap 10).
    let (a, a2) = cell_rect();
    let (c, c2) = cell_rect();
    rendered_sized(vec2(250.0, 200.0), move |ui| {
        AutoLayout::horizontal()
            .wrap()
            .gap(10.0)
            .fixed(100.0, {
                let a2 = a2.clone();
                move |ui| {
                    a2.set(ui.max_rect());
                    ui.allocate_space(vec2(100.0, 30.0));
                }
            })
            .fixed(100.0, |ui| {
                ui.allocate_space(vec2(100.0, 30.0));
            })
            .fixed(100.0, {
                let c2 = c2.clone();
                move |ui| {
                    c2.set(ui.max_rect());
                    ui.allocate_space(vec2(100.0, 30.0));
                }
            })
            .show(ui);
    });
    let (ra, rc) = (a.get(), c.get());
    assert!(
        (rc.left() - ra.left()).abs() < 0.5,
        "wrapped child returns to line start"
    );
    assert!(
        (rc.top() - ra.top() - 40.0).abs() < 0.5,
        "wrapped child sits one line below (+40), got {}",
        rc.top() - ra.top()
    );
}

#[test]
fn wrap_fill_consome_resto_da_linha() {
    // Line 1: Fixed(100) + Fill → the fill takes the line's remainder; a following
    // Fixed(200) wraps to line 2.
    let avail = Rc::new(Cell::new(0.0f32));
    let (f, f2) = cell_rect();
    let (c, c2) = cell_rect();
    let (a, a2) = cell_rect();
    let av = avail.clone();
    rendered_sized(vec2(250.0, 200.0), move |ui| {
        av.set(ui.available_width());
        AutoLayout::horizontal()
            .wrap()
            .gap(10.0)
            .fixed(100.0, {
                let a2 = a2.clone();
                move |ui| {
                    a2.set(ui.max_rect());
                    ui.allocate_space(vec2(100.0, 30.0));
                }
            })
            .fill({
                let f2 = f2.clone();
                move |ui| {
                    f2.set(ui.max_rect());
                    ui.allocate_space(vec2(10.0, 30.0));
                }
            })
            .fixed(200.0, {
                let c2 = c2.clone();
                move |ui| {
                    c2.set(ui.max_rect());
                    ui.allocate_space(vec2(200.0, 30.0));
                }
            })
            .show(ui);
    });
    let expected = avail.get() - 100.0 - 10.0;
    assert!(
        (f.get().width() - expected).abs() < 0.5,
        "fill takes the line remainder {expected}, got {}",
        f.get().width()
    );
    assert!(
        c.get().top() > a.get().top() + 29.0,
        "the 200px child wraps to the second line"
    );
}

// ── Anti-ratchet: resize out and back must be idempotent (egui #1297 class) ──

#[test]
fn resize_shrink_volta() {
    // Two fill_min(120) columns, one containing a greedy width-filler. Shrink the
    // window 500→300 and back →500: the cells must return to their exact first-frame
    // rects — no ratchet, no retained stretch.
    let a = Rc::new(Cell::new(Rect::NOTHING));
    let b = Rc::new(Cell::new(Rect::NOTHING));
    let (a2, b2) = (a.clone(), b.clone());
    let mut installed = false;
    let mut harness = Harness::builder()
        .with_size(vec2(500.0, 200.0))
        .build_ui(move |ui| {
            if !installed {
                Theme::install(ui.ctx(), Mode::Dark);
                installed = true;
                return;
            }
            AutoLayout::horizontal()
                .gap(16.0)
                .fill_min(120.0, {
                    let a2 = a2.clone();
                    move |ui| {
                        a2.set(ui.max_rect());
                        let w = ui.available_width();
                        ui.set_min_width(w); // greedy content — the ratchet trigger
                        ui.allocate_space(vec2(w, 20.0));
                    }
                })
                .fill_min(120.0, {
                    let b2 = b2.clone();
                    move |ui| {
                        b2.set(ui.max_rect());
                        ui.allocate_space(vec2(10.0, 20.0));
                    }
                })
                .show(ui);
        });
    harness.run(); // install fonts
    harness.run();
    let (a_wide, b_wide) = (a.get(), b.get());

    harness.set_size(vec2(300.0, 200.0));
    harness.run();
    let (a_narrow, b_narrow) = (a.get(), b.get());
    assert!(
        a_narrow.width() < a_wide.width(),
        "shrinking the window must shrink the cell ({} -> {})",
        a_wide.width(),
        a_narrow.width()
    );
    assert!(
        b_narrow.right() <= 300.5,
        "no cell may escape the narrow window, got right={}",
        b_narrow.right()
    );

    harness.set_size(vec2(500.0, 200.0));
    harness.run();
    let (a_back, b_back) = (a.get(), b.get());
    assert!(
        (a_back.width() - a_wide.width()).abs() < 0.5
            && (b_back.width() - b_wide.width()).abs() < 0.5,
        "rects must return exactly after resize out-and-back: a {} vs {}, b {} vs {}",
        a_wide.width(),
        a_back.width(),
        b_wide.width(),
        b_back.width()
    );
}

// ── Rect API (moved from tests/atoms.rs) ─────────────────────────────────────

#[test]
fn auto_layout_layout_sizes_fixed_and_fill() {
    // The rect-returning AutoLayout path (for `&mut self` sibling cells): Fixed reserves px, Fill
    // takes the remainder, all cells span the full cross axis.
    let lead = Rc::new(Cell::new(0.0f32));
    let rest = Rc::new(Cell::new(0.0f32));
    let (l, r) = (lead.clone(), rest.clone());
    rendered_sized(vec2(400.0, 200.0), move |ui| {
        let out = AutoLayout::horizontal()
            .region(SizeMode::Fixed(120.0))
            .region(SizeMode::Fill)
            .layout(ui);
        l.set(out.rects[0].width());
        r.set(out.rects[1].width());
    });
    assert!(
        (lead.get() - 120.0).abs() < 0.5,
        "fixed cell should be 120px, got {}",
        lead.get()
    );
    assert!(
        rest.get() > 50.0,
        "fill cell should take the remainder, got {}",
        rest.get()
    );
}

#[test]
fn layout_region_respects_sizing_clamps() {
    // The rect API accepts full Sizing: a clamped flex region pins to its max.
    let a = Rc::new(Cell::new(0.0f32));
    let b = Rc::new(Cell::new(0.0f32));
    let (a2, b2) = (a.clone(), b.clone());
    rendered_sized(vec2(400.0, 200.0), move |ui| {
        let out = AutoLayout::horizontal()
            .region(Sizing::fill().max(80.0))
            .region(Sizing::fill())
            .layout(ui);
        a2.set(out.rects[0].width());
        b2.set(out.rects[1].width());
    });
    assert!(
        (a.get() - 80.0).abs() < 0.5,
        "clamped region pins at 80, got {}",
        a.get()
    );
    assert!(
        b.get() > 200.0,
        "open region takes the rest, got {}",
        b.get()
    );
}

// ── Regressions from Resize Lab manual QA (2026-06-09) ───────────────────────

#[test]
fn fill_columns_with_property_rows_keep_height() {
    // Resize Lab section (e): two fill_min(140) columns of PropertyRow+NumericField
    // collapsed to slivers ("2 traços"). Cells must be tall enough for two rows.
    use ouroboros_ui::atoms::NumericField;
    use ouroboros_ui::cells::PropertyRow;
    let (a, a2) = cell_rect();
    let (b, b2) = cell_rect();
    rendered_sized(vec2(520.0, 400.0), move |ui| {
        let mut v1 = 0.6f32;
        let mut v2 = 0.25f32;
        let mut v3 = 0.4f32;
        let mut v4 = 0.8f32;
        AutoLayout::horizontal()
            .gap(16.0)
            .fill_min(140.0, {
                let a2 = a2.clone();
                move |ui| {
                    a2.set(ui.max_rect());
                    PropertyRow::new("Bounce").show(ui, |ui| NumericField::new(&mut v1).show(ui));
                    PropertyRow::new("Friction").show(ui, |ui| NumericField::new(&mut v2).show(ui));
                }
            })
            .fill_min(140.0, {
                let b2 = b2.clone();
                move |ui| {
                    b2.set(ui.max_rect());
                    PropertyRow::new("Damping").show(ui, |ui| NumericField::new(&mut v3).show(ui));
                    PropertyRow::new("Restitution")
                        .show(ui, |ui| NumericField::new(&mut v4).show(ui));
                }
            })
            .show(ui);
    });
    assert!(
        a.get().height() >= 50.0,
        "left column must fit two property rows, got {}x{}",
        a.get().width(),
        a.get().height()
    );
    assert!(
        b.get().height() >= 50.0,
        "right column must fit two property rows, got {}x{}",
        b.get().width(),
        b.get().height()
    );
}

#[test]
fn fill_alert_band_grows_taller_when_narrow() {
    // Resize Lab section (b): the Fill alert must WRAP its long message in a narrow
    // panel (cell grows taller) instead of clipping at the Hug button.
    use ouroboros_ui::atoms::Button;
    use ouroboros_ui::molecules::Alert;
    let wide = Rc::new(Cell::new(0.0f32));
    let narrow = Rc::new(Cell::new(0.0f32));
    let msg = "Autosave recovered three unsaved changes from the previous session - \
               review them before publishing; a narrow panel must wrap this message.";
    for (size, sink) in [(700.0f32, wide.clone()), (300.0f32, narrow.clone())] {
        let s2 = sink.clone();
        rendered_sized(vec2(size, 400.0), move |ui| {
            AutoLayout::horizontal()
                .gap(8.0)
                .fill({
                    let s2 = s2.clone();
                    move |ui| {
                        s2.set(ui.max_rect().height().max(ui.min_rect().height()));
                        Alert::new(msg).warning().show(ui);
                    }
                })
                .hug(|ui| {
                    Button::new("Action").show(ui);
                })
                .show(ui);
        });
    }
    assert!(
        narrow.get() > wide.get() + 10.0,
        "narrow band must be taller (wrapped message): wide {} vs narrow {}",
        wide.get(),
        narrow.get()
    );
}

#[test]
fn columns_below_scroll_fold_keep_height() {
    // Resize Lab replica: the columns section sits BELOW the fold of a vertical
    // ScrollArea inside a fixed-height Splitter panel. available_height() at that
    // cursor is ~0 - the cross clamp must NOT crush the cells ("2 tracos" bug).
    use ouroboros_ui::atoms::NumericField;
    use ouroboros_ui::cells::PropertyRow;
    use ouroboros_ui::organisms::{PanelSpec, Splitter};
    let (a, a2) = cell_rect();
    rendered_sized(vec2(800.0, 600.0), move |ui| {
        let a2 = a2.clone();
        ui.allocate_ui(egui::vec2(ui.available_width(), 460.0), |ui| {
            Splitter::horizontal()
                .id_source("probe_split")
                .panel(
                    PanelSpec::new().size(0.65).min(180.0).max(520.0),
                    move |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("probe_scroll")
                            .auto_shrink([true, false])
                            .show(ui, |ui| {
                                ui.add_space(600.0); // push the section below the fold
                                let mut v1 = 0.6f32;
                                let mut v2 = 0.25f32;
                                AutoLayout::horizontal()
                                    .gap(16.0)
                                    .fill_min(140.0, {
                                        let a2 = a2.clone();
                                        move |ui| {
                                            a2.set(ui.max_rect());
                                            PropertyRow::new("Bounce")
                                                .show(ui, |ui| NumericField::new(&mut v1).show(ui));
                                            PropertyRow::new("Friction")
                                                .show(ui, |ui| NumericField::new(&mut v2).show(ui));
                                        }
                                    })
                                    .fill_min(140.0, |ui| {
                                        let _ = ui;
                                    })
                                    .show(ui);
                            });
                    },
                )
                .panel(PanelSpec::flex(), |ui| {
                    let _ = ui;
                })
                .show(ui);
        });
    });
    assert!(
        a.get().height() >= 50.0,
        "below-the-fold column must keep its height, got {}x{}",
        a.get().width(),
        a.get().height()
    );
}

#[test]
fn vertical_flow_below_scroll_fold_keeps_height() {
    // Same bug class on the main axis: a VERTICAL AutoLayout below the fold must hug
    // its content height, not clamp to the ~0 available_height at that cursor.
    let (a, a2) = cell_rect();
    rendered_sized(vec2(400.0, 300.0), move |ui| {
        let a2 = a2.clone();
        egui::ScrollArea::vertical()
            .id_salt("probe_vscroll")
            .auto_shrink([true, false])
            .show(ui, |ui| {
                ui.add_space(500.0); // below the fold
                AutoLayout::vertical()
                    .gap(8.0)
                    .hug({
                        let a2 = a2.clone();
                        move |ui| {
                            a2.set(ui.max_rect());
                            ui.allocate_space(egui::vec2(100.0, 40.0));
                        }
                    })
                    .hug(|ui| {
                        ui.allocate_space(egui::vec2(100.0, 40.0));
                    })
                    .show(ui);
            });
    });
    assert!(
        (a.get().height() - 40.0).abs() < 0.5,
        "below-the-fold vertical child keeps its height, got {}",
        a.get().height()
    );
}

// ── Review fixes (2026-06-09): regressões dos findings do code review ───────

#[test]
fn padding_offsets_content() {
    // Ported from the HUD suite: padding insets the first child on both axes.
    let (a, a2) = cell_rect();
    let frame = Rc::new(Cell::new(Rect::NOTHING));
    let fr = frame.clone();
    rendered_sized(vec2(200.0, 200.0), move |ui| {
        let a2 = a2.clone();
        let resp = AutoLayout::vertical()
            .pad(20.0)
            .fixed(40.0, move |ui| {
                a2.set(ui.max_rect());
                ui.allocate_space(vec2(100.0, 40.0));
            })
            .show(ui);
        fr.set(resp.rect);
    });
    assert!(
        (a.get().top() - frame.get().top() - 20.0).abs() < 0.5,
        "child y inset by padding, got {}",
        a.get().top() - frame.get().top()
    );
    assert!(
        (a.get().left() - frame.get().left() - 20.0).abs() < 0.5,
        "child x inset by padding, got {}",
        a.get().left() - frame.get().left()
    );
}

#[test]
fn layout_regions_never_negative() {
    // distribute_fill can hand a sibling a NEGATIVE share when another region's min
    // pins above the leftover - the rect API must clamp to zero, not invert rects.
    let w = Rc::new(Cell::new(-1.0f32));
    let sink = w.clone();
    rendered_sized(vec2(120.0, 100.0), move |ui| {
        let out = AutoLayout::horizontal()
            .region(Sizing::fill().min(140.0))
            .region(Sizing::fill())
            .layout(ui);
        sink.set(out.rects[1].width());
    });
    assert!(
        w.get() >= -0.01,
        "region width must never be negative, got {}",
        w.get()
    );
}

#[test]
fn columns_stack_when_narrow() {
    // The studio pilot scenario: two fill_min(220) columns with wrap() sit side by
    // side when wide and stack full-width when the panel cannot fit both.
    for (size, stacked) in [(600.0f32, false), (300.0f32, true)] {
        let (a, a2) = cell_rect();
        let (b, b2) = cell_rect();
        rendered_sized(vec2(size, 400.0), move |ui| {
            AutoLayout::horizontal()
                .wrap()
                .gap(24.0)
                .gap_cross(24.0)
                .fill_min(220.0, {
                    let a2 = a2.clone();
                    move |ui| {
                        a2.set(ui.max_rect());
                        ui.allocate_space(vec2(50.0, 60.0));
                    }
                })
                .fill_min(220.0, {
                    let b2 = b2.clone();
                    move |ui| {
                        b2.set(ui.max_rect());
                        ui.allocate_space(vec2(50.0, 60.0));
                    }
                })
                .show(ui);
        });
        let (ra, rb) = (a.get(), b.get());
        if stacked {
            assert!(
                rb.top() >= ra.bottom() - 0.5,
                "at {size}px the right column stacks below, got a={ra:?} b={rb:?}"
            );
            assert!(
                ra.width() > 250.0,
                "stacked columns take the full width, got {}",
                ra.width()
            );
        } else {
            assert!(
                (rb.top() - ra.top()).abs() < 0.5 && rb.left() > ra.right(),
                "at {size}px the columns sit side by side, got a={ra:?} b={rb:?}"
            );
        }
    }
}

#[test]
fn wrap_resize_idempotent() {
    // Anti-ratchet for the WRAP path: shrink (reflow to more lines) and grow back -
    // the first cell must return to its exact original rect.
    let a = Rc::new(Cell::new(Rect::NOTHING));
    let a2 = a.clone();
    let mut installed = false;
    let mut harness = Harness::builder()
        .with_size(vec2(560.0, 300.0))
        .build_ui(move |ui| {
            if !installed {
                Theme::install(ui.ctx(), Mode::Dark);
                installed = true;
                return;
            }
            let mut grid = AutoLayout::horizontal().wrap().gap(8.0).gap_cross(8.0);
            for i in 0..6 {
                let probe = (i == 0).then(|| a2.clone());
                grid = grid.fill_min(72.0, move |ui| {
                    if let Some(p) = &probe {
                        p.set(ui.max_rect());
                    }
                    ui.allocate_space(vec2(40.0, 24.0));
                });
            }
            grid.show(ui);
        });
    harness.run();
    harness.run();
    let wide = a.get();
    harness.set_size(vec2(250.0, 300.0));
    harness.run();
    let narrow = a.get();
    assert!(
        narrow.width() < wide.width(),
        "narrow window reflows the grid ({} -> {})",
        wide.width(),
        narrow.width()
    );
    harness.set_size(vec2(560.0, 300.0));
    harness.run();
    let back = a.get();
    assert!(
        (back.width() - wide.width()).abs() < 0.5 && (back.top() - wide.top()).abs() < 0.5,
        "wrap grid returns exactly after resize out-and-back: {} vs {}",
        wide.width(),
        back.width()
    );
}
