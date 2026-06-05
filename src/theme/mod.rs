//! Theme resolution and installation.
//!
//! [`Mode`] selects a palette; [`Theme::resolve`] produces the resolved
//! [`Theme`](crate::tokens::semantic::Theme); [`Theme::install`] wires fonts + the
//! resolved theme into the egui context, and [`Theme::get`]/[`Theme::get_from_ctx`]
//! retrieve it inside widgets. [`typography`] registers fonts and exposes the type styles.

pub mod typography;

use crate::tokens::semantic::Theme;
use egui::{FontDefinitions, Id, TextStyle};

/// Color mode. First-class infrastructure; `Dark` is populated, `Light` is a stub
/// that currently resolves to `Dark` (filled in a later milestone).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    Dark,
    Light,
}

impl Theme {
    /// Resolve the semantic theme for a [`Mode`].
    pub fn resolve(mode: Mode) -> Self {
        match mode {
            Mode::Dark => Self::dark(),
            Mode::Light => Self::light(),
        }
    }

    /// Install fonts + apply the resolved theme. Call once at startup.
    pub fn install(ctx: &egui::Context, mode: Mode) {
        let mut fonts = FontDefinitions::default();
        typography::register(&mut fonts);
        ctx.set_fonts(fonts);
        Self::apply(ctx, mode);
    }

    /// Apply the resolved theme (visuals + stored tokens + text styles) for `mode`,
    /// without re-registering fonts. Use this to switch [`Mode`] at runtime.
    pub fn apply(ctx: &egui::Context, mode: Mode) {
        let theme = Self::resolve(mode);

        // Switch egui's own Dark/Light theme so its built-in chrome (clear color,
        // native widgets, scrollbars) follows the mode too.
        ctx.set_theme(match mode {
            Mode::Dark => egui::ThemePreference::Dark,
            Mode::Light => egui::ThemePreference::Light,
        });

        ctx.global_style_mut(|style| {
            style.visuals.dark_mode = matches!(mode, Mode::Dark);
            style.visuals.panel_fill = theme.background;
            style.visuals.window_fill = theme.card;
            style.visuals.extreme_bg_color = theme.muted;
            style.visuals.faint_bg_color = theme.card;

            style
                .text_styles
                .insert(TextStyle::Heading, typography::h2().font_id());
            style
                .text_styles
                .insert(TextStyle::Body, typography::body().font_id());
            style
                .text_styles
                .insert(TextStyle::Button, typography::body().font_id());
            style
                .text_styles
                .insert(TextStyle::Monospace, typography::code().font_id());
            style
                .text_styles
                .insert(TextStyle::Small, typography::caption().font_id());
        });

        ctx.data_mut(|d| d.insert_temp(Id::NULL, theme));
    }

    /// Retrieve the installed theme from a [`Ui`](egui::Ui) (falls back to [`Default`]).
    pub fn get(ui: &egui::Ui) -> Self {
        ui.data(|d| d.get_temp::<Self>(Id::NULL))
            .unwrap_or_default()
    }

    /// Retrieve the installed theme from a [`Context`](egui::Context) directly.
    pub fn get_from_ctx(ctx: &egui::Context) -> Self {
        ctx.data(|d| d.get_temp::<Self>(Id::NULL))
            .unwrap_or_default()
    }
}
