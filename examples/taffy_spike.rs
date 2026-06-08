//! Taffy spike — gate de decisão (T02 da spec `taffy-layout-spike`).
//!
//! Duas colunas lado a lado renderizam o MESMO grid de stats (label + campo numérico):
//!
//! - **Esquerda — "AutoLayout (atual)"**: reproduz o padrão do studio
//!   (`ui.new_child(max_rect)` + `set_clip_rect` sobre um rect de largura calculada na mão).
//!   Quando a janela encolhe, label(120) + campo excedem a coluna e o conteúdo é
//!   **guilhotinado** pela clip-rect — exatamente o bug de creature stats / interface inspector.
//!
//! - **Direita — "egui_taffy (Grid)"**: o mesmo conteúdo num CSS Grid resolvido por taffy
//!   (`grid_template_columns: [120px, 1fr]`). taffy mede o conteúdo (intrinsic size) e aloca
//!   rects corretos; ao encolher, a coluna `1fr` cede e nada é cortado.
//!
//! Rode e **encolha a janela**: a esquerda corta, a direita reflowa. Esse é o go/no-go.
//!
//! Run: `cargo run --example taffy_spike`

use std::num::NonZeroUsize;

use eframe::egui::{self, vec2, Align, Layout, UiBuilder};
use egui_taffy::taffy::prelude::{fr, length, percent};
use egui_taffy::{taffy, tui, TuiBuilderLogic};
use ouroboros_ui::atoms::{Heading, NumericField, Text};
use ouroboros_ui::tokens::{core, layout};
use ouroboros_ui::{Mode, Theme};

/// As 11 stats de uma creature (label, valor) — o mesmo formato do grid real do studio.
fn seed_stats() -> Vec<(&'static str, f32)> {
    vec![
        ("STR", 12.0),
        ("AGI", 8.0),
        ("VIT", 15.0),
        ("INT", 5.0),
        ("DEX", 10.0),
        ("LUK", 3.0),
        ("HP máximo", 2400.0),
        ("SP máximo", 180.0),
        ("ATK", 320.0),
        ("DEF", 45.0),
        ("MDEF", 22.0),
    ]
}

/// LADO ESQUERDO — reproduz fielmente o anti-padrão do studio: largura calculada na mão,
/// filho recortado por `set_clip_rect`. Conteúdo que excede a coluna é cortado.
fn left_clipped(ui: &mut egui::Ui, stats: &mut [(&'static str, f32)]) {
    let col = ui.available_rect_before_wrap();
    let mut cui = ui.new_child(UiBuilder::new().max_rect(col));
    cui.set_clip_rect(col); // <- a guilhotina (igual módulo de interface / creature_details)
    for (label, value) in stats.iter_mut() {
        cui.horizontal(|ui| {
            ui.allocate_ui_with_layout(
                vec2(layout::PROPERTY_LABEL_WIDTH, core::CONTROL_MD),
                Layout::left_to_right(Align::Center),
                |ui| {
                    Text::new(*label).muted().show(ui);
                },
            );
            NumericField::new(value).range(0.0, 99999.0).show(ui);
        });
        cui.add_space(core::SPACE_2);
    }
}

/// LADO DIREITO — mesmo conteúdo num grid taffy `[120px | 1fr]`. taffy mede e aloca; ao
/// encolher, a coluna `1fr` cede e o campo continua inteiro (sem clip).
fn taffy_grid(ui: &mut egui::Ui, stats: &mut [(&'static str, f32)]) {
    tui(ui, ui.id().with("stats_grid"))
        .reserve_available_space()
        .style(taffy::Style {
            display: taffy::Display::Grid,
            grid_template_columns: vec![length(layout::PROPERTY_LABEL_WIDTH), fr(1.0)],
            gap: length(core::SPACE_2),
            size: taffy::Size {
                width: percent(1.0),
                height: percent(1.0),
            },
            align_items: Some(taffy::AlignItems::Center),
            justify_items: Some(taffy::AlignItems::Stretch),
            ..Default::default()
        })
        .show(|tui| {
            for (label, value) in stats.iter_mut() {
                // Célula 1: label (sem wrap em coluna — trunca se faltar espaço).
                tui.wrap_mode(egui::TextWrapMode::Truncate).ui(|ui| {
                    Text::new(*label).muted().show(ui);
                });
                // Célula 2: campo numérico, ocupa a coluna 1fr.
                tui.ui(|ui| {
                    NumericField::new(value).range(0.0, 99999.0).show(ui);
                });
            }
        });
}

fn main() -> eframe::Result<()> {
    let mut installed = false;
    let mode = Mode::Dark;
    let mut stats_left = seed_stats();
    let mut stats_right = seed_stats();

    eframe::run_ui_native(
        "taffy spike — encolha a janela e compare",
        eframe::NativeOptions::default(),
        move |ui, _frame| {
            if !installed {
                Theme::install(ui.ctx(), mode);
                // egui_taffy mede + relayout no mesmo frame via multipass.
                ui.ctx()
                    .options_mut(|o| o.max_passes = NonZeroUsize::new(2).unwrap());
                installed = true;
                ui.ctx().request_repaint();
                return;
            }
            let theme = Theme::get(ui);
            ui.painter()
                .rect_filled(ui.clip_rect(), 0.0, theme.background);

            ui.add_space(core::SPACE_2);
            Heading::new("Taffy spike — encolha a janela e compare os dois lados")
                .h2()
                .show(ui);
            ui.add_space(core::SPACE_3);

            ui.columns(2, |cols| {
                Heading::new("AutoLayout (atual) — clipa")
                    .heading()
                    .show(&mut cols[0]);
                cols[0].add_space(core::SPACE_2);
                left_clipped(&mut cols[0], &mut stats_left);

                Heading::new("egui_taffy (Grid) — reflowa")
                    .heading()
                    .show(&mut cols[1]);
                cols[1].add_space(core::SPACE_2);
                taffy_grid(&mut cols[1], &mut stats_right);
            });
        },
    )
}
