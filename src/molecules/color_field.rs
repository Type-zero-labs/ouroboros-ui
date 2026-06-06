//! ColorField molecule — a color swatch + editable hex, opening a full picker. [Unity/Figma]
//!
//! The swatch opens an HSV/RGB/hex color picker in a popover (egui's `color_picker`); the inline
//! hex field is editable. Composes [`ColorSwatch`] + [`Input`] + a popover.

use crate::atoms::{ColorSwatch, Input};
use crate::tokens::core;
use egui::color_picker::{color_picker_color32, Alpha};
use egui::{Color32, Id, Response, Ui};

/// A color field bound to a `&mut Color32`. `show` returns the hex input [`Response`].
pub struct ColorField<'a> {
    color: &'a mut Color32,
    alpha: bool,
    id_source: Option<Id>,
}

impl<'a> ColorField<'a> {
    pub fn new(color: &'a mut Color32) -> Self {
        Self {
            color,
            alpha: false,
            id_source: None,
        }
    }
    /// Allow editing the alpha channel.
    pub fn alpha(mut self, alpha: bool) -> Self {
        self.alpha = alpha;
        self
    }
    pub fn id_source(mut self, id: impl std::hash::Hash) -> Self {
        self.id_source = Some(Id::new(id));
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let color = self.color;
        let alpha_mode = if self.alpha {
            Alpha::OnlyBlend
        } else {
            Alpha::Opaque
        };
        let hex_id = self.id_source.unwrap_or_else(|| Id::new("color_field"));
        ui.horizontal(|ui| {
            let swatch = ColorSwatch::new(*color).show(ui);
            ui.add_space(core::SPACE_2);

            // Editable hex (e.g. `#1ABC9C`).
            let mut hex = format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b());
            let resp = Input::new(&mut hex).id_source(hex_id).show(ui);
            if resp.changed() {
                if let Ok(parsed) = Color32::from_hex(&hex) {
                    *color = parsed;
                }
            }

            // Click the swatch → full HSV/RGB/hex picker.
            egui::Popup::menu(&swatch).show(|ui| {
                ui.set_max_width(core::CONTROL_LG * 6.0);
                color_picker_color32(ui, color, alpha_mode);
            });
            resp
        })
        .inner
    }
}
