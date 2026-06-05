//! Storybook — token gallery for ouroboros-ui.
//!
//! Renders the foundation's tokens (color swatches, spacing/radius/shadow scales, type
//! scale) so token decisions can be validated by eye. Built out in T06.

fn main() -> eframe::Result<()> {
    eframe::run_ui_native(
        "ouroboros-ui storybook",
        eframe::NativeOptions::default(),
        move |ui, _frame| {
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.heading("ouroboros-ui storybook");
                ui.label("Token gallery — built out in T06.");
            });
        },
    )
}
