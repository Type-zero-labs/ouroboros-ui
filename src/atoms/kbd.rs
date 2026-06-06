//! Kbd atom — a keyboard-key chip. [shadcn Kbd]
//!
//! Mono text in a small token-bordered box (e.g. `⌘K`, `Ctrl`, `Esc`).

use crate::theme::typography;
use crate::tokens::core;
use crate::Theme;
use egui::{
    pos2, text::LayoutJob, text::TextFormat, vec2, CornerRadius, Response, Sense, Stroke,
    StrokeKind, Ui,
};

/// A keyboard key chip. Builder; `show` returns the [`Response`].
pub struct Kbd {
    keys: String,
}

impl Kbd {
    pub fn new(keys: impl Into<String>) -> Self {
        Self { keys: keys.into() }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let style = typography::kbd();
        let pad = vec2(core::SPACE_2, core::SPACE_1);
        let mut job = LayoutJob::default();
        job.wrap.max_width = f32::INFINITY;
        job.append(
            &self.keys,
            0.0,
            TextFormat {
                font_id: style.font_id(),
                color: theme.muted_foreground,
                extra_letter_spacing: style.tracking,
                ..Default::default()
            },
        );
        let galley = ui.painter().layout_job(job);
        let size = galley.size() + 2.0 * pad;
        let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
        let radius = CornerRadius::same(core::RADIUS_SM as u8);
        let painter = ui.painter();
        painter.rect_filled(rect, radius, theme.muted);
        painter.rect_stroke(
            rect,
            radius,
            Stroke::new(core::BORDER_THIN, theme.border),
            StrokeKind::Inside,
        );
        painter.galley(
            pos2(
                rect.center().x - galley.size().x / 2.0,
                rect.center().y - galley.size().y / 2.0,
            ),
            galley,
            theme.muted_foreground,
        );
        response
    }
}
