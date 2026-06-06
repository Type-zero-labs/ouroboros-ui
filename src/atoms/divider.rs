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
}

impl Divider {
    pub fn horizontal() -> Self {
        Self {
            axis: Axis::Horizontal,
            color: None,
            destructive: false,
        }
    }
    pub fn vertical() -> Self {
        Self {
            axis: Axis::Vertical,
            color: None,
            destructive: false,
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

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let color = self.color.unwrap_or(if self.destructive {
            theme.destructive
        } else {
            theme.border
        });
        let stroke = Stroke::new(core::BORDER_THIN, color);
        match self.axis {
            Axis::Horizontal => {
                let width = ui.available_width();
                let (rect, resp) =
                    ui.allocate_exact_size(vec2(width, core::BORDER_THIN), Sense::hover());
                ui.painter().hline(rect.x_range(), rect.center().y, stroke);
                resp
            }
            Axis::Vertical => {
                let height = ui.available_height();
                let (rect, resp) =
                    ui.allocate_exact_size(vec2(core::BORDER_THIN, height), Sense::hover());
                ui.painter().vline(rect.center().x, rect.y_range(), stroke);
                resp
            }
        }
    }
}
