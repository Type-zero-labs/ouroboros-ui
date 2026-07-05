//! Controls — a floating zoom/fit cluster overlaid on the canvas (screen space).
//!
//! Compose tier: a [`Surface`] holding DS [`Button`]s. Drawn after the Scene, in the canvas's
//! bottom-left corner, so it floats above the graph. It mutates nothing itself — it returns the
//! requested action for [`GraphView`](super::GraphView) to apply to the `scene_rect`.

use egui::{Area, Order, Rect, Vec2};

use crate::atoms::{Button, Surface, Text};
use crate::tokens::core;

/// What the user asked the controls to do this frame.
#[derive(Clone, Copy, Debug, Default)]
pub struct ControlsAction {
    pub zoom_in: bool,
    pub zoom_out: bool,
    pub fit: bool,
}

/// Draw the control cluster anchored to the bottom-left of `canvas`, reporting the current zoom
/// `percent` (for the readout). Drawn in a foreground [`Area`] so it sits **above** the Scene
/// sublayer and actually receives clicks. Returns the requested action.
pub(crate) fn show(ui: &mut egui::Ui, canvas: Rect, percent: i32) -> ControlsAction {
    let mut action = ControlsAction::default();
    let pos = canvas.left_bottom() + Vec2::new(core::SPACE_3, -core::SPACE_3 - core::CONTROL_LG);

    Area::new(ui.id().with("graph_controls"))
        .order(Order::Foreground)
        .fixed_pos(pos)
        .show(ui.ctx(), |ui| {
            Surface::new().elevated().pad(core::SPACE_1).show(ui, |ui| {
                ui.horizontal(|ui| {
                    if icon_btn(ui, egui_phosphor::light::MINUS, "graph_zoom_out") {
                        action.zoom_out = true;
                    }
                    Text::new(format!("{percent}%")).caption().muted().show(ui);
                    if icon_btn(ui, egui_phosphor::light::PLUS, "graph_zoom_in") {
                        action.zoom_in = true;
                    }
                    if icon_btn(ui, egui_phosphor::light::CORNERS_OUT, "graph_fit") {
                        action.fit = true;
                    }
                });
            });
        });

    action
}

fn icon_btn(ui: &mut egui::Ui, glyph: &'static str, id: &'static str) -> bool {
    Button::new("")
        .ghost()
        .sm()
        .icon_only()
        .icon_left(glyph)
        .id_source(id)
        .show(ui)
        .clicked()
}
