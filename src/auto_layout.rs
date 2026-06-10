//! Auto-layout — Figma-style flow layout for egui.
//!
//! Mirrors the **exact vocabulary** of the studio's HUD model
//! (`ouroboros-hud::model`) — `LayoutDirection`, `MainAlign`, `CrossAlign`, `Gap`,
//! `Padding`, `SizeMode`, `Sizing` — so designers get one mental model across the engine
//! HUD and the studio's own UI. Re-declared here (not a dependency) to keep `ouroboros-ui`
//! standalone; the engine crate stays source-of-truth for the serialized game HUD.
//!
//! Children are a list, each with its own [`Sizing`] on the main axis: `Hug` (size to
//! content), `Fill` (share leftover space), `Fixed(px)` — optionally clamped by
//! `min`/`max`. [`Gap::Auto`] distributes leftover space between children
//! (space-between). Child closures are `FnMut` — they run once invisibly to measure,
//! then once for real.
//!
//! ## Responsive contract
//!
//! The frame never exceeds the available space (its budget comes from the parent —
//! a `Splitter` panel rect, a window — which is *exogenous to the content*, so layout
//! is idempotent per frame: resizing a panel out and back yields the same rects, with
//! no ratchet). Measurement is bounded by that budget, leftover is distributed among
//! `Fill` children respecting `min`/`max` (with redistribution when one clamps), and
//! cells are clipped as a last resort so content never paints over a sibling. Opt out
//! with [`AutoLayout::allow_overflow`].
//!
//! `Hug` measures content against the budget: a greedy child (one that expands to
//! `available_width`) measures *as the whole budget* — for controls that should fill,
//! use `Fill` (optionally with `min`/`max`) instead of `Hug`.
//!
//! ## Wrap
//!
//! [`AutoLayout::wrap`] (horizontal only) reflows children onto new lines when they
//! don't fit — Figma's "wrap". A `Fill` child takes the remainder of *its* line. Line
//! spacing comes from [`AutoLayout::gap_cross`] (defaults to the main gap).
//!
//! ```ignore
//! AutoLayout::horizontal()
//!     .gap(8.0).pad(12.0).cross_align(CrossAlign::Center)
//!     .fixed(28.0, |ui| icon(ui))
//!     .fill(|ui| {})            // spacer pushes the next child to the end
//!     .hug(|ui| button(ui))
//!     .show(ui);
//!
//! // Responsive two-column form: each column floors at 220px.
//! AutoLayout::horizontal()
//!     .gap(24.0)
//!     .fill_min(220.0, |ui| left_column(ui))
//!     .fill_min(220.0, |ui| right_column(ui))
//!     .show(ui);
//!
//! // Stat grid that reflows: one row when wide, 2×3 when narrow.
//! AutoLayout::horizontal().wrap().gap(8.0)
//!     .fill_min(72.0, |ui| stat(ui, "STR"))
//!     .fill_min(72.0, |ui| stat(ui, "AGI"))
//!     // ...
//!     .show(ui);
//! ```

use egui::{pos2, vec2, Rect, Response, Sense, Ui, UiBuilder, Vec2};

/// Main-axis size used to measure children when the parent gives no finite budget
/// (e.g. inside a `ScrollArea` scrolling on this axis): effectively unbounded.
const MEASURE_UNBOUNDED: f32 = 100_000.0;

/// Extra clip bleed around a cell so focus rings / 1px strokes painted just outside a
/// widget's rect aren't guillotined.
const CLIP_BLEED: f32 = 2.0;

/// Primary axis children flow along.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LayoutDirection {
    Horizontal,
    #[default]
    Vertical,
}

impl LayoutDirection {
    fn main(self, v: Vec2) -> f32 {
        match self {
            LayoutDirection::Horizontal => v.x,
            LayoutDirection::Vertical => v.y,
        }
    }
    fn cross(self, v: Vec2) -> f32 {
        match self {
            LayoutDirection::Horizontal => v.y,
            LayoutDirection::Vertical => v.x,
        }
    }
    /// Build a rect from main/cross min + extents in this direction's coordinates.
    fn rect(self, main_min: f32, cross_min: f32, main_ext: f32, cross_ext: f32) -> Rect {
        match self {
            LayoutDirection::Horizontal => {
                Rect::from_min_size(pos2(main_min, cross_min), vec2(main_ext, cross_ext))
            }
            LayoutDirection::Vertical => {
                Rect::from_min_size(pos2(cross_min, main_min), vec2(cross_ext, main_ext))
            }
        }
    }
}

/// Alignment of the child block on the primary axis (ignored when [`Gap::Auto`] or any
/// `Fill` child consumes the leftover space).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MainAlign {
    #[default]
    Start,
    Center,
    End,
}

impl MainAlign {
    fn factor(self) -> f32 {
        match self {
            MainAlign::Start => 0.0,
            MainAlign::Center => 0.5,
            MainAlign::End => 1.0,
        }
    }
}

/// Alignment of each child on the cross axis.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CrossAlign {
    #[default]
    Start,
    Center,
    End,
}

impl CrossAlign {
    fn offset(self, free: f32) -> f32 {
        match self {
            CrossAlign::Start => 0.0,
            CrossAlign::Center => free * 0.5,
            CrossAlign::End => free,
        }
    }
}

/// Distance between children. `Auto` distributes leftover space evenly (space-between),
/// ignoring [`MainAlign`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Gap {
    Fixed(f32),
    Auto,
}

impl Default for Gap {
    fn default() -> Self {
        Gap::Fixed(0.0)
    }
}

/// Inner spacing between the frame edge and its children.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Padding {
    pub const fn all(v: f32) -> Self {
        Self {
            top: v,
            right: v,
            bottom: v,
            left: v,
        }
    }
    /// Horizontal (left/right) + vertical (top/bottom).
    pub const fn symmetric(x: f32, y: f32) -> Self {
        Self {
            top: y,
            right: x,
            bottom: y,
            left: x,
        }
    }
}

/// How a child is sized along the main axis.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SizeMode {
    /// Fixed size in px.
    Fixed(f32),
    /// Size to content.
    #[default]
    Hug,
    /// Grow to share leftover main-axis space.
    Fill,
}

/// Main-axis sizing of a child: a [`SizeMode`] plus optional `min`/`max` clamps.
/// Mirrors `ouroboros-hud::model::Sizing` (same names, no dependency).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Sizing {
    pub mode: SizeMode,
    pub min: Option<f32>,
    pub max: Option<f32>,
}

impl Sizing {
    pub const fn fixed(px: f32) -> Self {
        Self {
            mode: SizeMode::Fixed(px),
            min: None,
            max: None,
        }
    }
    pub const fn hug() -> Self {
        Self {
            mode: SizeMode::Hug,
            min: None,
            max: None,
        }
    }
    pub const fn fill() -> Self {
        Self {
            mode: SizeMode::Fill,
            min: None,
            max: None,
        }
    }
    /// Floor (px) on the main axis.
    pub const fn min(mut self, px: f32) -> Self {
        self.min = Some(px);
        self
    }
    /// Ceiling (px) on the main axis.
    pub const fn max(mut self, px: f32) -> Self {
        self.max = Some(px);
        self
    }
    /// Floor + ceiling (px) on the main axis.
    pub const fn clamped(mut self, min: f32, max: f32) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }
}

impl From<SizeMode> for Sizing {
    fn from(mode: SizeMode) -> Self {
        Self {
            mode,
            min: None,
            max: None,
        }
    }
}

/// Clamp `v` by optional bounds (min wins over max, like the HUD solver).
fn clamp_opt(v: f32, min: Option<f32>, max: Option<f32>) -> f32 {
    let mut v = v;
    if let Some(mx) = max {
        v = v.min(mx);
    }
    if let Some(mn) = min {
        v = v.max(mn);
    }
    v
}

struct Child<'a> {
    sizing: Sizing,
    add: Box<dyn FnMut(&mut Ui) + 'a>,
}

/// A Figma-style auto-layout frame. Build with [`AutoLayout::horizontal`] /
/// [`AutoLayout::vertical`], add children, then [`AutoLayout::show`].
pub struct AutoLayout<'a> {
    direction: LayoutDirection,
    gap: Gap,
    gap_cross: Option<f32>,
    padding: Padding,
    main_align: MainAlign,
    cross_align: CrossAlign,
    wrap: bool,
    allow_overflow: bool,
    children: Vec<Child<'a>>,
}

impl<'a> AutoLayout<'a> {
    fn new(direction: LayoutDirection) -> Self {
        Self {
            direction,
            gap: Gap::default(),
            gap_cross: None,
            padding: Padding::default(),
            main_align: MainAlign::default(),
            cross_align: CrossAlign::default(),
            wrap: false,
            allow_overflow: false,
            children: Vec::new(),
        }
    }

    pub fn horizontal() -> Self {
        Self::new(LayoutDirection::Horizontal)
    }
    pub fn vertical() -> Self {
        Self::new(LayoutDirection::Vertical)
    }

    pub fn gap(mut self, px: f32) -> Self {
        self.gap = Gap::Fixed(px);
        self
    }
    /// Space-between: distribute leftover space evenly between children.
    pub fn gap_auto(mut self) -> Self {
        self.gap = Gap::Auto;
        self
    }
    /// Gap between wrapped lines (defaults to the main gap when `Fixed`, else 0).
    pub fn gap_cross(mut self, px: f32) -> Self {
        self.gap_cross = Some(px);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }
    pub fn pad(mut self, v: f32) -> Self {
        self.padding = Padding::all(v);
        self
    }
    pub fn pad_xy(mut self, x: f32, y: f32) -> Self {
        self.padding = Padding::symmetric(x, y);
        self
    }

    pub fn main_align(mut self, a: MainAlign) -> Self {
        self.main_align = a;
        self
    }
    pub fn cross_align(mut self, a: CrossAlign) -> Self {
        self.cross_align = a;
        self
    }

    /// Reflow children onto new lines when they don't fit (Figma wrap). Horizontal only;
    /// a `Fill` child takes the remainder of its line. Not supported by the rect-returning
    /// [`AutoLayout::layout`] path.
    pub fn wrap(mut self) -> Self {
        debug_assert!(
            self.direction == LayoutDirection::Horizontal,
            "AutoLayout::wrap is horizontal-only (v1)"
        );
        self.wrap = true;
        self
    }

    /// Let the frame exceed the available space instead of clamping + clipping —
    /// restores the legacy overflow behavior for the rare container that scrolls itself.
    pub fn allow_overflow(mut self) -> Self {
        self.allow_overflow = true;
        self
    }

    /// Add a child with an explicit main-axis [`Sizing`] (a bare [`SizeMode`] converts).
    pub fn child(mut self, main: impl Into<Sizing>, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.children.push(Child {
            sizing: main.into(),
            add: Box::new(add),
        });
        self
    }
    /// Child sized to its content.
    pub fn hug(self, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.child(Sizing::hug(), add)
    }
    /// Child sized to its content, capped at `max` px.
    pub fn hug_max(self, max: f32, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.child(Sizing::hug().max(max), add)
    }
    /// Child that grows to share leftover space (also a flexible spacer when empty).
    pub fn fill(self, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.child(Sizing::fill(), add)
    }
    /// Filling child that never shrinks below `min` px.
    pub fn fill_min(self, min: f32, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.child(Sizing::fill().min(min), add)
    }
    /// Filling child clamped to `[min, max]` px.
    pub fn fill_clamped(self, min: f32, max: f32, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.child(Sizing::fill().clamped(min, max), add)
    }
    /// Child with a fixed main-axis size.
    pub fn fixed(self, px: f32, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.child(Sizing::fixed(px), add)
    }
    /// Child with an explicit [`Sizing`] (alias of [`AutoLayout::child`] for call sites
    /// that build the sizing separately).
    pub fn sized(self, s: Sizing, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.child(s, add)
    }

    /// Add a child with a main-axis [`Sizing`] but **no content closure**, for use with
    /// [`AutoLayout::layout`]. Use this when sibling cells each need `&mut self` of the caller —
    /// `FnMut` closures can't each borrow the same state, so lay out, get the rects back, and draw
    /// into them sequentially via `ui.new_child`.
    pub fn region(mut self, main: impl Into<Sizing>) -> Self {
        self.children.push(Child {
            sizing: main.into(),
            add: Box::new(|_| {}),
        });
        self
    }

    /// Lay out and return one rect per child **instead of** rendering content closures. Each cell
    /// spans the full cross axis; on the main axis `Fixed(px)` reserves its px (clamped by
    /// min/max), `Fill` shares the remainder (min/max-aware, with redistribution), and `Hug` is
    /// treated as `Fill` (content size can't be measured without a closure — use
    /// [`AutoLayout::show`] when you need `Hug`). Pair with [`AutoLayout::region`].
    ///
    /// This path allocates the whole `available_size` — it is meant for the root regions of a
    /// panel, not for flowing content inside a scroll area. `wrap` is not supported here.
    pub fn layout(self, ui: &mut Ui) -> AutoLayoutLayout {
        debug_assert!(!self.wrap, "AutoLayout::wrap is not supported by layout()");
        let dir = self.direction;
        let n = self.children.len();
        let pad_x = self.padding.left + self.padding.right;
        let pad_y = self.padding.top + self.padding.bottom;

        let avail = ui.available_size();
        let inner = vec2((avail.x - pad_x).max(0.0), (avail.y - pad_y).max(0.0));
        let inner_main = dir.main(inner);
        let inner_cross = dir.cross(inner);

        let fixed_gap = match self.gap {
            Gap::Fixed(g) => g,
            Gap::Auto => 0.0,
        };
        let gap_total = if n > 1 {
            fixed_gap * (n as f32 - 1.0)
        } else {
            0.0
        };
        let fixed_total: f32 = self
            .children
            .iter()
            .filter_map(|c| match c.sizing.mode {
                SizeMode::Fixed(v) => Some(clamp_opt(v, c.sizing.min, c.sizing.max)),
                _ => None,
            })
            .sum();
        // Hug has no closure to measure here — treated as Fill (documented above).
        let flex_sizings: Vec<Sizing> = self
            .children
            .iter()
            .filter(|c| !matches!(c.sizing.mode, SizeMode::Fixed(_)))
            .map(|c| c.sizing)
            .collect();
        let leftover = (inner_main - fixed_total - gap_total).max(0.0);
        let flex_resolved = distribute_fill(&flex_sizings, leftover);

        let mut flex_iter = flex_resolved.iter();
        let main_extent: Vec<f32> = self
            .children
            .iter()
            .map(|c| match c.sizing.mode {
                SizeMode::Fixed(v) => clamp_opt(v, c.sizing.min, c.sizing.max),
                _ => *flex_iter.next().expect("one resolved size per flex child"),
            })
            .collect();
        let content_main: f32 = main_extent.iter().sum::<f32>() + gap_total;
        let free = (inner_main - content_main).max(0.0);

        // Distribution: Auto = space-between; with flex children the leftover is already consumed;
        // otherwise honour main_align over the free space.
        let (start_offset, between_extra) = if self.gap == Gap::Auto && n > 1 {
            (0.0, free / (n as f32 - 1.0))
        } else if !flex_sizings.is_empty() {
            (0.0, 0.0)
        } else {
            (free * self.main_align.factor(), 0.0)
        };

        let (rect, response) = ui.allocate_exact_size(avail, Sense::hover());
        let inner_min = rect.min + vec2(self.padding.left, self.padding.top);
        let inner_main_min = dir.main(inner_min.to_vec2());
        let inner_cross_min = dir.cross(inner_min.to_vec2());

        let mut cursor = start_offset;
        let mut rects = Vec::with_capacity(n);
        for ext in &main_extent {
            rects.push(dir.rect(inner_main_min + cursor, inner_cross_min, *ext, inner_cross));
            cursor += ext + fixed_gap + between_extra;
        }

        AutoLayoutLayout { rects, response }
    }

    /// Lay out and render. Returns the frame's [`Response`].
    pub fn show(mut self, ui: &mut Ui) -> Response {
        if self.wrap && self.direction == LayoutDirection::Horizontal {
            return self.show_wrapped(ui);
        }
        let dir = self.direction;
        let n = self.children.len();
        let pad_x = self.padding.left + self.padding.right;
        let pad_y = self.padding.top + self.padding.bottom;

        let avail = ui.available_size();
        let inner_avail = vec2((avail.x - pad_x).max(0.0), (avail.y - pad_y).max(0.0));
        let inner_main = dir.main(inner_avail);
        let inner_cross = dir.cross(inner_avail);
        // Only the WIDTH axis is a real budget in egui: panels constrain x, while y is
        // hug/scroll territory — `available_height()` deep inside a vertical scroll can
        // be ~0 (below the fold) and must never crush cells. So each axis is bounded
        // only when it is the x axis with a sane finite value; the panel-width budget
        // is exogenous to content (Splitter rect), keeping measurement idempotent.
        let main_is_width = self.direction == LayoutDirection::Horizontal;
        let main_budget = if main_is_width && inner_main.is_finite() && inner_main >= 1.0 {
            inner_main
        } else {
            MEASURE_UNBOUNDED
        };
        let cross_bound = if !main_is_width && inner_cross.is_finite() && inner_cross >= 1.0 {
            inner_cross
        } else {
            MEASURE_UNBOUNDED
        };

        // ── Measure pass A: natural main + cross of Fixed/Hug children, bounded ──
        let mut main_nat = vec![0.0f32; n];
        let mut cross_nat = vec![0.0f32; n];
        let mut fill_sizings: Vec<Sizing> = Vec::new();
        for (i, child) in self.children.iter_mut().enumerate() {
            match child.sizing.mode {
                SizeMode::Fixed(v) => {
                    let main = clamp_opt(v, child.sizing.min, child.sizing.max);
                    let measured = measure(ui, dir, main, cross_bound, &mut child.add);
                    main_nat[i] = main;
                    cross_nat[i] = dir.cross(measured).min(cross_bound);
                }
                SizeMode::Hug => {
                    let measured = measure(ui, dir, main_budget, cross_bound, &mut child.add);
                    main_nat[i] = clamp_opt(dir.main(measured), child.sizing.min, child.sizing.max);
                    cross_nat[i] = dir.cross(measured).min(cross_bound);
                }
                SizeMode::Fill => {
                    fill_sizings.push(child.sizing);
                    // Resolved below; cross measured in pass B at the resolved width.
                }
            }
        }

        let fixed_gap = match self.gap {
            Gap::Fixed(g) => g,
            Gap::Auto => 0.0,
        };
        let gap_total = if n > 1 {
            fixed_gap * (n as f32 - 1.0)
        } else {
            0.0
        };
        let fixed_hug_main: f32 = main_nat.iter().sum();

        // ── Resolve Fill sizes: distribute the leftover, min/max-aware ──
        let free_for_fill = if inner_main.is_finite() {
            (inner_main - fixed_hug_main - gap_total).max(0.0)
        } else {
            // No finite budget (scroll axis): Fill has nothing to fill — floors only.
            0.0
        };
        let fill_resolved = distribute_fill(&fill_sizings, free_for_fill);

        // ── Measure pass B: cross of each Fill child at its *resolved* main size ──
        {
            let mut k = 0usize;
            for (i, child) in self.children.iter_mut().enumerate() {
                if matches!(child.sizing.mode, SizeMode::Fill) {
                    let main = fill_resolved[k].max(0.0);
                    main_nat[i] = main;
                    let measured = measure(ui, dir, main.max(1.0), cross_bound, &mut child.add);
                    cross_nat[i] = dir.cross(measured).min(cross_bound);
                    k += 1;
                }
            }
        }

        let content_main: f32 = main_nat.iter().sum::<f32>() + gap_total;
        let content_cross = cross_nat.iter().cloned().fold(0.0_f32, f32::max);

        // ── Container sizing: never exceed a finite WIDTH budget (unless opted out).
        // On the y axis the frame hugs its content — height overflow is the scroll's
        // job, and clamping to `available_height()` would crush flows below the fold. ──
        let needs_avail = self.gap == Gap::Auto
            || !fill_sizings.is_empty()
            || self.main_align != MainAlign::Start;
        let main_size = if needs_avail {
            if !inner_main.is_finite() {
                content_main
            } else if main_is_width && !self.allow_overflow {
                inner_main
            } else {
                content_main.max(inner_main)
            }
        } else if main_is_width && !self.allow_overflow && inner_main.is_finite() {
            content_main.min(inner_main)
        } else {
            content_main
        };
        let cross_size = content_cross;
        let leftover = (main_size - content_main).max(0.0);

        // ── Distribution ──
        let (start_offset, between_extra) = if self.gap == Gap::Auto && n > 1 {
            (0.0, leftover / (n as f32 - 1.0))
        } else if !fill_sizings.is_empty() {
            (0.0, 0.0)
        } else {
            (leftover * self.main_align.factor(), 0.0)
        };

        // ── Allocate the frame, then render children at explicit cells ──
        let outer = dir.rect(0.0, 0.0, main_size, cross_size).size() + vec2(pad_x, pad_y);
        let (rect, response) = ui.allocate_exact_size(outer, Sense::hover());
        let inner_min = rect.min + vec2(self.padding.left, self.padding.top);
        let inner_main_min = dir.main(inner_min.to_vec2());
        let inner_cross_min = dir.cross(inner_min.to_vec2());

        let mut cursor = start_offset;
        for (i, child) in self.children.iter_mut().enumerate() {
            let main_ext = main_nat[i];
            let cross_ext = cross_nat[i].min(cross_size);
            let cross_off = self.cross_align.offset((cross_size - cross_ext).max(0.0));
            let cell = dir.rect(
                inner_main_min + cursor,
                inner_cross_min + cross_off,
                main_ext,
                cross_ext.max(0.0),
            );
            let mut cui = ui.new_child(UiBuilder::new().max_rect(cell));
            if !self.allow_overflow {
                // Last-resort guard: with correct sizing the cell fits the frame and this
                // never bites; it only clips legitimate overflow (e.g. mins inside a panel
                // squeezed below its floors) instead of painting over siblings.
                cui.set_clip_rect(cell.expand(CLIP_BLEED).intersect(ui.clip_rect()));
            }
            (child.add)(&mut cui);
            cursor += main_ext + fixed_gap + between_extra;
        }

        response
    }

    /// Wrap path of [`AutoLayout::show`] — greedy line-breaking, then each line is laid
    /// out like a non-wrapping row.
    fn show_wrapped(&mut self, ui: &mut Ui) -> Response {
        let dir = self.direction;
        let n = self.children.len();
        let pad_x = self.padding.left + self.padding.right;
        let pad_y = self.padding.top + self.padding.bottom;

        let avail = ui.available_size();
        let inner_avail = vec2((avail.x - pad_x).max(0.0), (avail.y - pad_y).max(0.0));
        let inner_main = dir.main(inner_avail);
        // Wrap is horizontal-only: main = x (the real budget), cross = y (hug/scroll —
        // never bound by `available_height()`, which is ~0 below a scroll fold).
        let main_budget = if inner_main.is_finite() && inner_main >= 1.0 {
            inner_main
        } else {
            MEASURE_UNBOUNDED
        };
        let cross_bound = MEASURE_UNBOUNDED;

        let fixed_gap = match self.gap {
            Gap::Fixed(g) => g,
            Gap::Auto => 0.0,
        };
        let line_gap = self.gap_cross.unwrap_or(fixed_gap);

        // ── Measure pass A (Fixed/Hug) + break contribution per child ──
        // Break contribution: Fixed/Hug → natural main; Fill → its floor (min|0) — the
        // same intrinsic rule as the HUD solver's measure.
        let mut main_nat = vec![0.0f32; n];
        let mut cross_nat = vec![0.0f32; n];
        let mut contrib = vec![0.0f32; n];
        for (i, child) in self.children.iter_mut().enumerate() {
            match child.sizing.mode {
                SizeMode::Fixed(v) => {
                    let main = clamp_opt(v, child.sizing.min, child.sizing.max);
                    let measured = measure(ui, dir, main, cross_bound, &mut child.add);
                    main_nat[i] = main;
                    cross_nat[i] = dir.cross(measured).min(cross_bound);
                    contrib[i] = main;
                }
                SizeMode::Hug => {
                    let measured = measure(ui, dir, main_budget, cross_bound, &mut child.add);
                    main_nat[i] = clamp_opt(dir.main(measured), child.sizing.min, child.sizing.max);
                    cross_nat[i] = dir.cross(measured).min(cross_bound);
                    contrib[i] = main_nat[i];
                }
                SizeMode::Fill => {
                    contrib[i] = child.sizing.min.unwrap_or(0.0);
                }
            }
        }

        // ── Greedy line-breaking (≥1 child per line — never an infinite loop) ──
        let mut lines: Vec<std::ops::Range<usize>> = Vec::new();
        let mut start = 0usize;
        let mut cursor = 0.0f32;
        for (i, &w) in contrib.iter().enumerate() {
            let advance = if i == start { w } else { fixed_gap + w };
            if i > start && cursor + advance > main_budget {
                lines.push(start..i);
                start = i;
                cursor = w;
            } else {
                cursor += advance;
            }
        }
        if start < n {
            lines.push(start..n);
        }

        // ── Per line: resolve Fill over the line's leftover, then measure pass B ──
        for line in &lines {
            let count = line.len();
            let gaps = fixed_gap * (count.saturating_sub(1)) as f32;
            let fixed_hug: f32 = line
                .clone()
                .filter(|&i| !matches!(self.children[i].sizing.mode, SizeMode::Fill))
                .map(|i| main_nat[i])
                .sum();
            let fills: Vec<usize> = line
                .clone()
                .filter(|&i| matches!(self.children[i].sizing.mode, SizeMode::Fill))
                .collect();
            if fills.is_empty() {
                continue;
            }
            let free = if inner_main.is_finite() {
                (inner_main - fixed_hug - gaps).max(0.0)
            } else {
                0.0
            };
            let sizings: Vec<Sizing> = fills.iter().map(|&i| self.children[i].sizing).collect();
            let resolved = distribute_fill(&sizings, free);
            for (k, &i) in fills.iter().enumerate() {
                main_nat[i] = resolved[k].max(0.0);
                let child = &mut self.children[i];
                let measured = measure(ui, dir, main_nat[i].max(1.0), cross_bound, &mut child.add);
                cross_nat[i] = dir.cross(measured).min(cross_bound);
            }
        }

        // ── Container sizing: main = budget when finite (wrap is meaningless without
        // one), else hug of the widest line; cross = stacked line heights ──
        let line_main = |line: &std::ops::Range<usize>| -> f32 {
            let gaps = fixed_gap * (line.len().saturating_sub(1)) as f32;
            line.clone().map(|i| main_nat[i]).sum::<f32>() + gaps
        };
        let line_cross = |line: &std::ops::Range<usize>| -> f32 {
            line.clone().map(|i| cross_nat[i]).fold(0.0_f32, f32::max)
        };
        let widest = lines.iter().map(line_main).fold(0.0_f32, f32::max);
        let main_size = if inner_main.is_finite() {
            if self.allow_overflow {
                widest.max(inner_main)
            } else {
                inner_main
            }
        } else {
            widest
        };
        let cross_size: f32 = lines.iter().map(line_cross).sum::<f32>()
            + line_gap * (lines.len().saturating_sub(1)) as f32;

        // ── Allocate the frame, then render line by line ──
        let outer = dir.rect(0.0, 0.0, main_size, cross_size).size() + vec2(pad_x, pad_y);
        let (rect, response) = ui.allocate_exact_size(outer, Sense::hover());
        let inner_min = rect.min + vec2(self.padding.left, self.padding.top);
        let inner_main_min = dir.main(inner_min.to_vec2());
        let inner_cross_min = dir.cross(inner_min.to_vec2());

        let mut cross_cursor = 0.0f32;
        for line in &lines {
            let this_main = line_main(line);
            let this_cross = line_cross(line);
            let has_fill = line
                .clone()
                .any(|i| matches!(self.children[i].sizing.mode, SizeMode::Fill));
            let leftover = (main_size - this_main).max(0.0);
            let (start_offset, between_extra) = if self.gap == Gap::Auto && line.len() > 1 {
                (0.0, leftover / (line.len() as f32 - 1.0))
            } else if has_fill {
                (0.0, 0.0)
            } else {
                (leftover * self.main_align.factor(), 0.0)
            };

            let mut cursor = start_offset;
            for i in line.clone() {
                let main_ext = main_nat[i];
                let cross_ext = cross_nat[i].min(this_cross);
                let cross_off = self.cross_align.offset((this_cross - cross_ext).max(0.0));
                let cell = dir.rect(
                    inner_main_min + cursor,
                    inner_cross_min + cross_cursor + cross_off,
                    main_ext,
                    cross_ext.max(0.0),
                );
                let mut cui = ui.new_child(UiBuilder::new().max_rect(cell));
                if !self.allow_overflow {
                    cui.set_clip_rect(cell.expand(CLIP_BLEED).intersect(ui.clip_rect()));
                }
                (self.children[i].add)(&mut cui);
                cursor += main_ext + fixed_gap + between_extra;
            }
            cross_cursor += this_cross + line_gap;
        }

        response
    }
}

/// Output of [`AutoLayout::layout`] — one cell rect per child, plus the frame's [`Response`].
pub struct AutoLayoutLayout {
    pub rects: Vec<Rect>,
    pub response: Response,
}

/// Distribute `free` px among `Fill` children, respecting each [`Sizing`]'s min/max and
/// redistributing the excess of whoever clamps — port of the HUD solver's
/// `distribute_fill` (`ouroboros-hud/src/layout.rs`).
fn distribute_fill(sizings: &[Sizing], free: f32) -> Vec<f32> {
    let mut result = vec![0.0_f32; sizings.len()];
    let mut remaining = free;
    let mut pending: Vec<usize> = (0..sizings.len()).collect();
    while !pending.is_empty() {
        let share = remaining / pending.len() as f32;
        let mut progressed = false;
        let mut still: Vec<usize> = Vec::new();
        for &k in &pending {
            let s = sizings[k];
            let clamped = clamp_opt(share, s.min, s.max);
            if (clamped - share).abs() > f32::EPSILON {
                // Hit a bound: pin it and take it out of the split.
                result[k] = clamped;
                remaining -= clamped;
                progressed = true;
            } else {
                still.push(k);
            }
        }
        if !progressed {
            // Nobody clamped: split what's left evenly and stop.
            let share = remaining / still.len().max(1) as f32;
            for &k in &still {
                result[k] = share;
            }
            break;
        }
        pending = still;
    }
    result
}

/// Measure a child's natural size by rendering it once into an invisible sizing-pass ui,
/// bounded on **both** axes (the main bound is the container's budget — exogenous, so
/// measurement is idempotent; see the module docs).
fn measure(
    ui: &mut Ui,
    dir: LayoutDirection,
    main_bound: f32,
    cross_bound: f32,
    add: &mut dyn FnMut(&mut Ui),
) -> Vec2 {
    let big = dir
        .rect(0.0, 0.0, main_bound.max(1.0), cross_bound.max(1.0))
        .size();
    let max_rect = Rect::from_min_size(ui.next_widget_position(), big);
    // Salted id: the closure runs again for real in the same frame, so any widget with
    // an explicit id (Table, ScrollArea…) would otherwise clash with its render-pass
    // twin ("First/Second use of ID"), corrupting state and positioning. The salt keeps
    // measure-pass state in its own bucket (stateful widgets measure at default state).
    let mut child = ui.new_child(
        UiBuilder::new()
            .id_salt("__ouro_autolayout_measure")
            .invisible()
            .sizing_pass()
            .max_rect(max_rect),
    );
    add(&mut child);
    child.min_rect().size()
}
