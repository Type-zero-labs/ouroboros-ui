//! Splitter organism — resizable panes split by draggable dividers. [Element Plus Splitter]
//!
//! Horizontal (side-by-side) or vertical (stacked); each panel carries min/max bounds and may
//! be resizable and/or collapsible. Panel sizes persist for the **session** in egui memory
//! (keyed by `id_source`), not to disk. Composes the
//! [`SplitterHandle`](crate::atoms::SplitterHandle) atom per divider; never paints directly.
//!
//! ```ignore
//! Splitter::horizontal()
//!     .id_source("editor")
//!     .panel(PanelSpec::new().min(180.0).max(420.0), |ui| hierarchy(ui))
//!     .panel(PanelSpec::new(), |ui| viewport(ui))
//!     .panel(PanelSpec::new().collapsible(true), |ui| inspector(ui))
//!     .show(ui);
//! ```
//!
//! Resizing follows the adjacent-pair rule: dragging a divider grows one neighbor and shrinks
//! the other, clamped to both panels' min/max. Double-clicking a divider toggles the collapse
//! of an adjacent `collapsible` panel.

use crate::atoms::{Axis, SplitterHandle};
use crate::tokens::{core, layout};
use egui::{pos2, vec2, Id, Rect, Response, Sense, Ui, UiBuilder, Vec2};

/// Per-panel configuration. Builder; pair it with a content closure via [`Splitter::panel`].
#[derive(Clone, Copy, Debug)]
pub struct PanelSpec {
    /// Initial size as a fraction of the splitter's main axis (0..1). `None` = equal share.
    size: Option<f32>,
    /// Fixed size in px on the main axis. When `Some`, the panel is excluded from flex
    /// distribution (reserves its px before the rest is shared) and is non-resizable by default —
    /// for fixed chrome bands like headers/footers/toolbars. `None` = a flex panel.
    fixed: Option<f32>,
    /// Minimum / maximum size in px. `max` defaults to unbounded (`f32::INFINITY`) so a wide
    /// flex panel never blocks an adjacent divider's drag.
    min: f32,
    max: f32,
    resizable: bool,
    collapsible: bool,
    /// Inner padding (px) applied to the panel's content rect. `0` = flush (the default).
    pad: f32,
}

impl PanelSpec {
    pub fn new() -> Self {
        Self {
            size: None,
            fixed: None,
            min: layout::PANEL_MIN,
            max: f32::INFINITY,
            resizable: true,
            collapsible: false,
            pad: 0.0,
        }
    }
    /// A flex panel: no explicit size (takes the remainder) and unbounded `max`. Same as
    /// [`PanelSpec::new`] — a readable alias for the wide center pane (viewport/canvas).
    pub fn flex() -> Self {
        Self::new()
    }
    /// A fixed-size band: an absolute `px` size on the main axis, excluded from flex distribution
    /// (reserved before the remainder is shared) and **non-resizable** by default. The canonical
    /// header/footer/toolbar/rail band that must keep its px through window resizes. Override the
    /// divider behaviour with [`PanelSpec::resizable`] if a fixed band should still be draggable.
    pub fn fixed(px: f32) -> Self {
        Self {
            fixed: Some(px.max(0.0)),
            resizable: false,
            ..Self::new()
        }
    }
    /// Initial size as a fraction of the main axis (0..1).
    pub fn size(mut self, fraction: f32) -> Self {
        self.size = Some(fraction.clamp(0.0, 1.0));
        self
    }
    /// Inner padding (px) for the panel's content. Default `0` (flush).
    pub fn pad(mut self, px: f32) -> Self {
        self.pad = px;
        self
    }
    pub fn min(mut self, px: f32) -> Self {
        self.min = px;
        self
    }
    pub fn max(mut self, px: f32) -> Self {
        self.max = px;
        self
    }
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }
}

impl Default for PanelSpec {
    fn default() -> Self {
        Self::new()
    }
}

struct Panel<'a> {
    cfg: PanelSpec,
    add: Box<dyn FnMut(&mut Ui) + 'a>,
}

/// Session-persisted splitter state (panel fractions + collapse flags), keyed by the splitter Id.
#[derive(Clone, Default)]
struct SplitterState {
    fracs: Vec<f32>,
    collapsed: Vec<bool>,
}

/// A resizable pane splitter. Build with [`Splitter::horizontal`] / [`Splitter::vertical`],
/// add panels, then [`Splitter::show`].
pub struct Splitter<'a> {
    horizontal: bool,
    id_source: Option<Id>,
    panels: Vec<Panel<'a>>,
}

impl<'a> Splitter<'a> {
    fn new(horizontal: bool) -> Self {
        Self {
            horizontal,
            id_source: None,
            panels: Vec::new(),
        }
    }
    /// Panels laid out left-to-right, dividers drag horizontally.
    pub fn horizontal() -> Self {
        Self::new(true)
    }
    /// Panels stacked top-to-bottom, dividers drag vertically.
    pub fn vertical() -> Self {
        Self::new(false)
    }

    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id_source = Some(Id::new(id));
        self
    }

    /// Add a panel with its config and content closure.
    pub fn panel(mut self, cfg: PanelSpec, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.panels.push(Panel {
            cfg,
            add: Box::new(add),
        });
        self
    }

    /// Add a panel without a content closure, for use with [`Splitter::layout`]. Use this when
    /// each panel's body needs `&mut self` of the caller — three `FnMut(&mut Ui)` closures can't
    /// each borrow the same state, so instead lay the splitter out, get the rects back, and draw
    /// into them sequentially via `ui.new_child`.
    pub fn region(self, cfg: PanelSpec) -> Self {
        self.panel(cfg, |_| {})
    }

    /// Render panels by invoking their content closures.
    pub fn show(mut self, ui: &mut Ui) -> Response {
        let SplitterLayout { rects, response } = self.layout_rects(ui);
        for (i, slot) in rects.iter().enumerate() {
            if let Some(content_rect) = slot {
                let mut cui = ui.new_child(UiBuilder::new().max_rect(*content_rect));
                cui.set_clip_rect(*content_rect);
                (self.panels[i].add)(&mut cui);
            }
        }
        response
    }

    /// Lay the splitter out — draw the dividers, handle resize/collapse, persist state — and
    /// return one content rect per panel (`None` when collapsed) **instead of** invoking content
    /// closures. Use with [`Splitter::region`] when each panel body needs `&mut self`; draw into
    /// the returned rects sequentially via `ui.new_child`.
    pub fn layout(mut self, ui: &mut Ui) -> SplitterLayout {
        self.layout_rects(ui)
    }

    /// Shared layout core for [`Splitter::show`] and [`Splitter::layout`]: allocate, size panels
    /// (fixed-px bands + flex split), draw dividers, apply drag/collapse, persist state. Returns
    /// the per-panel content rects; the caller decides whether to render closures or hand the
    /// rects back.
    fn layout_rects(&mut self, ui: &mut Ui) -> SplitterLayout {
        let n = self.panels.len();
        let outer = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(outer, Sense::hover());
        if n == 0 {
            return SplitterLayout {
                rects: Vec::new(),
                response,
            };
        }

        let horizontal = self.horizontal;
        let main = |v: Vec2| if horizontal { v.x } else { v.y };
        let div = core::SPACE_2;
        let main_len = main(rect.size());
        let content_main = (main_len - div * (n as f32 - 1.0)).max(1.0);

        // ── Fixed bands reserve px up front; flex panels share what's left ──
        let fixed_px: Vec<Option<f32>> = self.panels.iter().map(|p| p.cfg.fixed).collect();
        let fixed_total: f32 = fixed_px.iter().flatten().copied().sum();
        let flex_content = (content_main - fixed_total).max(0.0);

        // ── Load or initialise session state ──
        let id = self.id_source.unwrap_or(response.id);
        let mut state = ui
            .data(|d| d.get_temp::<SplitterState>(id))
            .filter(|s| s.fracs.len() == n)
            .unwrap_or_else(|| SplitterState {
                fracs: init_fracs(&self.panels),
                collapsed: vec![false; n],
            });

        // ── Effective fractions: fixed and collapsed panels contribute 0 to the flex split ──
        let mut eff: Vec<f32> = (0..n)
            .map(|i| {
                if fixed_px[i].is_some() || state.collapsed[i] {
                    0.0
                } else {
                    state.fracs[i]
                }
            })
            .collect();
        let sum: f32 = eff.iter().sum();
        if sum > f32::EPSILON {
            for f in &mut eff {
                *f /= sum;
            }
        }
        // Fixed panels take their px (clamped to what's available); flex panels share the rest.
        let pixels: Vec<f32> = (0..n)
            .map(|i| match fixed_px[i] {
                Some(px) => px.min(content_main),
                None => eff[i] * flex_content,
            })
            .collect();

        // ── Render panels + dividers along the main axis ──
        let cell = |start: f32, len: f32, cross_full: &Rect| -> Rect {
            if horizontal {
                Rect::from_min_size(
                    pos2(start, cross_full.top()),
                    vec2(len, cross_full.height()),
                )
            } else {
                Rect::from_min_size(
                    pos2(cross_full.left(), start),
                    vec2(cross_full.width(), len),
                )
            }
        };
        let main_start = if horizontal { rect.left() } else { rect.top() };

        let mut cursor = main_start;
        let mut drag_for: Option<(usize, f32)> = None;
        let mut toggle: Option<usize> = None;
        let mut rects: Vec<Option<Rect>> = vec![None; n];

        // Indexed loop: each iteration touches several parallel arrays plus the `i + 1`
        // neighbor for the divider, so a single enumerate() doesn't fit cleanly.
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            let p_rect = cell(cursor, pixels[i], &rect);
            if !state.collapsed[i] {
                let pad = self.panels[i].cfg.pad;
                let content_rect = if pad > 0.0 {
                    p_rect.shrink(pad)
                } else {
                    p_rect
                };
                rects[i] = Some(content_rect);
            }
            cursor += pixels[i];

            // Divider after every panel except the last.
            if i + 1 < n {
                let d_rect = cell(cursor, div, &rect);
                let line = if horizontal {
                    Axis::Vertical
                } else {
                    Axis::Horizontal
                };
                let pair_resizable =
                    self.panels[i].cfg.resizable && self.panels[i + 1].cfg.resizable;
                let active = state.collapsed[i] || state.collapsed[i + 1];
                let mut hui = ui.new_child(UiBuilder::new().max_rect(d_rect));
                let h = SplitterHandle::new(line).active(active).show(&mut hui);
                if pair_resizable && h.dragged() {
                    drag_for = Some((i, main(h.drag_delta())));
                }
                if h.double_clicked() {
                    toggle = Some(i);
                }
                cursor += div;
            }
        }

        // ── Apply a drag: grow panel i, shrink panel i+1, clamped to both bounds ──
        if let Some((i, delta)) = drag_for {
            if delta != 0.0 {
                let bounds = (
                    self.panels[i].cfg.min,
                    self.panels[i].cfg.max,
                    self.panels[i + 1].cfg.min,
                    self.panels[i + 1].cfg.max,
                );
                // Drags only occur between two flex panels (fixed bands are non-resizable),
                // so the redistribution space is the flex content, not the full main axis.
                apply_drag(&mut state, i, delta, flex_content, bounds);
            }
        }

        // ── Apply a collapse toggle: prefer the right neighbor if collapsible, else left ──
        if let Some(i) = toggle {
            let target = if self.panels[i + 1].cfg.collapsible {
                Some(i + 1)
            } else if self.panels[i].cfg.collapsible {
                Some(i)
            } else {
                None
            };
            if let Some(t) = target {
                state.collapsed[t] = !state.collapsed[t];
            }
        }

        ui.data_mut(|d| d.insert_temp(id, state));
        SplitterLayout { rects, response }
    }
}

/// Output of [`Splitter::layout`] — one content rect per panel (`None` if collapsed), plus the
/// splitter's own [`Response`].
pub struct SplitterLayout {
    pub rects: Vec<Option<Rect>>,
    pub response: Response,
}

/// Initial fractions over the **flex** panels: honour explicit `size`, split the remainder
/// equally among the rest. Fixed bands get fraction `0` (they're sized in px, not by fraction).
fn init_fracs(panels: &[Panel<'_>]) -> Vec<f32> {
    let n = panels.len();
    let is_flex = |p: &Panel<'_>| p.cfg.fixed.is_none();
    let explicit: f32 = panels
        .iter()
        .filter(|p| is_flex(p))
        .filter_map(|p| p.cfg.size)
        .sum();
    let unset = panels
        .iter()
        .filter(|p| is_flex(p) && p.cfg.size.is_none())
        .count();
    let remainder = (1.0 - explicit).max(0.0);
    let each = if unset > 0 {
        remainder / unset as f32
    } else {
        0.0
    };
    let raw: Vec<f32> = panels
        .iter()
        .map(|p| {
            if p.cfg.fixed.is_some() {
                0.0
            } else {
                p.cfg.size.unwrap_or(each)
            }
        })
        .collect();
    let sum: f32 = raw.iter().sum();
    if sum > f32::EPSILON {
        raw.iter().map(|f| f / sum).collect()
    } else {
        vec![1.0 / n as f32; n]
    }
}

/// Move the boundary between panel `i` and `i+1` by `delta` px, clamped to both panels'
/// `[min, max]` bounds. `bounds` is `(min_a, max_a, min_b, max_b)`.
fn apply_drag(
    state: &mut SplitterState,
    i: usize,
    delta: f32,
    content_main: f32,
    bounds: (f32, f32, f32, f32),
) {
    if content_main <= f32::EPSILON {
        return;
    }
    let (min_a, max_a, min_b, max_b) = bounds;
    let total: f32 = state.fracs.iter().sum();
    let to_px = |f: f32| f / total * content_main;
    let to_frac = |px: f32| px / content_main * total;

    let a = to_px(state.fracs[i]);
    let b = to_px(state.fracs[i + 1]);
    let combined = a + b;

    // Allowed range for `a` so both panels honour their bounds.
    let a_lo = min_a.max(combined - max_b);
    let a_hi = max_a.min(combined - min_b);
    if a_lo > a_hi {
        return; // bounds can't be satisfied together — leave as is.
    }
    let a_new = (a + delta).clamp(a_lo, a_hi);

    state.fracs[i] = to_frac(a_new);
    state.fracs[i + 1] = to_frac(combined - a_new);
}
