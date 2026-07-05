//! Divider atom — a hairline rule in the `border` token, horizontal or vertical.
//!
//! Thickness is `core::BORDER_THIN`; color defaults to the `border` token (`.destructive()`
//! / `.color()` to override). Horizontal fills the available width; vertical the height.

use crate::tokens::core;
use crate::Theme;
use egui::{vec2, Color32, Response, Sense, Stroke, Ui};

/// Orientation of a [`Divider`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Axis {
    #[default]
    Horizontal,
    Vertical,
}

/// A hairline rule. Builder; `show` returns the [`Response`].
pub struct Divider {
    axis: Axis,
    color: Option<Color32>,
    destructive: bool,
    weight: f32,
    dotted: bool,
}

impl Divider {
    pub fn horizontal() -> Self {
        Self {
            axis: Axis::Horizontal,
            color: None,
            destructive: false,
            weight: core::BORDER_THIN,
            dotted: false,
        }
    }
    pub fn vertical() -> Self {
        Self {
            axis: Axis::Vertical,
            color: None,
            destructive: false,
            weight: core::BORDER_THIN,
            dotted: false,
        }
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
    /// Use the `destructive` token (the "red rule").
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
    /// Thicker rule (`BORDER_FOCUS`) — e.g. a tab underline indicator.
    pub fn thick(mut self) -> Self {
        self.weight = core::BORDER_FOCUS;
        self
    }
    /// Render as a dotted/dashed rule instead of a solid hairline. Same color/weight tokens.
    pub fn dotted(mut self) -> Self {
        self.dotted = true;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let color = self.color.unwrap_or(if self.destructive {
            theme.destructive
        } else {
            theme.border
        });
        let stroke = Stroke::new(self.weight, color);
        // Dash period: one `SPACE_1` mark, one `SPACE_1` gap.
        let dash = core::SPACE_1;
        match self.axis {
            Axis::Horizontal => {
                let width = ui.available_width();
                let (rect, resp) = ui.allocate_exact_size(vec2(width, self.weight), Sense::hover());
                let y = rect.center().y;
                if self.dotted {
                    let mut x = rect.left();
                    while x < rect.right() {
                        let x2 = (x + dash).min(rect.right());
                        ui.painter().hline(egui::Rangef::new(x, x2), y, stroke);
                        x += dash * 2.0;
                    }
                } else {
                    ui.painter().hline(rect.x_range(), y, stroke);
                }
                resp
            }
            Axis::Vertical => {
                let height = ui.available_height();
                let (rect, resp) =
                    ui.allocate_exact_size(vec2(self.weight, height), Sense::hover());
                let x = rect.center().x;
                if self.dotted {
                    let mut y = rect.top();
                    while y < rect.bottom() {
                        let y2 = (y + dash).min(rect.bottom());
                        ui.painter().vline(x, egui::Rangef::new(y, y2), stroke);
                        y += dash * 2.0;
                    }
                } else {
                    ui.painter().vline(x, rect.y_range(), stroke);
                }
                resp
            }
        }
    }
}
