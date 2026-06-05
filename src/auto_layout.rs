//! Auto-layout — Figma-style flow layout for egui.
//!
//! Mirrors the **exact vocabulary** of the studio's HUD model
//! (`ouroboros-hud::model`) — `LayoutDirection`, `MainAlign`, `CrossAlign`, `Gap`,
//! `Padding`, `SizeMode` — so designers get one mental model across the engine HUD and
//! the studio's own UI. Re-declared here (not a dependency) to keep `ouroboros-ui`
//! standalone; the engine crate stays source-of-truth for the serialized game HUD.
//!
//! Children are a list, each with its own [`SizeMode`] on the main axis: `Hug` (size to
//! content), `Fill` (share leftover space), `Fixed(px)`. [`Gap::Auto`] distributes
//! leftover space between children (space-between). Child closures are `FnMut` — they
//! run once invisibly to measure, then once for real.
//!
//! ```ignore
//! AutoLayout::horizontal()
//!     .gap(8.0).pad(12.0).cross_align(CrossAlign::Center)
//!     .fixed(28.0, |ui| icon(ui))
//!     .fill(|ui| {})            // spacer pushes the next child to the end
//!     .hug(|ui| button(ui))
//!     .show(ui);
//! ```

use egui::{pos2, vec2, Rect, Response, Sense, Ui, UiBuilder, Vec2};

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

struct Child<'a> {
    main: SizeMode,
    add: Box<dyn FnMut(&mut Ui) + 'a>,
}

/// A Figma-style auto-layout frame. Build with [`AutoLayout::horizontal`] /
/// [`AutoLayout::vertical`], add children, then [`AutoLayout::show`].
pub struct AutoLayout<'a> {
    direction: LayoutDirection,
    gap: Gap,
    padding: Padding,
    main_align: MainAlign,
    cross_align: CrossAlign,
    children: Vec<Child<'a>>,
}

impl<'a> AutoLayout<'a> {
    fn new(direction: LayoutDirection) -> Self {
        Self {
            direction,
            gap: Gap::default(),
            padding: Padding::default(),
            main_align: MainAlign::default(),
            cross_align: CrossAlign::default(),
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

    /// Add a child with an explicit main-axis [`SizeMode`].
    pub fn child(mut self, main: SizeMode, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.children.push(Child {
            main,
            add: Box::new(add),
        });
        self
    }
    /// Child sized to its content.
    pub fn hug(self, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.child(SizeMode::Hug, add)
    }
    /// Child that grows to share leftover space (also a flexible spacer when empty).
    pub fn fill(self, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.child(SizeMode::Fill, add)
    }
    /// Child with a fixed main-axis size.
    pub fn fixed(self, px: f32, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.child(SizeMode::Fixed(px), add)
    }

    /// Lay out and render. Returns the frame's [`Response`].
    pub fn show(mut self, ui: &mut Ui) -> Response {
        let dir = self.direction;
        let n = self.children.len();
        let pad_x = self.padding.left + self.padding.right;
        let pad_y = self.padding.top + self.padding.bottom;

        let avail = ui.available_size();
        let inner_avail = vec2((avail.x - pad_x).max(0.0), (avail.y - pad_y).max(0.0));
        let cross_bound = dir.cross(inner_avail);

        // ── Measure pass: natural main + cross of each child ──
        let mut main_nat = vec![0.0f32; n];
        let mut cross_nat = vec![0.0f32; n];
        let mut fill_count = 0usize;
        for (i, child) in self.children.iter_mut().enumerate() {
            let measured = measure(ui, dir, cross_bound, &mut child.add);
            cross_nat[i] = dir
                .cross(measured)
                .min(cross_bound.max(dir.cross(measured)));
            main_nat[i] = match child.main {
                SizeMode::Fixed(v) => v,
                SizeMode::Hug => dir.main(measured),
                SizeMode::Fill => {
                    fill_count += 1;
                    0.0
                }
            };
        }

        let fixed_gap = match self.gap {
            Gap::Fixed(g) => g,
            Gap::Auto => 0.0,
        };
        let content_main: f32 = main_nat.iter().sum::<f32>()
            + if n > 1 {
                fixed_gap * (n as f32 - 1.0)
            } else {
                0.0
            };
        let content_cross = cross_nat.iter().cloned().fold(0.0_f32, f32::max);

        // ── Container sizing ──
        let needs_avail =
            self.gap == Gap::Auto || fill_count > 0 || self.main_align != MainAlign::Start;
        let main_size = if needs_avail {
            content_main.max(dir.main(inner_avail))
        } else {
            content_main
        };
        let cross_size = content_cross;
        let leftover = (main_size - content_main).max(0.0);

        // ── Distribution ──
        let (start_offset, between_extra, fill_share) = if self.gap == Gap::Auto && n > 1 {
            (0.0, leftover / (n as f32 - 1.0), 0.0)
        } else if fill_count > 0 {
            (0.0, 0.0, leftover / fill_count as f32)
        } else {
            (leftover * self.main_align.factor(), 0.0, 0.0)
        };

        // ── Allocate the frame, then render children at explicit cells ──
        let outer = dir.rect(0.0, 0.0, main_size, cross_size).size() + vec2(pad_x, pad_y);
        let (rect, response) = ui.allocate_exact_size(outer, Sense::hover());
        let inner_min = rect.min + vec2(self.padding.left, self.padding.top);
        let inner_main_min = dir.main(inner_min.to_vec2());
        let inner_cross_min = dir.cross(inner_min.to_vec2());

        let mut cursor = start_offset;
        for (i, child) in self.children.iter_mut().enumerate() {
            let main_ext = match child.main {
                SizeMode::Fill => fill_share,
                _ => main_nat[i],
            };
            let cross_ext = cross_nat[i].min(cross_size);
            let cross_off = self.cross_align.offset((cross_size - cross_ext).max(0.0));
            let cell = dir.rect(
                inner_main_min + cursor,
                inner_cross_min + cross_off,
                main_ext,
                cross_ext.max(0.0),
            );
            let mut cui = ui.new_child(UiBuilder::new().max_rect(cell));
            (child.add)(&mut cui);
            cursor += main_ext + fixed_gap + between_extra;
        }

        response
    }
}

/// Measure a child's natural size by rendering it once into an invisible sizing-pass ui.
fn measure(
    ui: &mut Ui,
    dir: LayoutDirection,
    cross_bound: f32,
    add: &mut dyn FnMut(&mut Ui),
) -> Vec2 {
    // Large on the main axis (don't constrain Hug), bounded on the cross axis.
    let big = dir.rect(0.0, 0.0, 100_000.0, cross_bound.max(1.0)).size();
    let max_rect = Rect::from_min_size(ui.next_widget_position(), big);
    let mut child = ui.new_child(
        UiBuilder::new()
            .invisible()
            .sizing_pass()
            .max_rect(max_rect),
    );
    add(&mut child);
    child.min_rect().size()
}
