//! Field molecule family — labeled form controls.
//!
//! [`Field`] wraps a control with a label and hint/error, in three orientations
//! (vertical/horizontal/responsive — shadcn). [`FieldGroup`] stacks fields; [`FieldSet`] +
//! legend groups them semantically; [`FieldSeparator`] divides groups. All compose atoms.

use crate::atoms::{Divider, Surface, SurfaceFill, Text};
use crate::tokens::{core, layout};
use crate::Theme;
use egui::{Response, Ui};

/// Field layout. `Responsive` goes horizontal at/above [`layout::FIELD_HORIZONTAL_MIN`]. [shadcn]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FieldOrientation {
    #[default]
    Vertical,
    Horizontal,
    Responsive,
}

/// A labeled form field. `show` runs `control` and lays out label + hint/error around it.
pub struct Field {
    label: String,
    required: bool,
    hint: Option<String>,
    error: Option<String>,
    orientation: FieldOrientation,
}

impl Field {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            required: false,
            hint: None,
            error: None,
            orientation: FieldOrientation::default(),
        }
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }
    pub fn orientation(mut self, orientation: FieldOrientation) -> Self {
        self.orientation = orientation;
        self
    }
    pub fn horizontal(self) -> Self {
        self.orientation(FieldOrientation::Horizontal)
    }
    pub fn responsive(self) -> Self {
        self.orientation(FieldOrientation::Responsive)
    }

    pub fn show(self, ui: &mut Ui, control: impl FnOnce(&mut Ui) -> Response) -> Response {
        let theme = Theme::get(ui);
        let horizontal = match self.orientation {
            FieldOrientation::Vertical => false,
            FieldOrientation::Horizontal => true,
            FieldOrientation::Responsive => ui.available_width() >= layout::FIELD_HORIZONTAL_MIN,
        };
        let label = self.label;
        let required = self.required;
        let hint = self.hint;
        let error = self.error;

        if horizontal {
            ui.horizontal(|ui| {
                label_row(ui, &theme, &label, required);
                ui.add_space(core::SPACE_4);
                ui.vertical(|ui| {
                    let response = control(ui);
                    below(ui, &theme, &hint, &error);
                    response
                })
                .inner
            })
            .inner
        } else {
            ui.vertical(|ui| {
                let labeled = label_row(ui, &theme, &label, required);
                if labeled {
                    ui.add_space(core::SPACE_1);
                }
                let response = control(ui);
                below(ui, &theme, &hint, &error);
                response
            })
            .inner
        }
    }
}

fn label_row(ui: &mut Ui, theme: &Theme, label: &str, required: bool) -> bool {
    if label.is_empty() {
        return false;
    }
    ui.horizontal(|ui| {
        Text::new(label).label().show(ui);
        if required {
            Text::new("*").label().color(theme.destructive).show(ui);
        }
    });
    true
}

fn below(ui: &mut Ui, theme: &Theme, hint: &Option<String>, error: &Option<String>) {
    // Free-form prose: wrap on the available width so a long error/hint grows the
    // field downward instead of running past a narrow panel.
    if let Some(error) = error {
        ui.add_space(core::SPACE_1);
        Text::new(error)
            .caption()
            .color(theme.error)
            .wrap()
            .show(ui);
    } else if let Some(hint) = hint {
        ui.add_space(core::SPACE_1);
        Text::new(hint).caption().muted().wrap().show(ui);
    }
}

/// Stacks multiple [`Field`]s with a standard gap. [shadcn FieldGroup]
pub struct FieldGroup;

impl FieldGroup {
    pub fn new() -> Self {
        Self
    }
    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Response {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = core::SPACE_4;
            content(ui);
        })
        .response
    }
}

impl Default for FieldGroup {
    fn default() -> Self {
        Self::new()
    }
}

/// A semantic group of fields with an optional legend. [shadcn FieldSet + FieldLegend]
pub struct FieldSet {
    legend: Option<String>,
}

impl FieldSet {
    pub fn new() -> Self {
        Self { legend: None }
    }
    pub fn legend(mut self, legend: impl Into<String>) -> Self {
        self.legend = Some(legend.into());
        self
    }
    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Response {
        Surface::new()
            .fill(SurfaceFill::None)
            .pad(core::SPACE_4)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    if let Some(legend) = self.legend {
                        Text::new(legend).label().show(ui);
                        ui.add_space(core::SPACE_2);
                    }
                    content(ui);
                });
            })
            .response
    }
}

impl Default for FieldSet {
    fn default() -> Self {
        Self::new()
    }
}

/// A divider between field groups, with optional inline label. [shadcn FieldSeparator]
///
/// *(v1: rule + centered caption; true inline line–text–line is a later refinement.)*
pub struct FieldSeparator {
    label: Option<String>,
}

impl FieldSeparator {
    pub fn new() -> Self {
        Self { label: None }
    }
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    pub fn show(self, ui: &mut Ui) -> Response {
        if let Some(label) = self.label {
            ui.vertical(|ui| {
                let response = Divider::horizontal().show(ui);
                ui.add_space(core::SPACE_1);
                ui.vertical_centered(|ui| {
                    Text::new(label).caption().muted().show(ui);
                });
                response
            })
            .inner
        } else {
            Divider::horizontal().show(ui)
        }
    }
}

impl Default for FieldSeparator {
    fn default() -> Self {
        Self::new()
    }
}
