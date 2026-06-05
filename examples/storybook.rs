//! Storybook — token gallery for ouroboros-ui.
//!
//! Renders the foundation's tokens so decisions can be validated by eye: semantic +
//! core color swatches, the spacing / radius / shadow scales, the composite type scale,
//! and a Phosphor-Light icon row. No components yet — this is the foundation surface.
//!
//! Run: `cargo run --example storybook`

use egui::{
    vec2, Color32, CornerRadius, RichText, Sense, Stroke, StrokeKind, Ui,
};
use egui_phosphor::light;
use ouroboros_ui::auto_layout::{AutoLayout, CrossAlign, MainAlign};
use ouroboros_ui::theme::typography;
use ouroboros_ui::tokens::{core, layout};
use ouroboros_ui::{Mode, Theme};

fn main() -> eframe::Result<()> {
    let mut installed = false;
    eframe::run_ui_native(
        "ouroboros-ui storybook",
        eframe::NativeOptions::default(),
        move |ui, _frame| {
            if !installed {
                // `set_fonts` only takes effect next frame — install, then skip rendering
                // this frame so the named Iosevka families exist before we reference them.
                Theme::install(ui.ctx(), Mode::Dark);
                installed = true;
                ui.ctx().request_repaint();
                return;
            }
            let theme = Theme::get(ui);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_max_width(880.0);
                ui.add_space(core::SPACE_4);
                title(ui, "ouroboros-ui", &theme);
                ui.label(
                    RichText::new("foundation — tokens · Iosevka · Phosphor Light")
                        .font(typography::body().font_id())
                        .color(theme.muted_foreground),
                );
                ui.add_space(core::SPACE_6);

                section(ui, "Semantic — surfaces & text", &theme);
                swatch_row(
                    ui,
                    &[
                        ("background", theme.background),
                        ("card", theme.card),
                        ("popover", theme.popover),
                        ("muted", theme.muted),
                        ("border", theme.border),
                        ("border_strong", theme.border_strong),
                    ],
                    &theme,
                );
                swatch_row(
                    ui,
                    &[
                        ("foreground", theme.foreground),
                        ("muted_foreground", theme.muted_foreground),
                        ("disabled_foreground", theme.disabled_foreground),
                        ("primary", theme.primary),
                        ("ring", theme.ring),
                    ],
                    &theme,
                );

                section(ui, "Core — zinc ramp", &theme);
                swatch_row(
                    ui,
                    &[
                        ("50", core::ZINC_50),
                        ("100", core::ZINC_100),
                        ("200", core::ZINC_200),
                        ("300", core::ZINC_300),
                        ("400", core::ZINC_400),
                        ("500", core::ZINC_500),
                        ("600", core::ZINC_600),
                        ("700", core::ZINC_700),
                        ("800", core::ZINC_800),
                        ("900", core::ZINC_900),
                        ("950", core::ZINC_950),
                    ],
                    &theme,
                );

                section(ui, "Status", &theme);
                swatch_row(
                    ui,
                    &[
                        ("success", theme.success),
                        ("warning", theme.warning),
                        ("error", theme.error),
                        ("info", theme.info),
                        ("neutral", theme.neutral),
                    ],
                    &theme,
                );

                section(ui, "Spacing — base 4px", &theme);
                for (name, w) in [
                    ("SPACE_1", core::SPACE_1),
                    ("SPACE_2", core::SPACE_2),
                    ("SPACE_3", core::SPACE_3),
                    ("SPACE_4", core::SPACE_4),
                    ("SPACE_5", core::SPACE_5),
                    ("SPACE_6", core::SPACE_6),
                    ("SPACE_8", core::SPACE_8),
                    ("SPACE_10", core::SPACE_10),
                    ("SPACE_12", core::SPACE_12),
                ] {
                    ui.horizontal(|ui| {
                        ui.allocate_ui(vec2(80.0, 16.0), |ui| {
                            ui.label(
                                RichText::new(name)
                                    .font(typography::code().font_id())
                                    .color(theme.muted_foreground),
                            );
                        });
                        let (rect, _) = ui.allocate_exact_size(vec2(w, 14.0), Sense::hover());
                        ui.painter()
                            .rect_filled(rect, CornerRadius::same(2), theme.primary);
                        ui.label(
                            RichText::new(format!("{w}"))
                                .font(typography::caption().font_id())
                                .color(theme.muted_foreground),
                        );
                    });
                }

                section(ui, "Radius", &theme);
                ui.horizontal(|ui| {
                    for (name, r) in [
                        ("SM 4", core::RADIUS_SM),
                        ("MD 6", core::RADIUS_MD),
                        ("LG 8", core::RADIUS_LG),
                        ("XL 12", core::RADIUS_XL),
                    ] {
                        ui.vertical(|ui| {
                            let (rect, _) =
                                ui.allocate_exact_size(vec2(56.0, 56.0), Sense::hover());
                            ui.painter()
                                .rect_filled(rect, CornerRadius::same(r as u8), theme.muted);
                            ui.painter().rect_stroke(
                                rect,
                                CornerRadius::same(r as u8),
                                Stroke::new(1.0, theme.border_strong),
                                StrokeKind::Inside,
                            );
                            ui.add_space(core::SPACE_1);
                            ui.label(
                                RichText::new(name)
                                    .font(typography::caption().font_id())
                                    .color(theme.muted_foreground),
                            );
                        });
                        ui.add_space(core::SPACE_4);
                    }
                });

                section(ui, "Shadows", &theme);
                ui.label(
                    RichText::new("shown on a light panel — dark-theme shadows are subtle by design (borders carry elevation)")
                        .font(typography::caption().font_id())
                        .color(theme.muted_foreground),
                );
                ui.add_space(core::SPACE_2);
                // Light backdrop so the (black) shadow geometry is legible.
                egui::Frame::default()
                    .fill(core::ZINC_200)
                    .corner_radius(CornerRadius::same(core::RADIUS_LG as u8))
                    .inner_margin(core::SPACE_6)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            for (name, shadow) in [
                                ("sm", core::SHADOW_SM),
                                ("md", core::SHADOW_MD),
                                ("lg", core::SHADOW_LG),
                            ] {
                                egui::Frame::default()
                                    .fill(core::ZINC_50)
                                    .corner_radius(CornerRadius::same(core::RADIUS_LG as u8))
                                    .shadow(shadow)
                                    .inner_margin(core::SPACE_4)
                                    .show(ui, |ui| {
                                        ui.label(
                                            RichText::new(format!("shadow {name}"))
                                                .font(typography::label().font_id())
                                                .color(core::ZINC_900),
                                        );
                                    });
                                ui.add_space(core::SPACE_8);
                            }
                        });
                    });

                section(ui, "Type scale", &theme);
                for (name, style) in [
                    ("display", typography::display()),
                    ("h1", typography::h1()),
                    ("h2", typography::h2()),
                    ("heading", typography::heading()),
                    ("body", typography::body()),
                    ("body_strong", typography::body_strong()),
                    ("label", typography::label()),
                    ("caption", typography::caption()),
                    ("code", typography::code()),
                    ("kbd", typography::kbd()),
                ] {
                    ui.horizontal(|ui| {
                        ui.allocate_ui(vec2(140.0, style.size), |ui| {
                            ui.label(
                                RichText::new(format!("{name} · {}px", style.size))
                                    .font(typography::code().font_id())
                                    .color(theme.muted_foreground),
                            );
                        });
                        ui.label(
                            RichText::new("Ouroboros — the serpent renews")
                                .font(style.font_id())
                                .color(theme.foreground),
                        );
                    });
                    ui.add_space(core::SPACE_1);
                }

                section(ui, "Icons — Phosphor Light", &theme);
                ui.horizontal_wrapped(|ui| {
                    for g in [
                        light::GRID_FOUR,
                        light::SQUARES_FOUR,
                        light::CUBE,
                        light::TERMINAL,
                        light::GEAR,
                        light::PALETTE,
                        light::MAGNIFYING_GLASS,
                        light::STAR,
                        light::CHECK,
                        light::WARNING,
                        light::INFO,
                        light::LIGHTBULB,
                    ] {
                        ui.label(
                            RichText::new(g).size(22.0).color(theme.foreground),
                        );
                        ui.add_space(core::SPACE_3);
                    }
                });

                section(ui, "Sizing — controls & icons", &theme);
                ui.horizontal(|ui| {
                    for (name, h) in [
                        ("SM 26", core::CONTROL_SM),
                        ("MD 32", core::CONTROL_MD),
                        ("LG 38", core::CONTROL_LG),
                    ] {
                        let (rect, _) = ui.allocate_exact_size(vec2(96.0, h), Sense::hover());
                        ui.painter()
                            .rect_filled(rect, CornerRadius::same(core::RADIUS_MD as u8), theme.muted);
                        ui.painter().rect_stroke(
                            rect,
                            CornerRadius::same(core::RADIUS_MD as u8),
                            Stroke::new(core::BORDER_THIN, theme.border_strong),
                            StrokeKind::Inside,
                        );
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            name,
                            typography::caption().font_id(),
                            theme.muted_foreground,
                        );
                        ui.add_space(core::SPACE_3);
                    }
                });
                ui.add_space(core::SPACE_2);
                ui.horizontal(|ui| {
                    for (g, sz, lbl) in [
                        (light::CUBE, core::ICON_SM, "14"),
                        (light::CUBE, core::ICON_MD, "16"),
                        (light::CUBE, core::ICON_LG, "20"),
                        (light::CUBE, core::ICON_XL, "24"),
                    ] {
                        ui.vertical(|ui| {
                            ui.label(RichText::new(g).size(sz).color(theme.foreground));
                            ui.label(
                                RichText::new(lbl)
                                    .font(typography::caption().font_id())
                                    .color(theme.muted_foreground),
                            );
                        });
                        ui.add_space(core::SPACE_4);
                    }
                });

                section(ui, "Opacity & overlays", &theme);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("enabled")
                            .font(typography::body().font_id())
                            .color(theme.foreground),
                    );
                    ui.add_space(core::SPACE_4);
                    ui.label(
                        RichText::new("disabled 0.5")
                            .font(typography::body().font_id())
                            .color(theme.foreground.gamma_multiply(core::OPACITY_DISABLED)),
                    );
                });
                ui.add_space(core::SPACE_2);
                ui.horizontal(|ui| {
                    // scrim over a light tile
                    overlay_tile(ui, "scrim 0.6", core::ZINC_200, core::SCRIM);
                    ui.add_space(core::SPACE_4);
                    overlay_tile(ui, "hover .06", theme.muted, Color32::from_white_alpha(15));
                    ui.add_space(core::SPACE_4);
                    overlay_tile(ui, "press .12", theme.muted, Color32::from_white_alpha(31));
                });

                section(ui, "Layout — panels & breakpoints", &theme);
                for (name, w) in [
                    ("SIDEBAR 240", layout::SIDEBAR_WIDTH),
                    ("INSPECTOR 300", layout::INSPECTOR_WIDTH),
                    ("PANEL_MIN 180", layout::PANEL_MIN),
                    ("PANEL_MAX 480", layout::PANEL_MAX),
                ] {
                    ui.horizontal(|ui| {
                        let (rect, _) = ui.allocate_exact_size(vec2(w, 14.0), Sense::hover());
                        ui.painter()
                            .rect_filled(rect, CornerRadius::same(2), theme.border_strong);
                        ui.label(
                            RichText::new(name)
                                .font(typography::code().font_id())
                                .color(theme.muted_foreground),
                        );
                    });
                }
                ui.add_space(core::SPACE_2);
                ui.label(
                    RichText::new("breakpoints  ·  compact <720  ·  normal <1024  ·  wide ≥1440")
                        .font(typography::code().font_id())
                        .color(theme.muted_foreground),
                );

                section(ui, "Motion — durations & easing", &theme);
                let t = ui.input(|i| i.time) as f32;
                for (name, dur, easing) in [
                    ("fast 0.10  EaseOut", core::DURATION_FAST, core::Easing::EaseOut),
                    ("normal 0.18  EaseOut", core::DURATION_NORMAL, core::Easing::EaseOut),
                    ("slow 0.30  EaseInOut", core::DURATION_SLOW, core::Easing::EaseInOut),
                ] {
                    // Loop the dot across the track over (2*dur + pause), easing each leg.
                    let period = dur * 2.0 + 0.6;
                    let phase = (t % period) / period;
                    let leg = if phase < 0.5 { phase * 2.0 } else { 1.0 - (phase - 0.5) * 2.0 };
                    let e = easing.apply(leg.clamp(0.0, 1.0));
                    ui.horizontal(|ui| {
                        ui.allocate_ui(vec2(180.0, 16.0), |ui| {
                            ui.label(
                                RichText::new(name)
                                    .font(typography::code().font_id())
                                    .color(theme.muted_foreground),
                            );
                        });
                        let (track, _) = ui.allocate_exact_size(vec2(240.0, 16.0), Sense::hover());
                        ui.painter().rect_filled(track, CornerRadius::same(8), theme.muted);
                        let r = 6.0;
                        let x = track.left() + r + e * (track.width() - 2.0 * r);
                        ui.painter()
                            .circle_filled(egui::pos2(x, track.center().y), r, theme.primary);
                    });
                }
                ui.ctx().request_repaint();

                section(ui, "Auto-layout (Figma flow)", &theme);
                let (p, pf) = (theme.primary, theme.primary_foreground);
                let (mu, fg) = (theme.muted, theme.foreground);

                ui.label(cap("gap Auto — space-between", &theme));
                al_box(ui, &theme, |ui| {
                    AutoLayout::horizontal()
                        .gap_auto()
                        .pad(core::SPACE_2)
                        .cross_align(CrossAlign::Center)
                        .hug(|ui| chip(ui, "A", p, pf))
                        .hug(|ui| chip(ui, "B", mu, fg))
                        .hug(|ui| chip(ui, "C", mu, fg))
                        .show(ui);
                });

                ui.label(cap("Fill spacer — left group · spacer · right action", &theme));
                al_box(ui, &theme, |ui| {
                    AutoLayout::horizontal()
                        .gap(core::SPACE_2)
                        .pad(core::SPACE_2)
                        .cross_align(CrossAlign::Center)
                        .hug(|ui| chip(ui, "File", mu, fg))
                        .hug(|ui| chip(ui, "Edit", mu, fg))
                        .fill(|_ui| {})
                        .hug(|ui| chip(ui, "Run", p, pf))
                        .show(ui);
                });

                ui.label(cap("main_align Center", &theme));
                al_box(ui, &theme, |ui| {
                    AutoLayout::horizontal()
                        .gap(core::SPACE_2)
                        .pad(core::SPACE_2)
                        .main_align(MainAlign::Center)
                        .cross_align(CrossAlign::Center)
                        .hug(|ui| chip(ui, "ok", p, pf))
                        .hug(|ui| chip(ui, "cancel", mu, fg))
                        .show(ui);
                });

                ui.label(cap("cross_align Center — mixed heights", &theme));
                al_box(ui, &theme, |ui| {
                    AutoLayout::horizontal()
                        .gap(core::SPACE_3)
                        .pad(core::SPACE_2)
                        .cross_align(CrossAlign::Center)
                        .hug(|ui| {
                            ui.label(RichText::new(light::CUBE).size(28.0).color(p));
                        })
                        .hug(|ui| chip(ui, "label", mu, fg))
                        .show(ui);
                });

                ui.label(cap("vertical stack — gap + padding", &theme));
                al_box(ui, &theme, |ui| {
                    AutoLayout::vertical()
                        .gap(core::SPACE_2)
                        .pad(core::SPACE_3)
                        .cross_align(CrossAlign::Start)
                        .hug(|ui| chip(ui, "row one", mu, fg))
                        .hug(|ui| chip(ui, "row two", mu, fg))
                        .hug(|ui| chip(ui, "row three", p, pf))
                        .show(ui);
                });

                ui.add_space(core::SPACE_8);
            });
        },
    )
}

fn title(ui: &mut Ui, text: &str, theme: &Theme) {
    ui.label(
        RichText::new(text)
            .font(typography::display().font_id())
            .color(theme.foreground),
    );
}

fn cap(text: &str, theme: &Theme) -> RichText {
    RichText::new(text)
        .font(typography::caption().font_id())
        .color(theme.muted_foreground)
}

/// Bordered, width-bounded box wrapping an auto-layout demo so leftover-space
/// distribution (space-between / fill / center) is visible.
fn al_box(ui: &mut Ui, theme: &Theme, add: impl FnOnce(&mut Ui)) {
    egui::Frame::default()
        .stroke(Stroke::new(1.0, theme.border))
        .corner_radius(CornerRadius::same(core::RADIUS_MD as u8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width().min(420.0));
            add(ui);
        });
    ui.add_space(core::SPACE_4);
}

/// A content-sized (Hug) pill with a centered label — a stand-in child for layout demos.
fn chip(ui: &mut Ui, label: &str, fill: Color32, fg: Color32) {
    let pad = vec2(10.0, 6.0);
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_owned(), typography::label().font_id(), fg);
    let size = galley.size() + pad * 2.0;
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter()
        .rect_filled(rect, CornerRadius::same(core::RADIUS_MD as u8), fill);
    ui.painter().galley(rect.min + pad, galley, fg);
}

fn section(ui: &mut Ui, text: &str, theme: &Theme) {
    ui.add_space(core::SPACE_6);
    ui.label(
        RichText::new(text)
            .font(typography::heading().font_id())
            .color(theme.foreground),
    );
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width().min(880.0), 1.0), Sense::hover());
    ui.painter().rect_filled(rect, CornerRadius::ZERO, theme.border);
    ui.add_space(core::SPACE_3);
}

fn hex(c: Color32) -> String {
    format!("#{:02x}{:02x}{:02x}", c.r(), c.g(), c.b())
}

/// A tile of `base` with `overlay` veiled on top, captioned — for opacity/scrim demos.
fn overlay_tile(ui: &mut Ui, label: &str, base: Color32, overlay: Color32) {
    ui.vertical(|ui| {
        let (rect, _) = ui.allocate_exact_size(vec2(104.0, 44.0), Sense::hover());
        let cr = CornerRadius::same(core::RADIUS_MD as u8);
        ui.painter().rect_filled(rect, cr, base);
        ui.painter().rect_filled(rect, cr, overlay);
        ui.add_space(core::SPACE_1);
        ui.label(
            RichText::new(label)
                .font(typography::caption().font_id())
                .color(core::ZINC_400),
        );
    });
}

fn swatch_row(ui: &mut Ui, items: &[(&str, Color32)], theme: &Theme) {
    ui.horizontal_wrapped(|ui| {
        for (name, color) in items {
            ui.vertical(|ui| {
                let (rect, _) = ui.allocate_exact_size(vec2(104.0, 44.0), Sense::hover());
                ui.painter()
                    .rect_filled(rect, CornerRadius::same(core::RADIUS_MD as u8), *color);
                ui.painter().rect_stroke(
                    rect,
                    CornerRadius::same(core::RADIUS_MD as u8),
                    Stroke::new(1.0, Color32::from_white_alpha(18)),
                    StrokeKind::Inside,
                );
                ui.add_space(core::SPACE_1);
                ui.label(
                    RichText::new(*name)
                        .font(typography::caption().font_id())
                        .color(theme.foreground),
                );
                ui.label(
                    RichText::new(hex(*color))
                        .font(typography::code().font_id())
                        .color(theme.muted_foreground),
                );
            });
            ui.add_space(core::SPACE_3);
        }
    });
}
