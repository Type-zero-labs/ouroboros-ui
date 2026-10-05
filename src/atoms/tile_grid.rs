//! TileGrid atom — a texture split into `cols × rows` tiles, with a selectable tile block.
//! [Tiled / Godot TileSet picker]
//!
//! Click picks one tile; drag picks the rectangle between the press and the release tile.
//! The texture is consumer data (an atlas); grid lines, the drag preview and the selection
//! ring paint with theme tokens.

use crate::tokens::core;
use crate::Theme;
use egui::{pos2, vec2, Id, Rect, Response, Sense, Stroke, StrokeKind, TextureId, Ui, Vec2};

/// A block of tiles: top-left `(col, row)` and size `w × h` (both ≥ 1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TileRect {
    pub col: u32,
    pub row: u32,
    pub w: u32,
    pub h: u32,
}

impl TileRect {
    /// The block spanning two tiles, in any order.
    pub fn spanning(a: (u32, u32), b: (u32, u32)) -> Self {
        Self {
            col: a.0.min(b.0),
            row: a.1.min(b.1),
            w: a.0.abs_diff(b.0) + 1,
            h: a.1.abs_diff(b.1) + 1,
        }
    }
}

/// A tile picker over a texture. Builder; `show` returns the [`Response`], marked
/// `changed()` on the frame the selection is committed (click or drag release).
pub struct TileGrid<'a> {
    texture: TextureId,
    size: Vec2,
    cols: u32,
    rows: u32,
    selection: &'a mut Option<TileRect>,
    max_width: f32,
    id_salt: Id,
}

impl<'a> TileGrid<'a> {
    /// `size` is the texture's pixel size; `cols × rows` its tile grid.
    pub fn new(
        texture: TextureId,
        size: Vec2,
        cols: u32,
        rows: u32,
        selection: &'a mut Option<TileRect>,
    ) -> Self {
        Self {
            texture,
            size,
            cols: cols.max(1),
            rows: rows.max(1),
            selection,
            max_width: f32::INFINITY,
            id_salt: Id::new("tile_grid"),
        }
    }

    /// Cap the drawn width; the texture never scales up past 1:1.
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    pub fn id_salt(mut self, salt: impl std::hash::Hash) -> Self {
        self.id_salt = Id::new(salt);
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let avail = ui.available_width().min(self.max_width);
        let scale = (avail / self.size.x.max(1.0)).min(1.0);
        let draw = self.size * scale;
        let (rect, mut response) = ui.allocate_exact_size(draw, Sense::click_and_drag());
        let painter = ui.painter_at(rect);
        painter.image(
            self.texture,
            rect,
            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
            core::TINT_NONE,
        );

        let cell = vec2(draw.x / self.cols as f32, draw.y / self.rows as f32);
        let line = Stroke::new(core::BORDER_THIN, theme.scrim);
        for c in 1..self.cols {
            let x = rect.left() + cell.x * c as f32;
            painter.line_segment([pos2(x, rect.top()), pos2(x, rect.bottom())], line);
        }
        for r in 1..self.rows {
            let y = rect.top() + cell.y * r as f32;
            painter.line_segment([pos2(rect.left(), y), pos2(rect.right(), y)], line);
        }

        let (cols, rows) = (self.cols, self.rows);
        let tile_at = |p: egui::Pos2| {
            (
                (((p.x - rect.left()) / cell.x).floor().max(0.0) as u32).min(cols - 1),
                (((p.y - rect.top()) / cell.y).floor().max(0.0) as u32).min(rows - 1),
            )
        };
        let block_rect = |t: TileRect| {
            Rect::from_min_size(
                pos2(
                    rect.left() + cell.x * t.col as f32,
                    rect.top() + cell.y * t.row as f32,
                ),
                vec2(cell.x * t.w as f32, cell.y * t.h as f32),
            )
        };

        // The press tile lives in temp memory while the drag is in flight.
        let anchor_id = ui.id().with(self.id_salt).with("anchor");
        let pointer = response.interact_pointer_pos();
        if response.drag_started() {
            if let Some(p) = pointer {
                ui.data_mut(|d| d.insert_temp(anchor_id, tile_at(p)));
            }
        }
        let anchor: Option<(u32, u32)> = ui.data(|d| d.get_temp(anchor_id));
        let mut commit = None;
        if response.clicked() {
            commit = pointer.map(|p| TileRect::spanning(tile_at(p), tile_at(p)));
        } else if let (Some(a), Some(p)) = (anchor, pointer) {
            let block = TileRect::spanning(a, tile_at(p));
            if response.drag_stopped() {
                commit = Some(block);
                ui.data_mut(|d| d.remove::<(u32, u32)>(anchor_id));
            } else {
                painter.rect_stroke(
                    block_rect(block),
                    0.0,
                    Stroke::new(core::BORDER_FOCUS, theme.muted_foreground),
                    StrokeKind::Inside,
                );
            }
        }
        if let Some(block) = commit {
            *self.selection = Some(block);
            response.mark_changed();
        }
        if let Some(sel) = *self.selection {
            if sel.col < cols && sel.row < rows {
                painter.rect_stroke(
                    block_rect(sel),
                    0.0,
                    Stroke::new(core::BORDER_FOCUS, theme.ring),
                    StrokeKind::Inside,
                );
            }
        }
        response
    }
}
